pub enum ResponseType {
    Empty,      // Internal
    Reserved,   // Internal

    // Success 2.xx
    Created,                    // 2.01
    Deleted,                    // 2.02
    Valid,                      // 2.03
    Changed,                    // 2.04
    Content,                    // 2.05
    
    // Client error 4.xx
    BadRequest,                 // 4.00
    Unauthorized,               // 4.01
    BadOption,                  // 4.02
    Forbidden,                  // 4.03
    NotFound,                   // 4.04
    MethodNotAllowed,           // 4.05
    NotAcceptable,              // 4.06
    PreconditionFailed,         // 4.12
    RequestEntityTooLarge,      // 4.13
    UnsupportedContentFormat,   // 4.15
    
    // Server Error 5.xx
    InternalServerError,        // 5.00
    NotImplemented,             // 5.01
    BadGateway,                 // 5.02
    ServiceUnavailable,         // 5.03
    GatewayTimeout,             // 5.04
    ProxyingNotSupported,       // 5.05  
}

pub struct Response {

}

pub fn response_to_number(opt: &ResponseType) -> u8 {
    match *opt {
        ResponseType::Unknown => 0x00,

        ResponseType::Created => 0x41,
        ResponseType::Deleted => 0x42,
        ResponseType::Valid => 0x43,
        ResponseType::Changed => 0x44,
        ResponseType::Content => 0x45,

        ResponseType::BadRequest => 0x80,
        ResponseType::Unauthorized => 0x81,
        ResponseType::BadOption => 0x82,
        ResponseType::Forbidden => 0x83,
        ResponseType::NotFound => 0x84,
        ResponseType::MethodNotAllowed => 0x85,
        ResponseType::NotAcceptable => 0x86,
        ResponseType::PreconditionFailed => 0x8C,
        ResponseType::RequestEntityTooLarge => 0x8D,
        ResponseType::UnsupportedContentFormat => 0x8F,

        ResponseType::InternalServerError => 0x90,
        ResponseType::NotImplemented => 0x91,
        ResponseType::BadGateway => 0x92,
        ResponseType::ServiceUnavailable => 0x93,
        ResponseType::GatewayTimeout => 0x94,
        ResponseType::ProxyingNotSupported => 0x95,
        
        _ => 0xFF,
    } as u8;
}

pub fn number_to_response(num: &u8) -> CoapOptionsDef {
    match *num {
        0x00 => MessageClass::Unknown,

        0x01 => RequestType::Get,
        0x02 => RequestType::Post,
        0x03 => RequestType::Put,
        0x04 => RequestType::Delete,

        0x41 => ResponseType::Created,
        0x42 => ResponseType::Deleted,
        0x43 => ResponseType::Valid,
        0x44 => ResponseType::Changed,
        0x45 => ResponseType::Content,

        0x80 => ResponseType::BadRequest,
        0x81 => ResponseType::Unauthorized,
        0x82 => ResponseType::BadOption,
        0x83 => ResponseType::Forbidden,
        0x84 => ResponseType::NotFound,
        0x85 => ResponseType::MethodNotAllowed,
        0x86 => ResponseType::NotAcceptable,
        0x8C => ResponseType::PreconditionFailed,
        0x8D => ResponseType::RequestEntityTooLarge,
        0x8F => ResponseType::UnsupportedContentFormat,

        0x90 => ResponseType::InternalServerError,
        0x91 => ResponseType::NotImplemented,
        0x92 => ResponseType::BadGateway,
        0x93 => ResponseType::ServiceUnavailable,
        0x94 => ResponseType::GatewayTimeout,
        0x95 => ResponseType::ProxyingNotSupported,
    }
}

