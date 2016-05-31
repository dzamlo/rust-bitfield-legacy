#![feature(const_fn)]
#![feature(plugin)]
#![plugin(bitfield)]
#![allow(dead_code)]

static FOO1: foo::Bar = foo::Bar::new([0]);
static FOO2: foo::Bar = foo::Bar { data: [0] };

mod foo {
    bitfield!{
        #[const_new]
        #[pub_data]
        pub Bar,
        a: 1
    }
}

fn main() {}
