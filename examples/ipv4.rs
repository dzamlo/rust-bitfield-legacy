#![feature(phase)]
#![allow(dead_code)]

#[phase(plugin)]
extern crate bitfield;

use std::io::net::ip::{IpAddr, Ipv4Addr};

bitfield!(IpV4Header,
    version: 4,
    ihl: 4,
    dscp: 5,
    ecn: 3,
    total_length: 16,
    identification: 16,
    reserved: 1,
    df: 1,
    mf: 1,
    fragment_offset: 13,
    time_to_live: 8,
    protocol: 8,
    header_checksum: 16,
    source_address: [8, ..4],
    destination_address: [8, ..4], 
    )
    
impl IpV4Header {
    fn get_source_as_ip_addr(&self) -> IpAddr {
       let src = self.get_source_address();
       Ipv4Addr(src[0], src[1], src[2], src[3])
    }

    fn get_destination_as_ip_addr(&self) -> IpAddr {
       let dst = self.get_destination_address();
       Ipv4Addr(dst[0], dst[1], dst[2], dst[3])
    }
}

fn main() {
    let data = [0x45, 0x00, 0x00, 0x40, 0x69, 0x27, 0x40, 0x00, 0x40, 0x11, 
                0x4d, 0x0d, 0xc0, 0xa8, 0x01, 0x2a, 0xc0, 0xa8, 0x01, 0xfe];
    
    let header = IpV4Header::new(data);
    
    assert!(header.get_version() == 4);
    assert!(header.get_total_length() == 64);
    assert!(header.get_identification() == 0x6927);
    assert!(header.get_df());
    assert!(!header.get_mf());
    assert!(header.get_fragment_offset() == 0);
    assert!(header.get_protocol() == 0x11);
    assert!(header.get_source_address() == [192, 168, 1, 42]);
    println!("from {} to {}", header.get_source_as_ip_addr(), header.get_destination_as_ip_addr());
}

