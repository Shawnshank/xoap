extern crate xoap;

use xoap::{CoapConfig, CoapServer};

fn main() {
    let mut config = CoapConfig::new();
    config.add_resource(res_1, "res_1");
    config.add_resource(res_2, "res_2");
    config.add_resource(res_3, "res_3");
    config.add_resource(res_4, "res_4");

    let server = CoapServer::new(config);
    
    // Resource 1
    //let mut request_res = [66, 1, 0, 123, 100, 101, 181, 114, 101, 115, 95, 49, 255, 1, 2];

    // Resource 2
    let mut request_res = [66, 1, 0, 123, 100, 101, 181, 114, 101, 115, 95, 50, 255, 1, 2];

    // Resource 3
    //let mut request_res = [66, 1, 0, 123, 100, 101, 181, 114, 101, 115, 95, 51, 255, 1, 2];

    // Resource 4
    //let mut request_res = [66, 1, 0, 123, 100, 101, 181, 114, 101, 115, 95, 52, 255, 1, 2];

    let _response = server.handle_message(&mut request_res);
    //assert_eq!(response, request_res1);
}

fn res_1() -> u8 {
    println!("Accessing Resource 1");
    1
}

fn res_2() -> u8 {
    println!("Accessing Resource 2");
    2
}

fn res_3() -> u8 {
    println!("Accessing Resource 3");
    3
}

fn res_4() -> u8 {
    println!("Accessing Resource 4");
    4
}