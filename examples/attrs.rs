#![feature(plugin)]
#![allow(dead_code)]
#![plugin(bitfield)]
#![deny(missing_docs)]

//! You can use the same attributes as for `struct`s.
//! The attributes only apply to the struct declaration, not the impl/methods
//!
//! You can also set attributes for each field.
//! They will apply to the getter and setter methods.

bitfield!{
    /// You can put documentation comments
    #[derive(Debug, Clone)]
    #[allow(non_camel_case_types)]
    pub demo_attrs,
    /// The first field
    pub field1: 3,
    #[allow(missing_docs)]
    pub field2: 10,
    /// The third field
    pub field3: [2; 4],
    #[allow(missing_docs)]
    pub field4: [2; 7],
}

fn main() {
    let mut foo = demo_attrs::new([0; 5]);
    let bar = foo.clone();
    foo.set_field1(5);
    println!("foo: {:?}", foo);
    println!("bar: {:?}", bar);
}
