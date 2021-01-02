use crate::CoapError;
use heapless::consts::*;
use heapless::Vec;

mod header;
mod option;

#[derive(Clone)]
pub struct CoapToken {
    token: Vec<u8, U8>,
    length: usize,
}

impl CoapToken {
    pub fn new() -> Self {
        CoapToken {
            token: Vec::<u8, U8>::new(),
            length: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }
}

pub struct CoapMessage {
    header: header::CoapHeader,
    token: CoapToken,
    options: option::CoapOptions,
    payload_marker: u8,
    payload: Vec<u8, U255>,
    payload_length: usize,
}

impl CoapMessage {
    pub fn new(header: header::CoapHeader, payload: &[u8]) -> Self {
        let payload_length = payload.len();
        let payload = Vec::from_slice(payload).unwrap();
        CoapMessage {
            header,
            token: CoapToken::new(),
            options: option::CoapOptions::new(),
            payload_marker: 0xff,
            payload: payload,
            payload_length,
        }
    }

    pub fn set_token(&mut self, token: &[u8]) -> Result<(), CoapError> {
        if token.len() > 8 {
            return Err(CoapError::MessageError);
        }
        self.token.length = token.len();
        self.token.token = Vec::from_slice(token).unwrap();

        Ok(())
    }

    pub fn add_option(&mut self, option: option::CoapOption) -> Result<(), CoapError> {
        self.options.push(option)?;
        Ok(())
    }

    pub fn encode(&mut self) -> Result<([u8; 1024], usize), CoapError> {
        let mut index = 0;
        let mut msg: [u8; 1024] = [0; 1024];
        let header = self.header.encode();
        for i in &header {
            msg[index] = *i;
            index += 1;
        }
        if self.token.len() != 0 {
            for i in 0..self.token.len() {
                msg[index] = self.token.token[i];
                index += 1;
            }
        }
        if self.options.len() != 0 {
            let mut prev_option = option::CoapOptionNumbers::Zero;
            for _i in 0..self.options.len() {
                let option = self.options.pop()?;
                let o = self.options.pop()?.encode(prev_option)?;
                for j in 0..o.1 {
                    msg[index] = o.0[j];
                    index += 1;
                }
                prev_option = option.get_option_number();
            }
        }
        if self.payload_length != 0 {
            msg[index] = self.payload_marker;
            index += 1;
            for i in 0..self.payload_length  {
                msg[index] = self.payload[i];
                index += 1;
            }
        }

        Ok((msg, index))
    }
}

#[cfg(test)]
mod tests {
    use crate::message;
    use crate::message::header;
    use crate::message::option;

    #[test]
    fn encode() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut msg = message::CoapMessage::new(
            header::CoapHeader::new(
                header::CoapHeaderType::Acknowledgement,
                0,
                header::CoapHeaderCode::Content,
                123,
            ),
            &data,
        );
        let en_msg = msg.encode().unwrap();

        let arr_msg = 
        assert_eq!(en_msg.0[..en_msg.1], [0;1024][..en_msg.1]);
        assert_eq!(en_msg.1, 14);
    }
}
