#![feature(plugin_registrar, quote)]
#![feature(struct_variant)]

extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ext::base::{ExtCtxt, MacItems, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::common::SeqSep;
use syntax::parse::parser::Parser;
use syntax::parse::token;

#[deriving(Show)]
enum Field {
   ArrayField {name: String, count: uint, element_length: u64},
   ScalarField {name: String, length: u64},   
}

impl Field {
    fn bit_len(&self) -> u64 {
       match *self {
          ArrayField(_, count, element_length) => (count as u64) * element_length,
          ScalarField(_, length) => length,
       }
    }
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
       ArrayField {name: name,  element_length:  element_length, count: count as uint}
    }
    else {
      //ScalarField
      let length = parse_u64(parser);
      if length == 0  || length > 64{
          let span = parser.last_span;
          parser.span_fatal(span, "Field length must be > 0 and <= 64");
      }
      ScalarField {name: name, length: length}
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

    //TODO: convert fields to getter/setter methods
    
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
