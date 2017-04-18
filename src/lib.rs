#![feature(i128_type, plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

mod field;
mod misc;

use rustc_plugin::Registry;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::parse::common::SeqSep;
use syntax::parse::token;
use syntax::symbol::keywords;
use syntax::tokenstream;
use syntax::util::small_vector::SmallVector;

use field::parse_field;
use misc::*;

fn expand_bitfield(cx: &mut ExtCtxt,
                   _sp: Span,
                   tts: &[tokenstream::TokenTree])
                   -> Box<MacResult + 'static> {

    let mut parser = cx.new_parser_from_tts(tts);
    let attrs = parser.parse_outer_attributes().unwrap();
    let is_pub = parser.eat_keyword(keywords::Pub);
    let struct_ident = match parser.parse_ident() {
        Ok(ident) => ident,
        Err(mut e) => {
            e.emit();
            parser.abort_if_errors();
            unreachable!();
        }
    };

    if let Err(mut e) = parser.expect(&token::Comma) {
        e.emit();
    }

    let sep = SeqSep {
        sep: Some(token::Comma),
        trailing_sep_allowed: true,
    };

    let fields = match parser.parse_seq_to_end(&token::Eof, sep, |p| Ok(parse_field(p))) {
        Ok(fields) => fields,
        Err(mut e) => {
            e.emit();
            vec![]
        }
    };

    let mut const_new = false;
    let mut pub_data = false;
    let attrs = attrs.into_iter()
        .filter(|a| {
            if a.path == "const_new" {
                const_new = true;
                false
            } else if a.path == "pub_data" {
                pub_data = true;
                false
            } else {
                true
            }
        })
        .collect();
    let mut methods = Vec::with_capacity(fields.len() * 2 + 1);
    let bit_length = fields.iter().fold(0, |a, b| a + b.bit_len());
    let byte_length = ((bit_length + 7) / 8) as usize;
    let maybe_pub = make_maybe_pub(is_pub);
    let maybe_pub_data = make_maybe_pub(pub_data);
    let maybe_const_new = make_maybe_ident(const_new, "const");
    let struct_decl = if byte_length > 0 {
        let new_doc = format!("Creates a new `{}`", struct_ident);
        let method_new = quote_item!(cx,
            impl $struct_ident {
                #[doc = $new_doc]
                $maybe_pub $maybe_const_new fn new(data: [u8; $byte_length]) -> $struct_ident {
                    $struct_ident { data: data}
                }
            })
            .unwrap();
        methods.push(method_new);
        quote_item!(cx, $maybe_pub struct $struct_ident { $maybe_pub_data data: [u8; $byte_length]};).unwrap()
    } else {
        quote_item!(cx, $maybe_pub struct $struct_ident { };).unwrap()
    };
    let struct_decl = struct_decl.map(|mut s| {
        s.attrs = attrs;
        s
    });

    let mut field_start = 0;
    for field in fields {
        methods.extend(field.to_methods(cx, struct_ident, field_start));
        field_start += field.bit_len();
    }

    let items = if !methods.is_empty() {
        vec![struct_decl, merge_impls(methods)]
    } else {
        vec![struct_decl]
    };
    let s = SmallVector::many(items);
    MacEager::items(s)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bitfield", expand_bitfield);
}
