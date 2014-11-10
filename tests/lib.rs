#![feature(phase)]

#[phase(plugin)]
extern crate bitfield;

use std::{u8, u16, u32, u64};

bitfield!(BitfiedlTestStruct1,
    aligned_u8: 8,
    aligned_u16: 16,
    aligned_u32: 32,
    aligned_u64: 64,
    aligned_bool: 1,
    unaligned_u8: 8,
    unaligned_u16: 16,
    unaligned_u32: 32,
    unaligned_u64: 64, 
    unaligned_bool: 1,
    unaligned_2 : 2,
    unaligned_3 : 3,
    unaligned_7 : 7,
    unaligned_9 : 9,
    unaligned_15 : 15,
    unaligned_17 : 17,
    unaligned_31 : 31,
    unaligned_33 : 33,
    unaligned_63 : 63,
    )
    
#[test]
fn get_set_aligned() {
   let data = [0, ..53];
   let mut foo = BitfiedlTestStruct1::new(data);
   
   assert!(foo.get_aligned_u8() == 0);
   foo.set_aligned_u8(u8::MAX);
   assert!(foo.get_aligned_u8() == u8::MAX);
   
   assert!(foo.get_aligned_u16() == 0);
   foo.set_aligned_u16(u16::MAX);
   assert!(foo.get_aligned_u16() == u16::MAX);
   
   assert!(foo.get_aligned_u32() == 0);
   println!("{}", foo.data.as_slice());
   foo.set_aligned_u32(u32::MAX);
   println!("{}", foo.data.as_slice());
   assert!(foo.get_aligned_u32() == u32::MAX);
   
   assert!(foo.get_aligned_u64() == 0);
   foo.set_aligned_u64(u64::MAX);
   assert!(foo.get_aligned_u64() == u64::MAX);

   assert!(foo.get_aligned_bool() == false);
   foo.set_aligned_bool(true);
   assert!(foo.get_aligned_bool() == true);
}

#[test]
fn get_set_unaligned_whole_size() {
   let data = [0, ..53];
   let mut foo = BitfiedlTestStruct1::new(data);
   
   assert!(foo.get_unaligned_u8() == 0);
   foo.set_unaligned_u8(u8::MAX);
   assert!(foo.get_unaligned_u8() == u8::MAX);
   
   assert!(foo.get_unaligned_u16() == 0);
   foo.set_unaligned_u16(u16::MAX);
   assert!(foo.get_unaligned_u16() == u16::MAX);
   
   assert!(foo.get_unaligned_u32() == 0);
   foo.set_unaligned_u32(u32::MAX);
   assert!(foo.get_unaligned_u32() == u32::MAX);
   
   assert!(foo.get_unaligned_u64() == 0);
   foo.set_unaligned_u64(u64::MAX);
   assert!(foo.get_unaligned_u64() == u64::MAX);

   assert!(foo.get_unaligned_bool() == false);
   foo.set_unaligned_bool(true);
   assert!(foo.get_unaligned_bool() == true);
}

#[test]
fn get_set_unaligned_nonwhole_size(){
   let data = [0, ..53];
   let mut foo = BitfiedlTestStruct1::new(data);
   
   assert!(foo.get_unaligned_2() == 0);
   foo.set_unaligned_2(u8::MAX);
   assert!(foo.get_unaligned_2() == 0x3);
   
   assert!(foo.get_unaligned_3() == 0);
   foo.set_unaligned_3(u8::MAX);
   assert!(foo.get_unaligned_3() == 0x7);
   
   assert!(foo.get_unaligned_7() == 0);
   foo.set_unaligned_7(u8::MAX);
   assert!(foo.get_unaligned_7() == 0x7F);
   
   assert!(foo.get_unaligned_9() == 0);
   foo.set_unaligned_9(u16::MAX);
   assert!(foo.get_unaligned_9() == 0x1FF);
   
   assert!(foo.get_unaligned_15() == 0);
   foo.set_unaligned_15(u16::MAX);
   assert!(foo.get_unaligned_15() == 0x7FFF);
   
   assert!(foo.get_unaligned_17() == 0);
   foo.set_unaligned_17(u32::MAX);
   assert!(foo.get_unaligned_17() == 0x1FFFF);
   
   assert!(foo.get_unaligned_31() == 0);
   foo.set_unaligned_31(u32::MAX);
   assert!(foo.get_unaligned_31() == 0x7FFFFFFF);
   
   assert!(foo.get_unaligned_33() == 0);
   foo.set_unaligned_33(u64::MAX);
   assert!(foo.get_unaligned_33() == 0x1FFFFFFFF);
   
   assert!(foo.get_unaligned_63() == 0);
   foo.set_unaligned_63(u64::MAX);
   assert!(foo.get_unaligned_63() == 0x7FFFFFFFFFFFFFFF);
}

bitfield!(BitfiedlTestStruct2,
    aligned_u8: [8, ..3],
    aligned_u16: [16, ..3],
    aligned_u32: [32, ..3],
    aligned_u64: [64, ..3],
    aligned_bool: [1, ..3],
    unaligned_u8: [8, ..3],
    unaligned_u16: [16, ..3],
    unaligned_u32: [32, ..3],
    unaligned_u64: [64, ..3],
    unaligned_bool: [1, ..3],
    unaligned_2 : [2, ..3],
    unaligned_3 : [3, ..3],
    unaligned_7 : [7, ..3],
    unaligned_9 : [9, ..3],
    unaligned_15 : [15, ..3],
    unaligned_17 : [17, ..3],
    unaligned_31 : [31, ..3],
    unaligned_33 : [33, ..3],
    unaligned_63 : [63, ..3],
    )
    
#[test]
fn get_set_aligned_array() {
   let data = [0, ..159];
   let mut foo = BitfiedlTestStruct2::new(data);
   
   assert!(foo.get_aligned_u8() == [0, 0, 0]);
   foo.set_aligned_u8([42, 0, u8::MAX]);
   assert!(foo.get_aligned_u8() == [42, 0, u8::MAX]);
   
   assert!(foo.get_aligned_u16() == [0, 0, 0]);
   foo.set_aligned_u16([42, 0, u16::MAX]);
   assert!(foo.get_aligned_u16() == [42, 0, u16::MAX]);
   
   assert!(foo.get_aligned_u32() == [0, 0, 0]);
   println!("{}", foo.data.as_slice());
   foo.set_aligned_u32([42, 0, u32::MAX]);
   println!("{}", foo.data.as_slice());
   assert!(foo.get_aligned_u32() == [42, 0, u32::MAX]);
   
   assert!(foo.get_aligned_u64() == [0, 0, 0]);
   foo.set_aligned_u64([42, 0, u64::MAX]);
   assert!(foo.get_aligned_u64() == [42, 0, u64::MAX]);

   assert!(foo.get_aligned_bool() == [false, false, false]);
   foo.set_aligned_bool([false, false, true]);
   assert!(foo.get_aligned_bool() == [false, false, true]);
}

#[test]
fn get_set_unaligned_whole_size_array() {
   let data = [0, ..159];
   let mut foo = BitfiedlTestStruct2::new(data);
   
   assert!(foo.get_unaligned_u8() == [0, 0, 0]);
   foo.set_unaligned_u8([42, 0, u8::MAX]);
   assert!(foo.get_unaligned_u8() == [42, 0, u8::MAX]);
   
   assert!(foo.get_unaligned_u16() == [0, 0, 0]);
   foo.set_unaligned_u16([42, 0, u16::MAX]);
   assert!(foo.get_unaligned_u16() == [42, 0, u16::MAX]);
   
   assert!(foo.get_unaligned_u32() == [0, 0, 0]);
   println!("{}", foo.data.as_slice());
   foo.set_unaligned_u32([42, 0, u32::MAX]);
   println!("{}", foo.data.as_slice());
   assert!(foo.get_unaligned_u32() == [42, 0, u32::MAX]);
   
   assert!(foo.get_unaligned_u64() == [0, 0, 0]);
   foo.set_unaligned_u64([42, 0, u64::MAX]);
   assert!(foo.get_unaligned_u64() == [42, 0, u64::MAX]);

   assert!(foo.get_unaligned_bool() == [false, false, false]);
   foo.set_unaligned_bool([false, false, true]);
   assert!(foo.get_unaligned_bool() == [false, false, true]);
}

#[test]
fn get_set_unaligned_nonwhole_size_array(){
   let data = [0, ..159];
   let mut foo = BitfiedlTestStruct2::new(data);
   
   assert!(foo.get_unaligned_2() == [0, 0, 0]);
   foo.set_unaligned_2([42, 0, u8::MAX]);
   assert!(foo.get_unaligned_2() == [0x2, 0, 0x3]);
   
   assert!(foo.get_unaligned_3() == [0, 0, 0]);
   foo.set_unaligned_3([42, 0, u8::MAX]);
   assert!(foo.get_unaligned_3() == [0x2, 0, 0x7]);
   
   assert!(foo.get_unaligned_7() == [0, 0, 0]);
   foo.set_unaligned_7([42, 0, u8::MAX]);
   assert!(foo.get_unaligned_7() == [42, 0, 0x7F]);
   
   assert!(foo.get_unaligned_9() == [0, 0, 0]);
   foo.set_unaligned_9([42, 0, u16::MAX]);
   assert!(foo.get_unaligned_9() == [42, 0, 0x1FF]);
   
   assert!(foo.get_unaligned_15() == [0, 0, 0]);
   foo.set_unaligned_15([42, 0, u16::MAX]);
   assert!(foo.get_unaligned_15() == [42, 0, 0x7FFF]);
   
   assert!(foo.get_unaligned_17() == [0, 0, 0]);
   foo.set_unaligned_17([42, 0, u32::MAX]);
   assert!(foo.get_unaligned_17() == [42, 0, 0x1FFFF]);
   
   assert!(foo.get_unaligned_31() == [0, 0, 0]);
   foo.set_unaligned_31([42, 0, u32::MAX]);
   assert!(foo.get_unaligned_31() == [42, 0, 0x7FFFFFFF]);
   
   assert!(foo.get_unaligned_33() == [0, 0, 0]);
   foo.set_unaligned_33([42, 0, u64::MAX]);
   assert!(foo.get_unaligned_33() == [42, 0, 0x1FFFFFFFF]);
   
   assert!(foo.get_unaligned_63() == [0, 0, 0]);
   foo.set_unaligned_63([42, 0, u64::MAX]);
   assert!(foo.get_unaligned_63() == [42, 0, 0x7FFFFFFFFFFFFFFF]);
}
