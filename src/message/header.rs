pub struct CoapHeader {
    version: u8,       // u2
    t: CoapHeaderType, // u2
    tkl: u8,           // u4
    code: CoapHeaderCode,
    message_id: u16,
}

#[derive(Debug, PartialEq)]
pub enum CoapHeaderType {
    Confirmable,
    NonConfirmable,
    Acknowledgement,
    Reset,
}

impl From<u8> for CoapHeaderType {
    fn from(item: u8) -> Self {
        match item {
            0 => CoapHeaderType::Confirmable,
            1 => CoapHeaderType::NonConfirmable,
            2 => CoapHeaderType::Acknowledgement,
            3 => CoapHeaderType::Reset,
            _ => unreachable!(),
        }
    }
}
impl From<CoapHeaderType> for u8 {
    fn from(item: CoapHeaderType) -> Self {
        match item {
            CoapHeaderType::Confirmable => 0,
            CoapHeaderType::NonConfirmable => 1,
            CoapHeaderType::Acknowledgement => 2,
            CoapHeaderType::Reset => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CoapHeaderCode {
    Created,
    Deleted,
    Valid,
    Changed,
    Content,
    BadRequest,
    Unauthorized,
    BadOption,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    PreconditionFailed,
    RequestEntityTooLarge,
    UnsupportedContentFormat,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    ProxyingNotSupported,
}

impl From<u8> for CoapHeaderCode {
    fn from(item: u8) -> Self {
        // c.dd -> c*32+d = decimal
        match item {
            65 => CoapHeaderCode::Created,
            66 => CoapHeaderCode::Deleted,
            67 => CoapHeaderCode::Valid,
            68 => CoapHeaderCode::Changed,
            69 => CoapHeaderCode::Content,
            128 => CoapHeaderCode::BadRequest,
            129 => CoapHeaderCode::Unauthorized,
            130 => CoapHeaderCode::BadOption,
            131 => CoapHeaderCode::Forbidden,
            132 => CoapHeaderCode::NotFound,
            133 => CoapHeaderCode::MethodNotAllowed,
            134 => CoapHeaderCode::NotAcceptable,
            140 => CoapHeaderCode::PreconditionFailed,
            141 => CoapHeaderCode::RequestEntityTooLarge,
            143 => CoapHeaderCode::UnsupportedContentFormat,
            160 => CoapHeaderCode::InternalServerError,
            161 => CoapHeaderCode::NotImplemented,
            162 => CoapHeaderCode::BadGateway,
            163 => CoapHeaderCode::ServiceUnavailable,
            164 => CoapHeaderCode::GatewayTimeout,
            165 => CoapHeaderCode::ProxyingNotSupported,
            _ => unreachable!(),
        }
    }
}
impl From<CoapHeaderCode> for u8 {
    fn from(item: CoapHeaderCode) -> Self {
        match item {
            CoapHeaderCode::Created => 65,
            CoapHeaderCode::Deleted => 66,
            CoapHeaderCode::Valid => 67,
            CoapHeaderCode::Changed => 68,
            CoapHeaderCode::Content => 69,
            CoapHeaderCode::BadRequest => 128,
            CoapHeaderCode::Unauthorized => 129,
            CoapHeaderCode::BadOption => 130,
            CoapHeaderCode::Forbidden => 131,
            CoapHeaderCode::NotFound => 132,
            CoapHeaderCode::MethodNotAllowed => 133,
            CoapHeaderCode::NotAcceptable => 134,
            CoapHeaderCode::PreconditionFailed => 140,
            CoapHeaderCode::RequestEntityTooLarge => 141,
            CoapHeaderCode::UnsupportedContentFormat => 143,
            CoapHeaderCode::InternalServerError => 160,
            CoapHeaderCode::NotImplemented => 161,
            CoapHeaderCode::BadGateway => 162,
            CoapHeaderCode::ServiceUnavailable => 163,
            CoapHeaderCode::GatewayTimeout => 164,
            CoapHeaderCode::ProxyingNotSupported => 165,
        }
    }
}

impl CoapHeader {
    pub fn new(t: CoapHeaderType, tkl: u8, code: CoapHeaderCode, message_id: u16) -> Self {
        CoapHeader {
            version: 0x1,
            t,
            tkl,
            code,
            message_id,
        }
    }
    pub fn encode(self) -> [u8; 4] {
        let t: u8 = self.t.into();
        let vtt: u8 = (self.version << 6) | (t << 4) | self.tkl;
        let code: u8 = self.code.into();
        let msg_2: u8 = (self.message_id & 255) as u8;
        let msg_1: u8 = (self.message_id >> 8) as u8;

        [vtt, code, msg_1, msg_2]
    }
    pub fn decode(buf: &[u8]) -> CoapHeader {
        let version: u8 = buf[0] >> 6;
        let t: CoapHeaderType = ((buf[0] << 2) >> 6).into();
        let tkl: u8 = buf[0] & 15;
        let code: CoapHeaderCode = buf[1].into();
        let message_id: u16 = (buf[2] as u16) << 8 | buf[3] as u16;

        CoapHeader {
            version,
            t,
            tkl,
            code,
            message_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::header::*;
    #[test]
    fn encode_decode() {
        let header = CoapHeader::new(
            CoapHeaderType::Acknowledgement,
            3,
            CoapHeaderCode::Changed,
            123,
        );
        let en_header = header.encode();
        let de_header = CoapHeader::decode(&en_header);

        assert_eq!(de_header.version, 1);
        assert_eq!(de_header.t, CoapHeaderType::Acknowledgement);
        assert_eq!(de_header.tkl, 3);
        assert_eq!(de_header.code, CoapHeaderCode::Changed);
        assert_eq!(de_header.message_id, 123);
    }
}
