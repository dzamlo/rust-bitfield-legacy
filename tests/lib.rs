#[plugin]
extern crate bitfield;

use std::{u8, u16, u32, u64};

bitfield!{BitfiedlTestStruct1,
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
    }
    
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
   foo.set_aligned_u32(u32::MAX);
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

bitfield!{BitfiedlTestStruct2,
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
    }
    
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
   foo.set_aligned_u32([42, 0, u32::MAX]);
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
   foo.set_unaligned_u32([42, 0, u32::MAX]);
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

bitfield!{BitfiedlTestStruct3,
    byte: [1, ..8],
    }
    
#[test]
fn bits_pos() {
    let data = [0];
    let mut foo = BitfiedlTestStruct3::new(data);
    assert!(foo.get_byte() == [false, ..8]);
    foo.data[0] = 0xFF;
    assert!(foo.get_byte() == [true, ..8]);
    foo.data[0] = 0b11000011;
    assert!(foo.get_byte() == [true, true, false, false, false, false, true, true]);
    foo.data[0] = 0b10101010;
    assert!(foo.get_byte() == [true, false, true, false, true, false, true, false]);
    foo.set_byte([true, ..8]);
    assert!(foo.data[0] == 0xFF);
    foo.set_byte([false, ..8]);
    assert!(foo.data[0] == 0x00);
    foo.set_byte([true, true, false, false, false, false, true, true]);
    assert!(foo.data[0] == 0b11000011);
    foo.set_byte([true, false, true, false, true, false, true, false]);
    assert!(foo.data[0] == 0b10101010);
}

bitfield!{BitfiedlTestStruct4,
    value: 16,
    }

#[test]
fn byte_order_16() {
   let data = [0, ..2];
   let mut foo = BitfiedlTestStruct4::new(data);
   assert!(foo.get_value() == 0x0000);
   foo.data = [0xAB, 0xCD];
   assert!(foo.get_value() == 0xABCD);
   foo.data = [0xCD, 0xAB];
   assert!(foo.get_value() == 0xCDAB);
   foo.set_value(0x1234);
   assert!(foo.data == [0x12, 0x34]);
   foo.set_value(0x3412);
   assert!(foo.data == [0x34, 0x12]);
}

bitfield!{BitfiedlTestStruct5,
    value: 32,
    }

#[test]
fn byte_order_32() {
   let data = [0, ..4];
   let mut foo = BitfiedlTestStruct5::new(data);
   assert!(foo.get_value() == 0x00000000);
   foo.data = [0x12, 0x34, 0x56, 0x78];
   assert!(foo.get_value() == 0x12345678);
   foo.set_value(0xFEDCBA98);
   assert!(foo.data == [0xFE, 0xDC, 0xBA, 0x98]);
}

bitfield!{BitfiedlTestStruct6,
    value: 64,
    }

#[test]
fn byte_order_64() {
   let data = [0, ..8];
   let mut foo = BitfiedlTestStruct6::new(data);
   assert!(foo.get_value() == 0x0000000000000000);
   foo.data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
   assert!(foo.get_value() == 0x123456789ABCDEF0);
   foo.set_value(0xFEDCBA9876543210);
   assert!(foo.data == [0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10]);
}

bitfield!{BitfiedlTestStruct7,
    field1: 0x10,
    field2: 0o10,
    field3: 0b10,
    field4: 6_0,
    field5: [0xA, ..0b11],
    }

#[test]
fn other_literal_form_for_fields_size() {
   let data = [0, ..15];
   let mut foo = BitfiedlTestStruct7::new(data);
   
   assert!(foo.get_field1() == 0x0);
   foo.set_field1(u16::MAX);
   assert!(foo.get_field1() == u16::MAX);
   
   assert!(foo.get_field2() == 0x0);
   foo.set_field2(u8::MAX);
   assert!(foo.get_field2() == u8::MAX);
   
   assert!(foo.get_field3() == 0x0);
   foo.set_field3(u8::MAX);
   assert!(foo.get_field3() == 0x3);
   
   assert!(foo.get_field4() == 0x0);
   foo.set_field4(u64::MAX);
   assert!(foo.get_field4() == 0xFFFFFFFFFFFFFFF);
   
   assert!(foo.get_field5() == [0, 0, 0]);
   foo.set_field5([42, 0, u16::MAX]);
   assert!(foo.get_field5() == [42, 0, 0x3FF]);
}
