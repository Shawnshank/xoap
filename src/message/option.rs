use crate::CoapError;
use heapless::consts::*;
use heapless::Vec;

#[derive(Clone)]
pub struct CoapOptions {
    options: Vec<CoapOption, U10>,
    length: usize,
}

impl CoapOptions {
    pub fn new() -> Self {
        CoapOptions {
            options: Vec::<CoapOption, U10>::new(),
            length: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn push(&mut self, option: CoapOption) -> Result<(), CoapError> {
        match self.options.push(option) {
            Ok(_) => Ok(()),
            Err(_) => Err(CoapError::MessageError),
        }
    }
    pub fn pop(&mut self) -> Result<CoapOption, CoapError> {
        match self.options.pop() {
            Some(e) => Ok(e),
            _ => Err(CoapError::MessageError),
        }
    }
}

#[derive(Clone)]
pub struct CoapOption {
    option: CoapOptionNumbers,
    data: Vec<u8, U255>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CoapOptionNumbers {
    Zero,
    IfMatch,
    UriHost,
    ETag,
    IfNoneMatch,
    UriPort,
    LocationPath,
    UriPath,
    ContentFormat,
    MaxAge,
    UriQuery,
    Accept,
    LocationQuery,
    ProxyUri,
    ProxyScheme,
    Size1,
}

impl From<u8> for CoapOptionNumbers {
    fn from(item: u8) -> Self {
        match item {
            0 => CoapOptionNumbers::Zero,
            1 => CoapOptionNumbers::IfMatch,
            3 => CoapOptionNumbers::UriHost,
            4 => CoapOptionNumbers::ETag,
            5 => CoapOptionNumbers::IfNoneMatch,
            7 => CoapOptionNumbers::UriPort,
            8 => CoapOptionNumbers::LocationPath,
            11 => CoapOptionNumbers::UriPath,
            12 => CoapOptionNumbers::ContentFormat,
            14 => CoapOptionNumbers::MaxAge,
            15 => CoapOptionNumbers::UriQuery,
            17 => CoapOptionNumbers::Accept,
            20 => CoapOptionNumbers::LocationQuery,
            35 => CoapOptionNumbers::ProxyUri,
            39 => CoapOptionNumbers::ProxyScheme,
            60 => CoapOptionNumbers::Size1,
            _ => unreachable!(), // TODO: Handle Reserved cases
        }
    }
}
impl From<CoapOptionNumbers> for u8 {
    fn from(item: CoapOptionNumbers) -> Self {
        match item {
            CoapOptionNumbers::Zero => 0,
            CoapOptionNumbers::IfMatch => 1,
            CoapOptionNumbers::UriHost => 3,
            CoapOptionNumbers::ETag => 4,
            CoapOptionNumbers::IfNoneMatch => 5,
            CoapOptionNumbers::UriPort => 7,
            CoapOptionNumbers::LocationPath => 8,
            CoapOptionNumbers::UriPath => 11,
            CoapOptionNumbers::ContentFormat => 12,
            CoapOptionNumbers::MaxAge => 14,
            CoapOptionNumbers::UriQuery => 15,
            CoapOptionNumbers::Accept => 17,
            CoapOptionNumbers::LocationQuery => 20,
            CoapOptionNumbers::ProxyUri => 35,
            CoapOptionNumbers::ProxyScheme => 39,
            CoapOptionNumbers::Size1 => 60,
        }
    }
}

impl CoapOption {
    pub fn new(option: CoapOptionNumbers, data: &[u8]) -> Self {
        let mut d: Vec<u8, U255> = Vec::new();
        d.extend_from_slice(data);
        CoapOption { option, data: d }
    }

    pub fn get_option_number(&self) ->  CoapOptionNumbers {
        self.option
    }
    pub fn encode(&self, prev_option: CoapOptionNumbers) -> Result<([u8; 255], usize), CoapError> {
        let mut v: [u8; 255] = [0; 255];
        let o: u8 = self.option.into();
        let po: u8 = prev_option.into();
        // Check so that we are encoding the options in order
        if po > o {
            return Err(CoapError::OptionError);
        }
        let option_delta = o - po;
        let option_length = self.data.len() as u8;
        // TODO: make the correct assumtion regarding available length, not just 254 bytes
        if option_length > 254 {
            return Err(CoapError::OptionError);
        }
        let mut byte_offset = 0;
        match option_delta {
            0..12 => v[0] = option_delta << 4,
            13..255 => {
                v[0] = 13 << 4;
                v[1] = option_delta - 13;
                byte_offset += 1;
            }
            _ => return Err(CoapError::OptionError),
        }

        match option_length {
            0..12 => v[0] = v[0] | option_length,
            13..255 => {
                v[0] = v[0] | 13;
                v[1] = option_length - 13;
                byte_offset += 1;
            }
            _ => return Err(CoapError::OptionError),
        }
        let mut index = 1 + byte_offset;
        for i in &self.data {
            v[index] = *i;
            index = index + 1;
        }
        let length = self.data.len() + 1 + byte_offset; // length of data + option length
        Ok((v, length))
    }

    pub fn decode(prev_option_number: u8, buf: &[u8]) -> Result<CoapOption, CoapError> {
        let d = buf[0] >> 4;
        let mut byte_offset = 0;
        let delta: u8 = match d {
            0..12 => d as u8,
            13 => {
                byte_offset = byte_offset + 1;
                buf[1] as u8 + 13
            }
            //14 => ((buf[1] as u16) << 8 | buf[2] as u16) + 269, // Will never be used as we can not handle higher numbers then 60
            _ => return Err(CoapError::OptionError),
        };

        let option: CoapOptionNumbers = (prev_option_number + delta).into();

        let l = buf[0] & 15;
        let length: u16 = match l {
            0..12 => l as u16,
            13 => {
                let len = buf[1 + byte_offset] as u16 + 13;
                byte_offset = byte_offset + 1;
                len
            }
            14 => {
                let len = ((buf[1 + byte_offset] as u16) << 8 | buf[2 + byte_offset] as u16) + 269;
                byte_offset = byte_offset + 2;
                len
            }
            _ => return Err(CoapError::OptionError),
        };

        let mut data = Vec::<u8, U255>::new();

        //assert_eq!(length as usize, buf.len() - byte_offset - 1);

        for i in 1..(length as usize + 1) {
            data.push(buf[i]).unwrap();
        }

        Ok(CoapOption { option, data })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::option::*;

    #[test]
    fn decode() {
        let option = CoapOption::decode(0, &[((3 << 4) | 5), 10, 11, 12, 13, 14]).unwrap();
        assert_eq!(option.option, CoapOptionNumbers::UriHost);
        assert_eq!(option.data[0], 10);
        assert_eq!(option.data[1], 11);
        assert_eq!(option.data[2], 12);
        assert_eq!(option.data[3], 13);
        assert_eq!(option.data[4], 14);
    }
    #[test]
    fn decode_previous_option() {
        let option = CoapOption::decode(3, &[((2 << 4) | 2), 10, 11]).unwrap();
        assert_eq!(option.option, CoapOptionNumbers::IfNoneMatch);
        assert_eq!(option.data[0], 10);
        //assert_eq!(option.data[2], 11);
    }

    #[test]
    fn encode() {
        let data = [1, 2, 3, 4, 5];
        let option = CoapOption::new(CoapOptionNumbers::UriHost, &data)
            .encode(CoapOptionNumbers::Zero)
            .unwrap();
        assert_eq!(
            CoapOptionNumbers::from(option.0[0] >> 4),
            CoapOptionNumbers::UriHost
        );
        assert_eq!(option.1, 1 + 5);
        assert_eq!(option.0[1], 1);
        assert_eq!(option.0[2], 2);
        assert_eq!(option.0[3], 3);
        assert_eq!(option.0[4], 4);
        assert_eq!(option.0[5], 5);
    }
    #[test]
    fn encode_previous_option() {
        let data = [1, 2, 3, 4, 5];
        let option = CoapOption::new(CoapOptionNumbers::ETag, &data)
            .encode(CoapOptionNumbers::UriHost)
            .unwrap();
        assert_eq!(
            CoapOptionNumbers::from(option.0[0] >> 4),
            CoapOptionNumbers::IfMatch
        );
        assert_eq!(option.1, 1 + 5);
        assert_eq!(option.0[1], 1);
        assert_eq!(option.0[2], 2);
        assert_eq!(option.0[3], 3);
        assert_eq!(option.0[4], 4);
        assert_eq!(option.0[5], 5);
    }
    #[test]
    fn encode_decode_option() {
        let data = [1, 2, 3, 4, 5];
        let vec_data: Vec<u8, U5> = Vec::from_slice(&data).unwrap();
        let en_option = CoapOption::new(CoapOptionNumbers::UriHost, &data)
            .encode(CoapOptionNumbers::Zero)
            .unwrap();
        let de_option = CoapOption::decode(0, &en_option.0).unwrap();

        assert_eq!(de_option.option, CoapOptionNumbers::UriHost);
        assert_eq!(de_option.data, vec_data);
    }
    #[test]
    fn encode_decode_previous_option() {
        let data = [1, 2, 3, 4, 5];
        let vec_data: Vec<u8, U5> = Vec::from_slice(&data).unwrap();
        let en_option = CoapOption::new(CoapOptionNumbers::UriHost, &data)
            .encode(CoapOptionNumbers::IfMatch)
            .unwrap();
        let de_option =
            CoapOption::decode(u8::from(CoapOptionNumbers::IfMatch), &en_option.0).unwrap();

        assert_eq!(de_option.option, CoapOptionNumbers::UriHost);
        assert_eq!(de_option.data, vec_data);
    }
}
