#![no_std]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

use heapless::consts::*;
use heapless::{String, Vec};

mod message;

use message::header::CoapHeader;
use message::header::{CoapHeaderCode, CoapHeaderType};

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
    callback: fn() -> u8,
    path: String<U255>,
}

impl CoapResource {
    pub fn get_path(&self) -> String<U255> {
        self.path.clone()
    }

    pub fn callback(&self) -> fn() -> u8 {
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

    pub fn add_resource(&mut self, cb: fn() -> u8, path: &str) /*-> Result<(), CoapError>*/
    {
        let res = CoapResource {
            callback: cb,
            path: String::from(path),
        };
        self.resources.push(res).unwrap();
        //Ok(())
    }
}

pub struct CoapServer<'a> {
    config: CoapConfig,
    buffer: &'a [u8],
}

impl<'a> CoapServer<'a> {
    pub fn new(config: CoapConfig, buffer: &'a mut [u8]) -> Self {
        CoapServer { config, buffer }
    }
    pub fn handle_message(self, msg: &mut [u8]) -> Vec<u8, U255> {
        let requset = match message::CoapMessage::decode(msg) {
            Ok(msg) => msg,
            Err(_) => panic!(), // Handle error with specific coap response message
        };

        let response = match requset.header.get_code() {
            CoapHeaderCode::EMPTY => {
                let header = CoapHeader::new(
                    CoapHeaderType::Reset,
                    requset.header.get_tkl(),
                    CoapHeaderCode::EMPTY,
                    requset.header.get_message_id(),
                );
                let message = message::CoapMessage::new(header, &[]);
                Some(message)
            }
            CoapHeaderCode::GET => self.handle_get(requset),
            CoapHeaderCode::POST => self.handle_post(requset),
            CoapHeaderCode::PUT => self.handle_put(requset),
            CoapHeaderCode::DELETE => self.handle_delete(requset),
            _ => match requset.header.get_type() {
                CoapHeaderType::Confirmable
                | CoapHeaderType::Reset
                | CoapHeaderType::Acknowledgement => {
                    let header = CoapHeader::new(
                        CoapHeaderType::Acknowledgement,
                        requset.header.get_tkl(),
                        CoapHeaderCode::MethodNotAllowed,
                        requset.header.get_message_id(),
                    );
                    let message = message::CoapMessage::new(header, &[]);
                    Some(message)
                }
                CoapHeaderType::NonConfirmable => {
                    let header = CoapHeader::new(
                        CoapHeaderType::NonConfirmable,
                        requset.header.get_tkl(),
                        CoapHeaderCode::MethodNotAllowed,
                        requset.header.get_message_id(),
                    );
                    let message = message::CoapMessage::new(header, &[]);
                    Some(message)
                }
            },
        };

        let encoded_response = response.unwrap().encode().unwrap();

        let mut msg_resp = Vec::<u8, U255>::from_slice(&encoded_response.0).unwrap();
        msg_resp.truncate(encoded_response.1);
        msg_resp
    }

    fn handle_get(self, mut msg: message::CoapMessage) -> Option<message::CoapMessage> {
        let mut payload: u8 = 0;
        while msg.options.len() > 0 {
            let option = msg.options.pop().unwrap();
            match option.get_option_number() {
                message::option::CoapOptionNumbers::UriPath => {
                    for res in self.config.resources.iter() {
                        if option.get_option_data() == res.get_path().into_bytes() {
                            payload = res.callback()();
                        }
                    }
                    if payload == 0 {
                        let header_type = CoapHeaderType::Acknowledgement;
                        let header_code = CoapHeaderCode::NotFound;
                        let header = CoapHeader::new(
                            header_type,
                            msg.header.get_tkl(),
                            header_code,
                            msg.header.get_message_id(),
                        );
                        let response = message::CoapMessage::new(header, &[]);
                        return Some(response);
                    }
                }
                // Add more options
                _ => panic!(),
            }
        }
        if msg.header.get_type() == CoapHeaderType::Confirmable {
            let header_type = CoapHeaderType::Acknowledgement;
            let header_code = CoapHeaderCode::Content;
            let header = CoapHeader::new(
                header_type,
                msg.header.get_tkl(),
                header_code,
                msg.header.get_message_id(),
            );
            let response = message::CoapMessage::new(header, &[payload]);
            return Some(response);
        } else if msg.header.get_type() == CoapHeaderType::NonConfirmable {
            let header_type = CoapHeaderType::NonConfirmable;
            let header_code = CoapHeaderCode::Content;
            let header = CoapHeader::new(
                header_type,
                msg.header.get_tkl(),
                header_code,
                msg.header.get_message_id(),
            );
            let response = message::CoapMessage::new(header, &[payload]);
            return Some(response);
        } else {
            return None;
        }
    }

    fn handle_post(self, _msg: message::CoapMessage) -> Option<message::CoapMessage> {
        None
    }

    fn handle_put(self, _msg: message::CoapMessage) -> Option<message::CoapMessage> {
        None
    }

    fn handle_delete(self, _msg: message::CoapMessage) -> Option<message::CoapMessage> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn resource_calling() {
        let mut config = CoapConfig::new();
        config.add_resource(dummy_function, "test");
        let mut buffer: [u8; 1024] = [0; 1024];
        let server = CoapServer::new(config, &mut buffer);

        let header = CoapHeader::new(CoapHeaderType::Confirmable, 2, CoapHeaderCode::GET, 123);
        let mut msg = message::CoapMessage::new(header, &[1, 2]);
        let option = message::option::CoapOption::new(
            message::option::CoapOptionNumbers::UriPath,
            "test".as_bytes(),
        );
        msg.add_option(option).unwrap();
        msg.set_token(&[100, 101]).unwrap();
        let mut raw_msg = msg.encode().unwrap();
        let resp = server.handle_message(&mut raw_msg.0);

        let expected_response = [98, 69, 0, 123, 255, dummy_function()];
        let mut ex_resp = Vec::<u8, U255>::from_slice(&expected_response).unwrap();
        ex_resp.truncate(expected_response.len());

        assert_eq!(ex_resp, resp);
    }

    fn dummy_function() -> u8 {
        assert_eq!("foo", "foo");
        1
    }
}
