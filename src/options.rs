use heapless::consts::*;
use heapless::Vec;

pub const MAX_VALUE_LENGTH: usize = 255;
pub const MAX_OPTIONS: usize = 10;

pub struct CoapOption {
    opt: CoapOptionsDef,
    //format: coap_format,
    //format: ValueFormat,
    value: Vec<u8, U255>,
}

impl CoapOption {
    pub fn new() -> Self {
        CoapOption {
            opt: CoapOptionsDef::None,
            //format: coap_format::new(),
            //format: ValueFormat::ValueEmpty,
            value: Vec::<u8, U255>::new(),
        }
    }
    pub fn get_option_type(&self) -> CoapOptionsDef {
        match self.opt {
            CoapOptionsDef::None => CoapOptionsDef::None,
            CoapOptionsDef::IfMatch => CoapOptionsDef::IfMatch,
            CoapOptionsDef::URIHost => CoapOptionsDef::URIHost,
            CoapOptionsDef::ETag => CoapOptionsDef::ETag,
            CoapOptionsDef::IfNoneMatch => CoapOptionsDef::IfNoneMatch,
            CoapOptionsDef::URIPort => CoapOptionsDef::URIPort,
            CoapOptionsDef::LocationPath => CoapOptionsDef::LocationPath,
            CoapOptionsDef::URIPath => CoapOptionsDef::URIPath,
            CoapOptionsDef::ContentFormat => CoapOptionsDef::ContentFormat,
            CoapOptionsDef::MaxAge => CoapOptionsDef::MaxAge,
            CoapOptionsDef::URIQuery => CoapOptionsDef::URIQuery,
            CoapOptionsDef::Accept => CoapOptionsDef::Accept,
            CoapOptionsDef::LocationQuery => CoapOptionsDef::LocationQuery,
            CoapOptionsDef::Block2 => CoapOptionsDef::Block2,
            CoapOptionsDef::Block1 => CoapOptionsDef::Block1,
            CoapOptionsDef::Size2 => CoapOptionsDef::Size2,
            CoapOptionsDef::ProxyURI => CoapOptionsDef::ProxyURI,
            CoapOptionsDef::ProxyScheme => CoapOptionsDef::ProxyScheme,
            CoapOptionsDef::Size1 => CoapOptionsDef::Size1,
        }
    }

    pub fn set_option_type(&mut self, t: CoapOptionsDef) {
        self.opt = t;
    }

    /*     pub fn get_option_format(&self) -> ValueFormat {
        let f = match self.format {
            ValueFormat::ValueEmpty => ValueFormat::ValueEmpty,
            ValueFormat::ValueOpaque => ValueFormat::ValueOpaque,
            ValueFormat::ValueUint => ValueFormat::ValueUint,
            ValueFormat::ValueString => ValueFormat::ValueString,
            ValueFormat::ValueBlock => ValueFormat::ValueBlock,
        };
        f
    }
    
    pub fn set_option_format(&mut self, f: ValueFormat) {
        self.format = f;
    } */

    pub fn get_option_value(&mut self) -> &mut Vec<u8, U255> {
        &mut self.value
    }

    pub fn set_option_value(&mut self, v: Vec<u8, U255>) {
        self.value = v;
    }
}

/*
+-----+---+---+---+---+----------------+--------+--------+-------------+
| No. | C | U | N | R | Name           | Format | Length | Default     |
+-----+---+---+---+---+----------------+--------+--------+-------------+
| 1   | x |   |   | x | If-Match       | opaque | 0-8    | (none)      |
| 3   | x | x | - |   | Uri-Host       | string | 1-255  | (see note 1)|
| 4   |   |   |   | x | ETag           | opaque | 1-8    | (none)      |
| 5   | x |   |   |   | If-None-Match  | empty  | 0      | (none)      |
| 7   | x | x | - |   | Uri-Port       | uint   | 0-2    | (see note 1)|
| 8   |   |   |   | x | Location-Path  | string | 0-255  | (none)      |
| 11  | x | x | - | x | Uri-Path       | string | 0-255  | (none)      |
| 12  |   |   |   |   | Content-Format | uint   | 0-2    | (none)      |
| 14  |   | x | - |   | Max-Age        | uint   | 0-4    | 60          |
| 15  | x | x | - | x | Uri-Query      | string | 0-255  | (none)      |
| 17  | x |   |   |   | Accept         | uint   | 0-2    | (none)      |
| 20  |   |   |   | x | Location-Query | string | 0-255  | (none)      |
| 28  |   |   | x |   | Size2          | uint   | 0-4    | (none)      |
| 35  | x | x | - |   | Proxy-Uri      | string | 1-1034 | (none)      |
| 39  | x | x | - |   | Proxy-Scheme   | string | 1-255  | (none)      |
| 60  |   |   | x |   | Size1          | uint   | 0-4    | (none)      |
+-----+---+---+---+---+----------------+--------+--------+-------------+
C = Critical, U = Unsafe, N = No-cache-Key, R = Repeatable,
*/

/* pub enum ValueFormat {
    ValueEmpty,  // A zero-length sequence of bytes.
    ValueOpaque, // An opaque sequence of bytes.
    ValueUint, // A non-negative integer that is represented in network byte order using the number of bytes given by the Option Length field.
    ValueString, // A Unicode string that is encoded using UTF-8 [RFC3629] in Net-Unicode form [RFC5198].
    ValueBlock,
}

pub fn format_to_number(fmt: ValueFormat) -> u16 {
    let num = match fmt {
        ValueFormat::ValueEmpty => 0,
        ValueFormat::ValueOpaque => 1,
        ValueFormat::ValueUint => 2,
        ValueFormat::ValueString => 3,
        ValueFormat::ValueBlock => 4,
    };
    num
}

pub fn number_to_format(num: u16) -> ValueFormat {
    let fmt = match num {
        0 => ValueFormat::ValueEmpty,
        1 => ValueFormat::ValueOpaque,
        2 => ValueFormat::ValueUint,
        3 => ValueFormat::ValueString,
        4 => ValueFormat::ValueBlock,
    };
    fmt
} */

pub enum CoapOptionsDef {
    None, // Internal
    IfMatch,
    URIHost,
    ETag,
    IfNoneMatch,
    URIPort,
    LocationPath,
    URIPath,
    ContentFormat,
    MaxAge,
    URIQuery,
    Accept,
    LocationQuery,
    Block2,
    Block1,
    Size2,
    ProxyURI,
    ProxyScheme,
    Size1,
}

pub fn option_to_number(opt: &CoapOptionsDef) -> u16 {
    match *opt {
        CoapOptionsDef::None => 0,
        CoapOptionsDef::IfMatch => 1,
        CoapOptionsDef::URIHost => 3,
        CoapOptionsDef::ETag => 4,
        CoapOptionsDef::IfNoneMatch => 5,
        CoapOptionsDef::URIPort => 7,
        CoapOptionsDef::LocationPath => 8,
        CoapOptionsDef::URIPath => 11,
        CoapOptionsDef::ContentFormat => 12,
        CoapOptionsDef::MaxAge => 14,
        CoapOptionsDef::URIQuery => 15,
        CoapOptionsDef::Accept => 17,
        CoapOptionsDef::LocationQuery => 20,
        CoapOptionsDef::Block2 => 23,
        CoapOptionsDef::Block1 => 27,
        CoapOptionsDef::Size2 => 28,
        CoapOptionsDef::ProxyURI => 35,
        CoapOptionsDef::ProxyScheme => 39,
        CoapOptionsDef::Size1 => 60,
    }
}

pub fn number_to_option(num: &u16) -> CoapOptionsDef {
    match *num {
        1 => CoapOptionsDef::IfMatch,
        3 => CoapOptionsDef::URIHost,
        4 => CoapOptionsDef::ETag,
        5 => CoapOptionsDef::IfNoneMatch,
        7 => CoapOptionsDef::URIPort,
        8 => CoapOptionsDef::LocationPath,
        11 => CoapOptionsDef::URIPath,
        12 => CoapOptionsDef::ContentFormat,
        14 => CoapOptionsDef::MaxAge,
        15 => CoapOptionsDef::URIQuery,
        17 => CoapOptionsDef::Accept,
        20 => CoapOptionsDef::LocationQuery,
        23 => CoapOptionsDef::Block2,
        27 => CoapOptionsDef::Block1,
        28 => CoapOptionsDef::Size2,
        35 => CoapOptionsDef::ProxyURI,
        39 => CoapOptionsDef::ProxyScheme,
        60 => CoapOptionsDef::Size1,
        _ => CoapOptionsDef::None,
    }
}
