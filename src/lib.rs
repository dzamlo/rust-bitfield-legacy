#![feature(plugin_registrar, quote)]
#![feature(struct_variant)]

extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ext::base::{ExtCtxt, MacItems, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::ToTokens;
use syntax::parse::common::SeqSep;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;

#[deriving(Show)]
enum Field {
   ArrayField {name: String, count: uint, element_length: u8},
   ScalarField {name: String, length: u8},   
}


impl Field {
    fn bit_len(&self) -> u64 {
       match *self {
          ArrayField(_, count, element_length) => (count as u64) * element_length as u64,
          ScalarField(_, length) => length as u64,
       }
    }
    
    fn gen_single_value_expr(cx: &mut ExtCtxt, start: u64, value_type: &P<ast::Ty>, length: u8) -> P<ast::Expr> {
        if length == 1 {
            quote_expr!(cx, (self.data[($start/8) as uint] & (0x1 << (7-$start%8) as uint)) != 0)
        } else {
            //TODO: check if optimised correctly
            quote_expr!(cx,
                {
                    let mut result: $value_type = 0;
                    let mut start = $start;
                    let mut bits_to_get = $length;
                    while bits_to_get > 0 {
                        let can_get = std::cmp::min(bits_to_get, 8-(start%8) as u8);
                        result <<= can_get as uint;
                        let byte = self.data[(start/8) as uint];
                        let mask = 0xFF >> (8-can_get) as uint;
                        result |= ((byte >> (8-can_get-(start%8) as u8) as uint) & mask) as $value_type;
                        bits_to_get -= can_get;
                        start += can_get as u64;
                    }
                    result
                }
            )
        }
    }
    
   fn to_methods(&self, cx: &mut ExtCtxt, start: u64) -> Vec<P<ast::Method>> {
       let mut methods = vec![];
       match *self {
           ArrayField(ref name, count, element_length) => {
               let element_type = size_to_ty(cx, element_length).unwrap();
               let value_type = make_array_ty(cx, &element_type, count);
               let getter_name = "get_".to_string() + *name;
               let getter_ident = token::str_to_ident(getter_name.as_slice());
               
               let mut element_getter_exprs = Vec::with_capacity(count);
               for i in range(0, count) {
                    let element_start = start+(i as u64)*(element_length as u64);
                    element_getter_exprs.push(
                        Field::gen_single_value_expr(cx, element_start, &element_type, element_length)
                    );
               }
               
               let getter_expr = cx.expr_vec(DUMMY_SP, element_getter_exprs);
               
               let getter = quote_method!(cx,
                   #[inline]
                   fn $getter_ident(&self) -> $value_type {
                      $getter_expr
                   }
               );
               methods.push(getter);
           },
           ScalarField(ref name, length) => {
               let value_type = size_to_ty(cx, length).unwrap();
               let getter_name = "get_".to_string() + *name;
               let getter_ident = token::str_to_ident(getter_name.as_slice());
               let getter_expr = Field::gen_single_value_expr(cx, start, &value_type, length);
               let getter = quote_method!(cx,
                   #[inline]
                   fn $getter_ident(&self) -> $value_type {
                      $getter_expr
                   }
               );
               methods.push(getter);
               
           },
       }
       
       methods
    }
}

/// Return the smaller bool or uint type than can hold an amount of bits.
fn size_to_ty(cx: &mut ExtCtxt, size: u8) -> Option<P<ast::Ty>> {
       match size {
          i if i == 0 => None,
          i if i == 1 => Some(quote_ty!(cx, bool)),
          i if i <= 8 => Some(quote_ty!(cx, u8)),
          i if i <= 16 => Some(quote_ty!(cx, u16)),
          i if i <= 32 => Some(quote_ty!(cx, u32)),
          i if i <= 64 => Some(quote_ty!(cx, u64)),          
          _ => None
       }
}

fn make_array_ty(cx: &mut ExtCtxt, elements_type: &P<ast::Ty>, length: uint) -> P<ast::Ty> {
    quote_ty!(cx, [$elements_type, ..$length])
}


fn parse_u64(parser: &mut Parser) -> u64 {
      let lit = parser.parse_lit();
      match lit.node {
          ast::LitInt(n, _) => n,
          _ => parser.span_fatal(lit.span, "unsigned integer literal expected")
      }
}

fn parse_field(parser: &mut Parser) -> Field {
    let ident = parser.parse_ident();
    let name = token::get_ident(ident).to_string();
    parser.expect(&token::Colon);
    if parser.eat(&token::OpenDelim(token::Bracket)) {
       // ArrayField
       let  element_length = parse_u64(parser);
       if element_length == 0 || element_length > 64 {
           let span = parser.last_span;
           parser.span_fatal(span, "Field length must be > 0 and <= 64");
       }
       parser.expect(&token::Comma);
       parser.expect(&token::DotDot);
       let count = parse_u64(parser);
       if count == 0 {
          let span = parser.last_span;
          parser.span_fatal(span, "Field length must be > 0");
       }
       parser.expect(&token::CloseDelim(token::Bracket));
       ArrayField {name: name,  element_length:  element_length as u8, count: count as uint}
    }
    else {
      //ScalarField
      let length = parse_u64(parser);
      if length == 0  || length > 64{
          let span = parser.last_span;
          parser.span_fatal(span, "Field length must be > 0 and <= 64");
      }
      ScalarField {name: name, length: length as u8}
    }
}

fn expand_bitfield(cx: &mut ExtCtxt, _sp: Span, tts: &[ast::TokenTree])
        -> Box<MacResult + 'static> {
    let mut parser = cx.new_parser_from_tts(tts);
    let struct_ident = parser.parse_ident();
    parser.expect(&token::Comma);
    
    let sep = SeqSep {
        sep: Some(token::Comma),
        trailing_sep_allowed: true,
    };
    
    let fields = parser.parse_seq_to_end(&token::Eof, sep, |p| parse_field(p));

    let bit_length = fields.iter().fold(0, |a, b| a + b.bit_len());
    let byte_length = ((bit_length+7)/8) as uint;
    
    let struct_decl = quote_item!(cx, struct $struct_ident<'a> { data: &'a [u8, ..$byte_length]};).unwrap();
     
    let mut methods = Vec::with_capacity(fields.len()*2+1);
    
    methods.push(ast::MethodImplItem(quote_method!(cx, 
        fn new(data: &'a [u8, ..$byte_length]) -> $struct_ident { 
            $struct_ident { data: data}
        })));

    let mut field_start = 0;
    for field in fields.iter() {
        methods.extend(field.to_methods(cx, field_start).into_iter().map(|m| ast::MethodImplItem(m)));
        field_start += field.bit_len();
    }
    
    let struct_impl_tpl = quote_item!(cx, impl<'a> $struct_ident<'a> { }).unwrap();
    
    let node = struct_impl_tpl.node.clone();
    //Put the methods we generated inside the impl block.
    let node = match node {
        ast::ItemImpl(a, b, c, _) => ast::ItemImpl(a, b, c, methods),
        _ => unreachable!()
    };
    
    let struct_impl = cx.item(DUMMY_SP, struct_ident, vec![], node);
    
    return MacItems::new(vec![struct_decl, struct_impl].into_iter());
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bitfield", expand_bitfield);
}
