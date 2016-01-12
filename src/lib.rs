#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::ToTokens;
use syntax::parse::common::SeqSep;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;


enum Field {
    ArrayField {
        name: String,
        count: usize,
        element_length: u8,
    },
    ScalarField {
        name: String,
        length: u8,
    },
}


impl Field {
    fn bit_len(&self) -> u64 {
        match *self {
            Field::ArrayField{name: _, count, element_length} => {
                (count as u64) * element_length as u64
            }
            Field::ScalarField{name: _, length} => length as u64,
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
                        // ExprParen should be a no-ops but it seem that the to_tokens used by quote_method
                        // "flatten" the expression. ExprParen prevent this.
                        let expr = cx.expr(DUMMY_SP, ast::ExprParen(expr));
                        let shifted = cx.expr_binary(DUMMY_SP,
                                                     ast::BiShl,
                                                     expr,
                                                     cx.expr_usize(DUMMY_SP, can_get as usize));
                        Some(cx.expr_binary(DUMMY_SP, ast::BiBitOr, shifted, bits_expr))
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
            quote_stmt!(cx, if value {self.data[$index] |= $mask}
                            else {self.data[$index] &= !($mask)})
                .unwrap()
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
                let mask = (0xFFu8 >> 8 - can_set as usize) <<
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
            let block = cx.block(DUMMY_SP, stmts, None);
            let expr = cx.expr(DUMMY_SP, ast::ExprBlock(block));
            cx.stmt_expr(expr)
        }
    }

    fn to_methods(&self,
                  cx: &mut ExtCtxt,
                  struct_ident: ast::Ident,
                  start: u64)
                  -> Vec<P<ast::Item>> {
        let mut methods = vec![];

        match *self {
            Field::ArrayField{ref name, count, element_length} => {

                let (element_type, value_type_length) = size_to_ty(cx, element_length).unwrap();
                let value_type = make_array_ty(cx, &element_type, count);
                let getter_name = "get_".to_string() + &name[..];
                let getter_ident = token::str_to_ident(&getter_name[..]);

                let mut element_getter_exprs = Vec::with_capacity(count);
                for i in 0..count {
                    let element_start = start + (i as u64) * (element_length as u64);
                    element_getter_exprs.push(Field::gen_single_value_get_expr(cx,
                                                                               &element_type,
                                                                               element_start,
                                                                               element_length));
                }

                let getter_expr = cx.expr_vec(DUMMY_SP, element_getter_exprs);

                let getter = quote_item!(cx,
                   impl $struct_ident {
                       #[inline]
                       fn $getter_ident(&self) -> $value_type {
                          $getter_expr
                       }
                   }
               )
                                 .unwrap();
                methods.push(getter);

                let setter_name = "set_".to_string() + &name[..];
                let setter_ident = token::str_to_ident(&setter_name[..]);
                let mut element_setter_stmts = Vec::new();
                for i in 0..count {
                    let element_start = start + (i as u64) * (element_length as u64);
                    let stmt = Field::gen_single_value_set_stmt(cx,
                                                                value_type_length,
                                                                element_start,
                                                                element_length);
                    element_setter_stmts.push(quote_stmt!(cx, { let value = value[$i]; $stmt})
                                                  .unwrap());
                }



                let block = cx.block(DUMMY_SP, element_setter_stmts, None);
                let expr = cx.expr(DUMMY_SP, ast::ExprBlock(block));
                let setter_stmt = cx.stmt_expr(expr);

                let setter = quote_item!(cx,
                   impl $struct_ident {
                       #[inline]
                       fn $setter_ident(&mut self, value: $value_type) {
                          $setter_stmt
                       }
                   }
               )
                                 .unwrap();
                methods.push(setter);

            }
            Field::ScalarField{ref name, length} => {

                let (value_type, value_type_length) = size_to_ty(cx, length).unwrap();
                let getter_name = "get_".to_string() + &name[..];
                let getter_ident = token::str_to_ident(&getter_name[..]);
                let getter_expr = Field::gen_single_value_get_expr(cx, &value_type, start, length);
                let getter = quote_item!(cx,
                   impl $struct_ident {
                       #[inline]
                       fn $getter_ident(&self) -> $value_type {
                          $getter_expr
                       }
                   }
               )
                                 .unwrap();
                methods.push(getter);

                let setter_name = "set_".to_string() + &name[..];
                let setter_ident = token::str_to_ident(&setter_name[..]);
                let setter_stmt = Field::gen_single_value_set_stmt(cx,
                                                                   value_type_length,
                                                                   start,
                                                                   length);
                let setter = quote_item!(cx,
                   impl $struct_ident {
                       #[inline]
                       fn $setter_ident(&mut self, value: $value_type) {
                          $setter_stmt
                       }
                   }
               )
                                 .unwrap();
                methods.push(setter);

            }
        }

        methods
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
        _ => None,
    }
}

fn make_array_ty(cx: &mut ExtCtxt, elements_type: &P<ast::Ty>, length: usize) -> P<ast::Ty> {
    quote_ty!(cx, [$elements_type; $length])
}

fn parse_u64(parser: &mut Parser) -> u64 {
    let lit = parser.parse_lit();
    match lit {
        Ok(lit) => {
            match lit.node {
                ast::LitInt(n, _) => n,
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

fn parse_field(parser: &mut Parser) -> Field {
    let ident = parser.parse_ident().unwrap();
    let name = ident.name.to_string();
    parser.expect(&token::Colon);
    if parser.eat(&token::OpenDelim(token::Bracket)).unwrap() {
        // ArrayField
        let mut element_length = parse_u64(parser);
        if element_length == 0 || element_length > 64 {
            let span = parser.last_span;
            parser.span_err(span, "Elements length must be > 0 and <= 64");
            // We set element_length to a dummy value, so we can continue parsing
            element_length = 1;
        }
        parser.expect(&token::Semi);
        let mut count = parse_u64(parser);
        if count == 0 {
            let span = parser.last_span;
            parser.span_err(span, "Elements count must be > 0");
            count = 1
        }
        parser.expect(&token::CloseDelim(token::Bracket));
        Field::ArrayField {
            name: name,
            element_length: element_length as u8,
            count: count as usize,
        }
    } else {
        // ScalarField
        let mut length = parse_u64(parser);
        if length == 0 || length > 64 {
            let span = parser.last_span;
            parser.span_err(span, "Field length must be > 0 and <= 64");
            length = 1;
        }
        Field::ScalarField {
            name: name,
            length: length as u8,
        }
    }
}

fn expand_bitfield(cx: &mut ExtCtxt,
                   _sp: Span,
                   tts: &[ast::TokenTree])
                   -> Box<MacResult + 'static> {

    let mut parser = cx.new_parser_from_tts(tts);
    let struct_ident = parser.parse_ident().unwrap();

    parser.expect(&token::Comma);

    let sep = SeqSep {
        sep: Some(token::Comma),
        trailing_sep_allowed: true,
    };

    let fields = parser.parse_seq_to_end(&token::Eof, sep, |p| Ok(parse_field(p))).unwrap();

    let mut items = Vec::with_capacity(fields.len() * 2 + 2);
    let bit_length = fields.iter().fold(0, |a, b| a + b.bit_len());
    let byte_length = ((bit_length + 7) / 8) as usize;
    let struct_decl = quote_item!(cx, struct $struct_ident { data: [u8; $byte_length]};).unwrap();
    items.push(struct_decl);

    let method_new = quote_item!(cx,
       impl $struct_ident {
           fn new(data: [u8; $byte_length]) -> $struct_ident {
               $struct_ident { data: data}
           }
        }
    )
                         .unwrap();
    items.push(method_new);

    let mut field_start = 0;
    for field in fields.iter() {
        items.extend(field.to_methods(cx, struct_ident, field_start));
        field_start += field.bit_len();
    }

    let s = SmallVector::many(items);
    return MacEager::items(s);
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bitfield", expand_bitfield);
}
