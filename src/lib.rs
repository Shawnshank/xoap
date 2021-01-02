#![no_std]
#![feature(exclusive_range_pattern)]

use heapless::{Vec, String};
use heapless::consts::*;

mod message;

#[derive(Debug)]
pub enum CoapError {
    ConfigError,
    EncodeOptionError,
    DecodeOptionError,
}

#[derive(Debug)]
pub struct CoapResource {
    callback: fn(),
    path: String<U255>,
}

pub struct CoapConfig {
    resources: Vec::<CoapResource, U8>,
}

impl CoapConfig {
    pub fn new() -> Self {
        CoapConfig {
            resources: Vec::<CoapResource, U8>::new(),
        }
    }

    pub fn add_resource(&mut self, cb: fn(), path: String<U255>) /*-> Result<(), CoapError>*/{
        let res = CoapResource{callback: cb, path: path};
        self.resources.push(res).unwrap();
        //Ok(())
    }
}



#[cfg(test)]
mod tests {

}