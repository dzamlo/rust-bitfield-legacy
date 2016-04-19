#![feature(plugin)]
#![plugin(bitfield)]

mod foo {
    bitfield!{pub Bar,
        pub field1: 1,
    }
}

fn main() {
    let mut baz = foo::Bar::new([0]);
    baz.set_field1(true);
    println!("field1: {:?}", baz.get_field1());
}
