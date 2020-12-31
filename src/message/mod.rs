use heapless::Vec;
use heapless::consts::*;

mod header;
mod option;

pub struct CoapMessage {
    header:  header::CoapHeader,
    token: Vec::<u8, U8>,
    options: Vec::<option::CoapOption, U10>,
    payload_marker: u8,
    payload: [u8],
}

