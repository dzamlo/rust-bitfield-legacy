#![feature(plugin_registrar)]

extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bitfield", expand_bitfield);
}

fn expand_bitfield(cx: &mut ExtCtxt, sp: Span, args: &[ast::TokenTree])
        -> Box<MacResult + 'static> {
    return DummyResult::any(sp);
}
