#![no_std]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

use heapless::consts::*;
use heapless::{String, Vec};

mod message;
mod response;

#[derive(Debug)]
pub enum CoapError {
    ConfigError,
    OptionError(message::option::CoapOptionError),
    OptionsError(message::option::CoapOptionError),
    HeaderError,
    MessageError,
}

#[derive(Debug, Clone)]
pub struct CoapResource {
    callback: fn(),
    path: String<U255>,
}

impl CoapResource {
    pub fn get_path(&self) -> String<U255> {
        self.path.clone()
    }

    pub fn get_callback(&self) -> fn() {
        self.callback
    }
}

pub struct CoapConfig {
    resources: Vec<CoapResource, U8>,
}

impl CoapConfig {
    pub fn new() -> Self {
        CoapConfig {
            resources: Vec::<CoapResource, U8>::new(),
        }
    }

    pub fn add_resource(&mut self, cb: fn(), path: String<U255>) /*-> Result<(), CoapError>*/
    {
        let res = CoapResource {
            callback: cb,
            path: path,
        };
        self.resources.push(res).unwrap();
        //Ok(())
    }
}

pub struct CoapServer {
    config: CoapConfig,
}

impl CoapServer {
    pub fn new(config: CoapConfig) -> Self {
        CoapServer { config }
    }
    pub fn handle_message(self, msg: &mut [u8]) {
        let requset = match message::CoapMessage::decode(msg) {
            Ok(msg) => msg,
            Err(_) => panic!(), // Handle error with specific coap response message
        };

        let _response = match requset.header.get_code() {
            message::header::CoapHeaderCode::GET => self.handle_get(requset),
            message::header::CoapHeaderCode::POST => self.handle_post(requset),
            message::header::CoapHeaderCode::PUT => self.handle_put(requset),
            message::header::CoapHeaderCode::DELETE => self.handle_delete(requset),
            _ => panic!(),
        };
    }

    fn handle_get(self, mut msg: message::CoapMessage) {
        while msg.options.len() > 0 {
            let option = msg.options.pop().unwrap();
            match option.get_option_number() {
                message::option::CoapOptionNumbers::UriPath => {
                    for res in self.config.resources.iter() {
                        if option.get_option_data() == res.get_path().into_bytes() {
                            res.get_callback()();
                        }
                    }
                }
                _ => panic!(),
            }
        }
    }

    fn handle_post(self, _msg: message::CoapMessage) {}

    fn handle_put(self, _msg: message::CoapMessage) {}

    fn handle_delete(self, _msg: message::CoapMessage) {}
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn config() {
        let mut config = CoapConfig::new();
        config.add_resource(dummy_function, String::from("test"));
        let server = CoapServer::new(config);

        let header = message::header::CoapHeader::new(
            message::header::CoapHeaderType::Confirmable,
            0,
            message::header::CoapHeaderCode::GET,
            123,
        );
        let mut msg = message::CoapMessage::new(header, &[1,2]);
        let option = message::option::CoapOption::new(
            message::option::CoapOptionNumbers::UriPath,
            "test".as_bytes(),
        );
        msg.add_option(option).unwrap();
        let mut raw_msg = msg.encode().unwrap();
        //assert_eq!(raw_msg.0, [0;1024]);
        server.handle_message(&mut raw_msg.0);
    }

    fn dummy_function() {
        assert_eq!("foo", "baz");
    }
}
