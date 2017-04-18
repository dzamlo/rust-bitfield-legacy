use syntax::ast;
use syntax::ast::Attribute;
use syntax::codemap::DUMMY_SP;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::symbol::keywords;
use syntax::ptr::P;
use std;

use misc::make_maybe_pub;
use misc::set_attrs_method;

pub enum FieldSize {
    Array { count: usize, element_length: u8 },
    Scalar { length: u8 },
}

pub struct Field {
    name: String,
    is_pub: bool,
    attrs: Vec<Attribute>,
    size: FieldSize,
}

impl Field {
    pub fn bit_len(&self) -> u64 {
        match self.size {
            FieldSize::Array {
                count,
                element_length,
            } => (count as u64) * element_length as u64,
            FieldSize::Scalar { length } => length as u64,
        }
    }

    fn gen_single_value_get_expr(cx: &mut ExtCtxt,
                                 value_type: &P<ast::Ty>,
                                 start: u64,
                                 length: u8)
                                 -> P<ast::Expr> {
        if length == 1 {
            let byte_shift = (7 - start % 8) as usize;
            quote_expr!(cx, (self.data[($start/8) as usize] & (0x1 << $byte_shift)) != 0)
        } else {
            let mut value_expr = None;
            let mut bits_to_get = length;
            let mut bit_offset = start;
            while bits_to_get > 0 {
                let can_get = std::cmp::min(bits_to_get, 8 - (bit_offset % 8) as u8);
                let index = (bit_offset / 8) as usize;
                let mask = 0xFFu8 >> (8 - can_get) as usize;
                let byte_shift = (8 - can_get - (bit_offset % 8) as u8) as usize;
                let bits_expr =
                    quote_expr!(cx, ((self.data[$index] >> $byte_shift) & $mask) as $value_type);

                value_expr = match value_expr {
                    Some(expr) => {
                        let shifted =
                            cx.expr_binary(DUMMY_SP,
                                           ast::BinOpKind::Shl,
                                           expr,
                                           cx.expr_usize(DUMMY_SP, can_get as usize));
                        Some(cx.expr_binary(DUMMY_SP, ast::BinOpKind::BitOr, shifted, bits_expr))
                    }
                    None => Some(bits_expr),
                };
                bits_to_get -= can_get;
                bit_offset += can_get as u64;
            }
            value_expr.unwrap()
        }
    }

    fn gen_single_value_set_stmt(cx: &mut ExtCtxt,
                                 type_length: u8,
                                 start: u64,
                                 length: u8)
                                 -> P<ast::Stmt> {
        if length == 1 {
            let mask = 0x1u8 << (7 - start % 8) as usize;
            let index = (start / 8) as usize;
            P(quote_stmt!(cx,
                          if value {self.data[$index] |= $mask
                          } else {self.data[$index] &= !($mask)
                          })
                      .unwrap())
        } else {
            let mut stmts = Vec::new();
            let mut bits_to_set = length;
            let mut bit_offset = start;

            let mut value_shift = (start % 8) as isize - (type_length - length) as isize +
                                  8 * (type_length / 8 - 1) as isize;

            while bits_to_set > 0 {
                let can_set = std::cmp::min(bits_to_set, 8 - (bit_offset % 8) as u8);
                let index = (bit_offset / 8) as usize;

                // only bits set to 1 in the mask are modified
                let mask = (0xFFu8 >> (8 - can_set as usize)) <<
                           (8 - can_set - (bit_offset % 8) as u8) as usize;

                // positive value of value_shift means we want to shift to the right and
                // negative value means we want shift to the left
                if value_shift > 0 {
                    let value_shift = value_shift as usize;
                    stmts.push(quote_stmt!(cx, self.data[$index] =
                        (self.data[$index] & !$mask) |
                        ((value >> $value_shift) as u8)& $mask)
                                       .unwrap());
                } else {
                    let value_shift = (-value_shift) as usize;
                    stmts.push(quote_stmt!(cx, self.data[$index] =
                        (self.data[$index] & !$mask) |
                        ((value << $value_shift) as u8)& $mask)
                                       .unwrap());
                }

                bits_to_set -= can_set;
                bit_offset += can_set as u64;
                value_shift -= 8;
            }
            let block = cx.block(DUMMY_SP, stmts);
            let expr = cx.expr(DUMMY_SP, ast::ExprKind::Block(block));
            P(cx.stmt_expr(expr))
        }
    }

    pub fn to_methods(&self,
                      cx: &mut ExtCtxt,
                      struct_ident: ast::Ident,
                      start: u64)
                      -> Vec<P<ast::Item>> {
        let mut methods = vec![];
        let maybe_pub = make_maybe_pub(self.is_pub);
        let getter_name = "get_".to_owned() + &self.name;
        let getter_ident = ast::Ident::from_str(&getter_name);
        let setter_name = "set_".to_owned() + &self.name;
        let setter_ident = ast::Ident::from_str(&setter_name);

        let (getter_expr, setter_stmt, value_type) = match self.size {
            FieldSize::Array {
                count,
                element_length,
            } => {
                let (element_type, value_type_length) = size_to_ty(cx, element_length).unwrap();
                let value_type = make_array_ty(cx, &element_type, count);

                let mut element_getter_exprs = Vec::with_capacity(count);
                for i in 0..count {
                    let element_start = start + (i as u64) * (element_length as u64);
                    element_getter_exprs.push(Field::gen_single_value_get_expr(cx,
                                                                               &element_type,
                                                                               element_start,
                                                                               element_length));
                }

                let getter_expr = cx.expr_vec(DUMMY_SP, element_getter_exprs);

                let mut element_setter_stmts = Vec::new();
                for i in 0..count {
                    let element_start = start + (i as u64) * (element_length as u64);
                    let stmt = Field::gen_single_value_set_stmt(cx,
                                                                value_type_length,
                                                                element_start,
                                                                element_length);
                    element_setter_stmts.push(quote_stmt!(cx, { let value = value[$i]; $stmt })
                                                  .unwrap());
                }



                let block = cx.block(DUMMY_SP, element_setter_stmts);
                let expr = cx.expr(DUMMY_SP, ast::ExprKind::Block(block));
                let setter_stmt = cx.stmt_expr(expr);
                (getter_expr, setter_stmt, value_type)

            }
            FieldSize::Scalar { length } => {
                let (value_type, value_type_length) = size_to_ty(cx, length).unwrap();
                let getter_expr = Field::gen_single_value_get_expr(cx, &value_type, start, length);
                let setter_stmt =
                    Field::gen_single_value_set_stmt(cx, value_type_length, start, length);
                (getter_expr, setter_stmt.unwrap(), value_type)
            }
        };

        let getter = quote_item!(cx,
           impl $struct_ident {
               #[inline]
               $maybe_pub fn $getter_ident(&self) -> $value_type {
                  $getter_expr
               }
           })
                .unwrap();
        let getter = set_attrs_method(getter, &self.attrs);
        methods.push(getter);

        let setter = quote_item!(cx,
           impl $struct_ident {
               #[inline]
               $maybe_pub fn $setter_ident(&mut self, value: $value_type) {
                  $setter_stmt
               }
           })
                .unwrap();
        let setter = set_attrs_method(setter, &self.attrs);
        methods.push(setter);

        methods
    }
}


macro_rules! expect_token {
    ($parser:expr, $token:expr) => {
        if let Err(mut e) = $parser.expect($token) {
            e.emit();
            $parser.bump();
        }
    }
}

fn parse_u128(parser: &mut Parser) -> u128 {
    let lit = parser.parse_lit();
    match lit {
        Ok(lit) => {
            match lit.node {
                ast::LitKind::Int(n, _) => n,
                _ => {
                    parser.span_err(lit.span, "unsigned integer literal expected");
                    1
                }
            }
        }
        Err(mut db) => {
            db.emit();
            1
        }
    }
}

pub fn parse_field(parser: &mut Parser) -> Field {
    let attrs = parser.parse_outer_attributes().unwrap();
    let is_pub = parser.eat_keyword(keywords::Pub);
    let ident = match parser.parse_ident() {
        Ok(ident) => ident,
        Err(mut e) => {
            e.emit();
            parser.abort_if_errors();
            unreachable!();
        }
    };
    let name = ident.name.to_string();
    expect_token!(parser, &token::Colon);

    let size = if parser.eat(&token::OpenDelim(token::Bracket)) {
        // Field::Array
        let mut element_length = parse_u128(parser);
        if element_length == 0 || element_length > 128 {
            let span = parser.prev_span;
            parser.span_err(span, "Elements length must be > 0 and <= 128");
            // We set element_length to a dummy value, so we can continue parsing
            element_length = 1;
        }

        expect_token!(parser, &token::Semi);

        let mut count = parse_u128(parser);
        if count == 0 {
            let span = parser.prev_span;
            parser.span_err(span, "Elements count must be > 0");
            count = 1
        }

        expect_token!(parser, &token::CloseDelim(token::Bracket));

        FieldSize::Array {
            element_length: element_length as u8,
            count: count as usize,
        }
    } else {
        // Field::Scalar
        let mut length = parse_u128(parser);
        if length == 0 || length > 128 {
            let span = parser.prev_span;
            parser.span_err(span, "Field length must be > 0 and <= 128");
            length = 1;
        }
        FieldSize::Scalar { length: length as u8 }
    };

    Field {
        name: name,
        is_pub: is_pub,
        attrs: attrs,
        size: size,
    }
}


/// Return the smaller bool or unsigned int type than can hold an amount of bits. Also return the size
/// of the type in bits.
/// The size for the bool type is not releavant because it is never used.
fn size_to_ty(cx: &mut ExtCtxt, size: u8) -> Option<(P<ast::Ty>, u8)> {
    match size {
        i if i == 0 => None,
        i if i == 1 => Some((quote_ty!(cx, bool), 1)),
        i if i <= 8 => Some((quote_ty!(cx, u8), 8)),
        i if i <= 16 => Some((quote_ty!(cx, u16), 16)),
        i if i <= 32 => Some((quote_ty!(cx, u32), 32)),
        i if i <= 64 => Some((quote_ty!(cx, u64), 64)),
        i if i <= 128 => Some((quote_ty!(cx, u128), 128)),
        _ => None,
    }
}

fn make_array_ty(cx: &mut ExtCtxt, elements_type: &P<ast::Ty>, length: usize) -> P<ast::Ty> {
    quote_ty!(cx, [$elements_type; $length])
}
