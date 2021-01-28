#![no_std]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]
#![deny(missing_docs)]


//! XoAP - CoAP for Embedded systems w/o allocator
//! 

use heapless::consts::*;
use heapless::{String, Vec};

mod message;

use message::header::CoapHeader;
use message::header::{CoapHeaderCode, CoapHeaderType};
use message::option::{CoapOption, CoapOptions};
use message::option::{CoapOptionNumbers, CoapOptionError};


#[derive(Debug)]
pub(crate) enum CoapError {
    ConfigError,
    OptionError(CoapOptionError),
    OptionsError(CoapOptionError),
    HeaderError,
    MessageError,
}

/// A CoAP resource, an endpoint that is being requested.
/// For example ```house/livingroom/temperature```
/// 
/// Takes the endpoint path and a callback function that will be executed when the enpoint is called
/// 
#[derive(Debug, Clone)]
pub struct CoapResource {
    callback: fn() -> u8,
    path: String<U255>,
}

impl CoapResource {
    /// Returns the enpoint path for the particular resource
    pub fn get_path(&self) -> String<U255> {
        self.path.clone()
    }

    /// Returns the callback function associated with the particular resource
    pub fn callback(&self) -> fn() -> u8 {
        self.callback
    }
}

/// CoAP server/client configuration struct.
/// Needs to be passed to the server/client during creation
pub struct CoapConfig {
    resources: Vec<CoapResource, U8>,
}

impl CoapConfig {
    /// Creates a new vector of (empty) resources
    pub fn new() -> Self {
        CoapConfig {
            resources: Vec::<CoapResource, U8>::new(),
        }
    }
    /// Adds a resource to the configuration
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

/// CoAP server.
/// Creates a CoAP server acting behavoir.
/// Takes a CoAP config struct and a buffer for message storage.
pub struct CoapServer<'a> {
    config: CoapConfig,
    buffer: &'a [u8],
}

impl<'a> CoapServer<'a> {
    /// Creates a new CoAP server
    pub fn new(config: CoapConfig, buffer: &'a mut [u8]) -> Self {
        CoapServer { config, buffer }
    }
    /// Handels a message and returns the response to be sent of to the request owner
    pub fn handle_message(self, msg: &mut [u8]) -> Vec<u8, U255> {
        let request = match message::CoapMessage::decode(msg) {
            Ok(msg) => msg,
            Err(_) => panic!(), // TODO: Handle error with specific coap response message
        };

        let response = match request.header.get_code() {
            CoapHeaderCode::EMPTY => {
                let header = CoapHeader::new(
                    CoapHeaderType::Reset,
                    request.header.get_tkl(),
                    CoapHeaderCode::EMPTY,
                    request.header.get_message_id(),
                );
                let message = message::CoapMessage::new(header, &[]);
                Some(message)
            }
            CoapHeaderCode::GET => self.handle_get(request),
            CoapHeaderCode::POST => self.handle_post(request),
            CoapHeaderCode::PUT => self.handle_put(request),
            CoapHeaderCode::DELETE => self.handle_delete(request),
            _ => match request.header.get_type() {
                CoapHeaderType::Confirmable
                | CoapHeaderType::Reset
                | CoapHeaderType::Acknowledgement => {
                    let header = CoapHeader::new(
                        CoapHeaderType::Acknowledgement,
                        request.header.get_tkl(),
                        CoapHeaderCode::MethodNotAllowed,
                        request.header.get_message_id(),
                    );
                    let message = message::CoapMessage::new(header, &[]);
                    Some(message)
                }
                CoapHeaderType::NonConfirmable => {
                    let header = CoapHeader::new(
                        CoapHeaderType::NonConfirmable,
                        request.header.get_tkl(),
                        CoapHeaderCode::MethodNotAllowed,
                        request.header.get_message_id(),
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
        let mut uri_path: String<U255> = String::new();
        let mut amount_of_uri_path_options: usize = 0;
        for opt in msg.options.options.iter() {
            match opt.get_option_number() {
                CoapOptionNumbers::UriPath => {
                    if amount_of_uri_path_options != 0 {
                        uri_path.push_str("/").unwrap();
                    }
                    amount_of_uri_path_options += 1;
                    uri_path.push_str(String::from_utf8(opt.get_option_data()).unwrap().as_str()).unwrap();
                },
                _ => panic!(),
            }
        }
        if amount_of_uri_path_options > 0 {
            for res in self.config.resources.iter() {
                if uri_path == res.get_path() {
                    payload = res.callback()();
                }
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
        config.add_resource(test, "test");
        let mut buffer: [u8; 1024] = [0; 1024];
        let server = CoapServer::new(config, &mut buffer);

        let header = CoapHeader::new(CoapHeaderType::Confirmable, 2, CoapHeaderCode::GET, 123);
        let mut msg = message::CoapMessage::new(header, &[1, 2]);
        let option = CoapOption::new(
            CoapOptionNumbers::UriPath,
            "test".as_bytes(),
        );
        msg.add_option(option).unwrap();
        msg.set_token(&[100, 101]).unwrap();
        let mut raw_msg = msg.encode().unwrap();
        let resp = server.handle_message(&mut raw_msg.0);

        let expected_response = [98, 69, 0, 123, 255, test()];
        let mut ex_resp = Vec::<u8, U255>::from_slice(&expected_response).unwrap();
        ex_resp.truncate(expected_response.len());

        assert_eq!(ex_resp, resp);
    }

    fn test() -> u8 {
        assert_eq!("foo", "foo");
        1
    }

    fn test_level() -> u8 {
        2
    }

    fn test_level_cheese() -> u8 {
        3
    }

    #[test]
    fn multiple_uri_path() {
        let mut config = CoapConfig::new();
        config.add_resource(test, "test");
        config.add_resource(test_level, "test/level");
        config.add_resource(test_level_cheese, "test/level/cheese");

        let mut buffer: [u8; 1024] = [0; 1024];
        let server = CoapServer::new(config, &mut buffer);

        let header = CoapHeader::new(CoapHeaderType::Confirmable, 2, CoapHeaderCode::GET, 123);
        let mut msg = message::CoapMessage::new(header, &[1, 2]);
        let option = CoapOption::new(
            CoapOptionNumbers::UriPath,
            "test".as_bytes(),
        );
        let option_2 = CoapOption::new(
            CoapOptionNumbers::UriPath,
            "level".as_bytes(),
        );
        let option_3 = CoapOption::new(
            CoapOptionNumbers::UriPath,
            "cheese".as_bytes(),
        );
        msg.add_option(option).unwrap();
        msg.add_option(option_2).unwrap();
        msg.add_option(option_3).unwrap();
        msg.set_token(&[100, 101]).unwrap();
        let mut raw_msg = msg.encode().unwrap();
        let resp = server.handle_message(&mut raw_msg.0);

        let expected_response = [98, 69, 0, 123, 255, test_level_cheese()];
        let mut ex_resp = Vec::<u8, U255>::from_slice(&expected_response).unwrap();
        ex_resp.truncate(expected_response.len());

        assert_eq!(ex_resp, resp);
    }

    #[test]
    fn endpoint_not_found() {
        let mut config = CoapConfig::new();
        config.add_resource(test, "test");
        config.add_resource(test_level, "test/level");
        config.add_resource(test_level_cheese, "test/level/cheese");

        let mut buffer: [u8; 1024] = [0; 1024];
        let server = CoapServer::new(config, &mut buffer);

        let header = CoapHeader::new(CoapHeaderType::Confirmable, 2, CoapHeaderCode::GET, 123);
        let mut msg = message::CoapMessage::new(header, &[1, 2]);
        let option = CoapOption::new(
            CoapOptionNumbers::UriPath,
            "test".as_bytes(),
        );
        let option_2 = CoapOption::new(
            CoapOptionNumbers::UriPath,
            "level".as_bytes(),
        );
        let option_3 = CoapOption::new(
            CoapOptionNumbers::UriPath,
            "wrongEndpoint".as_bytes(),
        );
        msg.add_option(option).unwrap();
        msg.add_option(option_2).unwrap();
        msg.add_option(option_3).unwrap();
        msg.set_token(&[100, 101]).unwrap();
        let mut raw_msg = msg.encode().unwrap();
        let resp = server.handle_message(&mut raw_msg.0);

        let expected_response = [98, 132, 0, 123];
        let mut ex_resp = Vec::<u8, U255>::from_slice(&expected_response).unwrap();
        ex_resp.truncate(expected_response.len());

        assert_eq!(ex_resp, resp);
    }
}
