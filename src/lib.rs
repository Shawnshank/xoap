// Xoap lib CoAP for ARM microcontrollers

#![no_std]

extern crate heapless;

use heapless::consts::*;
use heapless::Vec;

pub mod header;
pub mod options;
pub mod packet;

struct Resources {
    path: Vec<u8, U10>,
}

pub struct Xoap {
    packet: packet::Packet,
    resources: Vec<u8, U255>,
    //header: header::Header,
    //options: u8,
}

impl Xoap {
    pub fn new() -> Result<Self, ()> {
        let xoap = Xoap {
            packet: packet::Packet::new().unwrap(),
            resources: Vec::<u8, U255>::new(),
        };
        Ok(xoap)
    }

    pub fn update(&mut self, inc_data: &[u8], len: u8) {
        self.packet.decode_packet(inc_data, len)
        //self.header.decode(inc_data);
    }

    //pub fn register_resource(&mut self, )
}
