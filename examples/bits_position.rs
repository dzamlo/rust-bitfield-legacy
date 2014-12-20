#![feature(phase)]
#![allow(dead_code)]

#[phase(plugin)]
extern crate bitfield;

bitfield!{IpV4Header,
    field1: 7,
    field2: 7,
    field3: 10,
    }

fn main() {
    let data = [0, ..3];
    
    let mut header = IpV4Header::new(data);
    header.set_field1(0x7f);
    println!("Field 1 set to all 1:");
    println!("[{:08b}, {:08b}, {:08b}]", header.data[0], header.data[1], header.data[2]);
    header.set_field1(0);
    header.set_field2(0x7F);
    println!("Field 2 set to all 1:");
    println!("[{:08b} {:08b}, {:08b}]", header.data[0], header.data[1], header.data[2]);
    header.set_field2(0);
    header.set_field3(0x3FF);
    println!("Field 3 set to all 1:");
    println!("[{:08b}, {:08b}, {:08b}]", header.data[0], header.data[1], header.data[2]);
}

