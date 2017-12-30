// CoAP CLient
// D7018E - Embedded rust
// Joakim Lundberg <joakim@joakimlundberg.com>

// Externally used crates
extern crate coap;

use std::io::ErrorKind;
use coap::{CoAPClient, CoAPRequest, IsMessage, MessageType, CoAPOption};

fn main() {
    println!("Request by GET:");
    let addr = "127.0.0.1:5683";
    let endpoint = "test";

    let c = start_coap_client(addr); //CoAPClient::new(addr).unwrap();  
    coap_get(c, addr, endpoint);

    println!("Request by POST:");
    coap_post(addr, endpoint);
}

fn start_coap_client(address: &str) -> CoAPClient {
    // Creates the CoAP client for the specific IP address
    CoAPClient::new(address).unwrap()
}


fn coap_get(client: CoAPClient, address: &str, endpoint: &str) { 

    // Creates a request to be 
    let mut request = CoAPRequest::new();

    // Setup of the message header (4 bytes)
    request.set_version(1); 
    request.set_type(MessageType::Confirmable); // MessageType (Confirmable, Non-confirmable, Acknowledgement, Reset)
    request.set_code("0.01");  // Code -> Request code (1-10) or Response code (40-255)
    request.set_message_id(1); // 16-bit identifier for matching responses
    request.set_token(vec![0x51, 0x55, 0x77, 0xE8]); // optional response matching tokens
    request.add_option(CoAPOption::UriPath, endpoint.to_string().into_bytes()); // additional options. eg URI

    // Sends the request
    client.send(&request).unwrap();

    println!("Client request: coap://{}/{}", address, endpoint);

    match client.receive() {
        Ok(response) => {
            println!("Server reply: {}",
                     String::from_utf8(response.message.payload).unwrap());
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::WouldBlock => println!("Request timeout"),   // Unix
                ErrorKind::TimedOut => println!("Request timeout"),     // Windows
                _ => println!("Request error: {:?}", e),
            }
        }
    }
}

fn coap_post(address: &str, endpoint: &str) {

    let client = CoAPClient::new(address).unwrap();
    let mut request = CoAPRequest::new();
    request.set_version(1);
    request.set_type(MessageType::Confirmable);
    request.set_code("0.02");
    request.set_message_id(1);
    request.set_token(vec![0x51, 0x55, 0x77, 0xE8]);
    request.add_option(CoAPOption::UriPath, endpoint.to_string().into_bytes());
    request.set_payload(b"data".to_vec());

    client.send(&request).unwrap();
    println!("Client request: coap://{}/{}", address, endpoint);

    match client.receive() {
        Ok(response) => {
            println!("Server reply: {}",
                     String::from_utf8(response.message.payload).unwrap());
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::WouldBlock => println!("Request timeout"),   // Unix
                ErrorKind::TimedOut => println!("Request timeout"),     // Windows
                _ => println!("Request error: {:?}", e),
            }
        }
    }
}