#![feature(plugin_registrar, quote)]

extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacItems, MacResult, DummyResult};


fn expand_bitfield(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
        -> Box<MacResult + 'static> {
    let mut parser = cx.new_parser_from_tts(tts);
    let struct_ident = parser.parse_ident();
    
    //TODO: parse the fields description.
    
    let bit_length = 0u; //TODO: sum the bit length of every fields.
    let byte_length = (bit_length+7)/8;
    
    let struct_decl = quote_item!(cx, struct $struct_ident<'a> { data: &'a [u8, ..$byte_length]};).unwrap();

    //TODO: create the impl item with a constructor and methods for each fields.

    return MacItems::new(vec![struct_decl].into_iter());
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bitfield", expand_bitfield);
}
