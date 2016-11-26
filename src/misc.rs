use syntax::ast;
use syntax::ptr::P;


pub fn make_maybe_ident(maybe: bool, ident: &str) -> Option<ast::Ident> {
    if maybe {
        Some(ast::Ident::from_str(ident))
    } else {
        None
    }
}

pub fn make_maybe_pub(is_pub: bool) -> Option<ast::Ident> {
    make_maybe_ident(is_pub, "pub")
}

pub fn set_attrs_method(method: P<ast::Item>, attrs: &[ast::Attribute]) -> P<ast::Item> {
    method.map(|mut method| {
        if let ast::ItemKind::Impl(_, _, _, _, _, ref mut impl_items) = method.node {
            impl_items[0].attrs = attrs.to_vec();
        } else {
            unreachable!();
        }
        method
    })
}

pub fn get_impl_items(item: P<ast::Item>) -> Vec<ast::ImplItem> {
    item.and_then(|item| {
        if let ast::ItemKind::Impl(_, _, _, _, _, impl_items) = item.node {
            return impl_items;
        } else {
            unreachable!();
        }
    })
}

pub fn merge_impls(methods: Vec<P<ast::Item>>) -> P<ast::Item> {
    let mut methods = methods;
    let impl_block = methods.pop().unwrap();
    let mut methods_impl_items = vec![];
    for method in methods {
        methods_impl_items.extend(get_impl_items(method));
    }
    impl_block.map(|mut item| {
        if let ast::ItemKind::Impl(_, _, _, _, _, ref mut impl_items) = item.node {
            impl_items.extend(methods_impl_items);
        } else {
            unreachable!();
        }
        item
    })
}
