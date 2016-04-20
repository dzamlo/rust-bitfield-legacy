#![feature(plugin)]
#![allow(dead_code)]
#![plugin(bitfield)]

// You can use the same attributes as for structs.
// The attributes only apply to the struct declaration, not the impl/methods
bitfield!{
    /// You can put documentation comments
    #[derive(Debug, Clone)]
    #[allow(non_camel_case_types)]
    pub demo_attrs,
    field1: 3,
    field2: 10,
}

fn main() {
    let mut foo = demo_attrs::new([0, 0]);
    let bar = foo.clone();
    foo.set_field1(5);
    println!("foo: {:?}", foo);
    println!("bar: {:?}", bar);
}
