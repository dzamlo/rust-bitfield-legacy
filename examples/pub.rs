#![feature(plugin)]
#![plugin(bitfield)]

mod foo {
    bitfield!{pub Bar,
        pub field1: 1,
        #[allow(dead_code)]
        pub field2: [3; 4],
    }
}

fn main() {
    let mut baz = foo::Bar::new([0; 2]);
    baz.set_field1(true);
    println!("field1: {:?}", baz.get_field1());
    println!("field1: {:?}", baz.get_field2());
}
