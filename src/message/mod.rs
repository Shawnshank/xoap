use crate::CoapError;
use heapless::consts::*;
use heapless::Vec;

pub mod header;
pub mod option;

#[derive(Debug, Clone)]
pub enum CoapMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl From<u8> for CoapMethod {
    fn from(item: u8) -> Self {
        match item {
            1 => CoapMethod::GET,
            2 => CoapMethod::POST,
            3 => CoapMethod::PUT,
            4 => CoapMethod::DELETE,
            _ => unreachable!(),
        }
    }
}

impl From<CoapMethod> for u8 {
    fn from(item: CoapMethod) -> Self {
        match item {
            CoapMethod::GET => 1,
            CoapMethod::POST => 2,
            CoapMethod::PUT => 3,
            CoapMethod::DELETE => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoapMediaType {
    TextPlain,
    ApplicationLinkFormat,
    ApplicationXml,
    ApplicationOctetStream,
    ApplicationExi,
    ApplicationJson,
}

impl From<u8> for CoapMediaType {
    fn from(item: u8) -> Self {
        match item {
            0 => CoapMediaType::TextPlain,
            40 => CoapMediaType::ApplicationLinkFormat,
            41 => CoapMediaType::ApplicationXml,
            42 => CoapMediaType::ApplicationOctetStream,
            47 => CoapMediaType::ApplicationExi,
            50 => CoapMediaType::ApplicationJson,
            _ => unreachable!(),
        }
    }
}

impl From<CoapMediaType> for u8 {
    fn from(item: CoapMediaType) -> Self {
        match item {
            CoapMediaType::TextPlain => 0,
            CoapMediaType::ApplicationLinkFormat => 40,
            CoapMediaType::ApplicationXml => 41,
            CoapMediaType::ApplicationOctetStream => 42,
            CoapMediaType::ApplicationExi => 47,
            CoapMediaType::ApplicationJson => 50,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct CoapMessage {
    pub header: header::CoapHeader,
    token: CoapToken,
    pub options: option::CoapOptions,
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

    pub fn encode(&mut self) -> Result<([u8; 255], usize), CoapError> {
        let mut index = 0;
        let mut msg: [u8; 255] = [0; 255];
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
            for opt in self.options.options.iter() {
                let o = opt.encode(prev_option)?;
                for j in 0..o.1 {
                    msg[index] = o.0[j];
                    index += 1;
                }
                prev_option = opt.get_option_number();
            }
        }
        if self.payload_length != 0 {
            msg[index] = self.payload_marker;
            index += 1;
            for i in 0..self.payload_length {
                msg[index] = self.payload[i];
                index += 1;
            }
        }

        Ok((msg, index))
    }

    pub fn decode(buf: &mut [u8]) -> Result<Self, CoapError> {
        if buf.len() < 4 {
            return Err(CoapError::MessageError);
        }
        let (raw_header, mut rest) = buf.split_at_mut(4);
        let header = header::CoapHeader::decode(raw_header);
        let mut token: &[u8] = &[];
        if header.get_tkl() != 0 {
            let tok = rest.split_at_mut(header.get_tkl() as usize);
            token = tok.0;
            rest = tok.1;
        }
        if rest.len() == 0 {
            return Ok(CoapMessage::new(header, &[0]));
        }
        //assert_eq!(rest, &[0]);
        let (options, mut rest) = option::CoapOptions::decode(rest)?;

        if rest.len() > 1 && rest[0] == 0xff {
            rest = &rest[1..];
        }
        //assert_eq!(rest, &[0]);
        let mut new_message: CoapMessage = CoapMessage::new(header, rest);
        new_message.set_token(token)?;
        new_message.options = options;

        Ok(new_message)
    }
}

#[cfg(test)]
mod tests {
    use crate::message;
    use crate::message::header;
    use crate::message::option;

    #[test]
    fn encode_header_payload() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let header = header::CoapHeader::new(
            header::CoapHeaderType::Acknowledgement,
            0,
            header::CoapHeaderCode::Content,
            123,
        );
        let mut msg = message::CoapMessage::new(header.clone(), &data);
        let en_msg = msg.encode().unwrap();

        // Check payload marker and payload
        let expected_msg = [255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(en_msg.0[4..en_msg.1], expected_msg[..(en_msg.1 - 4)]);
        // Check payload length
        assert_eq!(en_msg.1, 4 + 1 + data.len());

        // Check header
        let de_header = header::CoapHeader::decode(&en_msg.0[0..4]);
        assert_eq!(header, de_header);
    }

    #[test]
    fn encode_decode_header_payload() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let header = header::CoapHeader::new(
            header::CoapHeaderType::Acknowledgement,
            0,
            header::CoapHeaderCode::Content,
            123,
        );
        let mut msg = message::CoapMessage::new(header.clone(), &data);
        let mut en_msg = msg.encode().unwrap();
        let buf = &mut en_msg.0[..en_msg.1];
        let de_msg = message::CoapMessage::decode(buf).unwrap();

        assert_eq!(de_msg, msg);
    }
    #[test]
    fn encode_decode_header_token_payload() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let header = header::CoapHeader::new(
            header::CoapHeaderType::Acknowledgement,
            3,
            header::CoapHeaderCode::Content,
            123,
        );
        let mut msg = message::CoapMessage::new(header.clone(), &data);
        msg.set_token(&[100, 111, 122]).unwrap();
        let mut en_msg = msg.encode().unwrap();
        let buf = &mut en_msg.0[..en_msg.1];
        let de_msg = message::CoapMessage::decode(buf).unwrap();

        assert_eq!(de_msg, msg);
    }
    #[test]
    fn encode_decode_header_token_option_payload() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let header = header::CoapHeader::new(
            header::CoapHeaderType::Acknowledgement,
            3,
            header::CoapHeaderCode::Content,
            123,
        );
        let mut msg = message::CoapMessage::new(header.clone(), &data);
        msg.set_token(&[100, 111, 122]).unwrap();
        msg.add_option(option::CoapOption::new(
            option::CoapOptionNumbers::Accept,
            &[],
        ))
        .unwrap();

        let ref_msg = msg.clone();
        let mut en_msg = msg.encode().unwrap();
        let buf = &mut en_msg.0[..en_msg.1];
        let de_msg = message::CoapMessage::decode(buf).unwrap();

        assert_eq!(de_msg, ref_msg);
    }

    #[test]
    fn encode_decode_header_token_multiple_option_payload() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let header = header::CoapHeader::new(
            header::CoapHeaderType::Acknowledgement,
            3,
            header::CoapHeaderCode::Content,
            123,
        );
        let mut msg = message::CoapMessage::new(header.clone(), &data);
        msg.set_token(&[100, 111, 122]).unwrap();
        msg.add_option(option::CoapOption::new(
            option::CoapOptionNumbers::IfNoneMatch,
            &[],
        ))
        .unwrap(); // 5
        msg.add_option(option::CoapOption::new(
            option::CoapOptionNumbers::UriPath,
            b"test",
        ))
        .unwrap(); // 11

        msg.add_option(option::CoapOption::new(
            option::CoapOptionNumbers::Accept,
            &[],
        ))
        .unwrap(); // 17

        msg.add_option(option::CoapOption::new(
            option::CoapOptionNumbers::ProxyUri,
            b"ost",
        ))
        .unwrap(); // 35

        let ref_msg = msg.clone();
        let mut en_msg = msg.encode().unwrap();
        let buf = &mut en_msg.0[..en_msg.1];
        let de_msg = message::CoapMessage::decode(buf).unwrap();

        assert_eq!(de_msg, ref_msg);
    }
}
