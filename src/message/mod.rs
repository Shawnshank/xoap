use crate::CoapError;
use heapless::consts::*;
use heapless::Vec;

mod header;
mod option;

pub struct CoapMessage {
    header: header::CoapHeader,
    token: Vec<u8, U8>,
    options: Vec<option::CoapOption, U10>,
    payload_marker: u8,
    payload: Vec<u8, U255>,
}

impl CoapMessage {
    pub fn new(header: header::CoapHeader, payload: &[u8]) -> Self {
        let mut payload_marker = 0xff;
        if payload.len() == 0 {
            payload_marker = 0;
        }
        let payload = Vec::from_slice(payload).unwrap();
        CoapMessage {
            header,
            token: Vec::<u8, U8>::new(),
            options: Vec::<option::CoapOption, U10>::new(),
            payload_marker,
            payload: payload,
        }
    }

    pub fn set_token(&mut self, token: &[u8]) -> Result<(), CoapError> {
        if token.len() > 8 {
            return Err(CoapError::MessageError);
        }
        self.token = Vec::from_slice(token).unwrap();

        Ok(())
    }

    pub fn add_option(&mut self, option: option::CoapOption) -> Result<(), CoapError> {
        match self.options.push(option) {
            Ok(_) => Ok(()),
            Err(_) => Err(CoapError::MessageError),
        }
    }
}
