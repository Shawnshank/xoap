/// Defines the header for a CoAP message

pub enum MessageType {
    None = -1,
    Confirmable = 0,
    NonConfirmable = 1,
    Acknowledgement = 2,
    Reset = 3,
}

pub struct Header {
    h_version: u8,
    h_type: MessageType,
    h_tkl: u8,
    h_code: u8,
    h_id: u16,
}

impl Header {
    pub fn new() -> Result<Self, ()> {
        let header = Header {
            h_version: 1,
            h_type: MessageType::None,
            h_tkl: 0,
            h_code: 0,
            h_id: 0,
        };
        Ok(header)
    }

    pub fn clear(&mut self) {
        self.h_version = 1;
        self.h_type = MessageType::None;
        self.h_tkl = 0;
        self.h_code = 0;
        self.h_id = 0;
    }

    pub fn get_version(&self) -> u8 {
        self.h_version
    }

    pub fn set_version(&mut self, ver: u8) {
        self.h_version = ver;
    }

    pub fn get_type(&self) -> MessageType {
        let t = match self.h_type {
            MessageType::None => MessageType::None,
            MessageType::Confirmable => MessageType::Confirmable,
            MessageType::NonConfirmable => MessageType::NonConfirmable,
            MessageType::Acknowledgement => MessageType::Acknowledgement,
            MessageType::Reset => MessageType::Reset,
        };
        t
    }

    pub fn set_type(&mut self, t: MessageType) {
        self.h_type = t;
    }

    pub fn get_tkl(&self) -> u8 {
        self.h_tkl
    }

    pub fn set_tkl(&mut self, t: u8) {
        self.h_tkl = t;
    }

    pub fn get_code(&self) -> u8 {
        self.h_code
    }

    pub fn set_code(&mut self, c: u8) {
        self.h_code = c;
    }

    pub fn get_id(&self) -> u16 {
        self.h_id
    }

    pub fn set_id(&mut self, id: u16) {
        self.h_id = id;
    }

    pub fn get_classcode(&self) -> u8 {
        self.h_code >> 5
    }

    pub fn set_classcode(&mut self, c: u8) {
        let detailcode = self.h_code & 0x1F;
        self.h_code = (c << 5) | detailcode;
    }

    pub fn get_detailcode(&self) -> u8 {
        self.h_code & 0x1F
    }

    pub fn set_detailcode(&mut self, c: u8) {
        let classcode = self.h_code & 0xE0;
        self.h_code = c | classcode;
    }

    pub fn type_to_number(&self, t: MessageType) -> u8 {
        let num = match t {
            MessageType::Confirmable => 0,
            MessageType::NonConfirmable => 1,
            MessageType::Acknowledgement => 2,
            MessageType::Reset => 3,
            _ => 100,
        };
        num
    }

    pub fn number_to_type(&self, n: u8) -> MessageType {
        let t = match n {
            0 => MessageType::Confirmable,
            1 => MessageType::NonConfirmable,
            2 => MessageType::Acknowledgement,
            3 => MessageType::Reset,
            _ => MessageType::None,
        };
        t
    }
}
