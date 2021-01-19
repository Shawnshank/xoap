use heapless::consts::*;
use heapless::Vec;

use header;
use options;

const MAX_PACKET_SIZE: usize = 255; // Maximum amount of bytes in a singel packet

pub struct Packet {
    packet_raw: Vec<u8, U255>, // = Vec::new();
    header: header::Header,
    token: u64, // 0 - 8 bytes
    options_vec: Vec<options::CoapOption, U10>,
    payload: Vec<u8, U255>,
}

impl Packet {
    pub fn new() -> Result<Self, ()> {
        let packet = Packet {
            packet_raw: Vec::<u8, U255>::new(), // Link with MAX_PACKET_SIZE
            header: header::Header::new().unwrap(),
            token: 0,
            options_vec: Vec::<options::CoapOption, U10>::new(),
            payload: Vec::<u8, U255>::new(),
        };
        Ok(packet)
    }

    pub fn clear(&mut self) {
        self.packet_raw.clear();
        self.header.clear();
        self.token = 0;
        self.options_vec.clear();
        self.payload.clear();
    }

    pub fn decode_packet(&mut self, raw_message: &[u8], len: u8) {
        self.clear();
        //let message_length = raw_message.len();
        self.packet_raw.extend_from_slice(&raw_message).unwrap();
        self.packet_raw.truncate(len as usize);

        self.decode_header();
        self.decode_token();
        let pos = self.decode_options() as u8;
        self.decode_payload(pos);
    }

    // Decodes the COAP header from the raw message
    fn decode_header(&mut self) {
        // Get the coap version (should be 1)
        let ver: u8 = self.packet_raw[0] >> 6;
        // Get the coap type
        let t: header::MessageType = self.header.number_to_type((self.packet_raw[0] << 2) >> 6);
        // Get the coap token length
        let tkl: u8 = self.packet_raw[0] & 0xF;
        // Get the coap message ID
        let id: u16 = (self.packet_raw[2] as u16) << 8 | (self.packet_raw[3] as u16);

        // Stores all decoded info into packet header
        self.header.set_version(ver);
        self.header.set_type(t);
        self.header.set_tkl(tkl);
        self.header.set_code(self.packet_raw[1]);
        self.header.set_id(id);
    }

    // Decodes the COAP token from the raw message
    fn decode_token(&mut self) {
        // If the token length in the header is 0 then the token is 0,
        // else interate the amount of bytes to read out the token
        if self.header.get_tkl() == 0 {
            self.token = 0;
        } else {
            let token_length = 4 + self.header.get_tkl();
            //
            for i in 4..token_length {
                self.token = self.token << 8 | self.packet_raw[i as usize] as u64;
            }
        }
    }

    // Decodes the COAP options (if there are any) from the raw message
    fn decode_options(&mut self) -> usize {
        let mut delta: u16 = 0;
        // Gets the start position of the option block (fixed_header_size + token_length)
        let mut pos: usize = 4 + self.header.get_tkl() as usize;
        let start_pos: usize = 4 + self.header.get_tkl() as usize;
        // While the byte read is NOT the payload marker decode the options
        let mut opt_len: usize = 0;
        while self.packet_raw[pos] != 0xFF || pos >= self.packet_raw.len() {
            // Find delta

            let mut new_delta = (self.packet_raw[pos] >> 4) as u16;
            /* if ((pos > start_pos) && new_delta == 0) {
                break;
            } */

            let mut new_opt = options::CoapOption::new();
            let mut delta_ext: usize = 0;
            let mut length_ext: usize = 0;

            if new_delta == 13 {
                new_delta = new_delta + self.packet_raw[pos + 1] as u16; // - 13;
                delta_ext = 1;
            } else if new_delta == 14 {
                new_delta = new_delta
                    + (((self.packet_raw[pos + 1] as u16) << 8)
                        | (self.packet_raw[pos + 2] as u16))
                    - 269;
                delta_ext = 2;
            }
            delta = delta + new_delta;

            new_opt.set_option_type(options::number_to_option(&delta));

            // Find length
            let mut length: u16 = self.packet_raw[pos] as u16 & 0xF as u16;
            if length == 13 {
                length = self.packet_raw[pos + 1 + delta_ext] as u16 - 13;
                length = 1;
            } else if length == 14 {
                length = (((self.packet_raw[pos + 1 + delta_ext] as u16) << 8)
                    | (self.packet_raw[pos + 2 + delta_ext] as u16))
                    - 269;
                length_ext = 2;
            }
            // Adds length amount of bytes into the option value
            for j in 0..length {
                /* self.options_vec[opt_len]
                .get_option_value()
                .push(self.packet_raw[(j as usize) + pos + delta_ext + length_ext]); */
                new_opt
                    .get_option_value()
                    .push(self.packet_raw[(j as usize) + pos + delta_ext + length_ext]);
            }

            self.options_vec.push(new_opt);

            // Iterate to next options first byte until payload marker is found
            pos = pos + 1 + delta_ext + length_ext + length as usize;

            // Iterate over Options vector
            opt_len = opt_len + 1;
        }
        // Resizes the options vector to the same length as the amount of options found in the raw message
        self.options_vec.truncate(opt_len);

        pos
    }

    // Decodes the COAP payload (Message) from the raw message
    fn decode_payload(&mut self, start_pos: u8) {
        let packet_len = self.packet_raw.len() as u8;
        if start_pos <= packet_len {
            let mut pos = start_pos;
            while pos <= packet_len {
                self.payload.push(self.packet_raw[pos as usize]);
                pos = pos + 1;
            }
            self.payload.truncate(pos as usize - start_pos as usize);
        }
    }
}
