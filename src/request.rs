use packet::Packet;


pub struct CoAPRequest {
    message: Packet,
    response: Option<CoAPResponse>,
    source: Option<SocketAddr>,
}

impl CoAPRequest {
    pub fn new() -> CoAPRequest {
        CoAPRequest {
            response: None,
            message: Packet::new(),
            source: None,
        }
    }

    pub fn from_packet(packet: Packet, source: &SocketAddr) -> CoAPRequest {
        CoAPRequest {
            response: CoAPResponse::new(&packet),
            message: packet,
            source: Some(source.clone()),
        }
    }

    pub fn set_method(&mut self, method: Method) {
        self.message.header.code = MessageClass::Request(method);
    }
}


pub enum Method {
    Unknown, // Internal
    GET,    // 0.01
    POST,   // 0.02
    PUT,    // 0.03
    DELETE, // 0.04
}

pub fn method_to_number(opt: &Method) -> u8 {
    match *opt {
        Method::Unknown => 0x00,

        Method::GET => 0x01,
        Method::POST => 0x02,
        Method::PUT => 0x03,
        Method::DELETE => 0x04,
        
        _ => 0xFF,
    } as u8;
}

pub fn method_to_number(opt: &Method) -> u8 {
    match *opt {
        0x00 => Method::Unknown,

        0x01 => Method::GET,
        0x02 => Method::POST ,
        0x03 => Method::PUT ,
        0x04 => Method::DELETE,
        
        _ => Method::Unknown,
    } as u8;
}