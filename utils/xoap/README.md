# xoap
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A fast and stable [Constrained Application Protocol(CoAP)](https://tools.ietf.org/html/rfc7252) library implemented in Rust.

built from
[Documentation](http://covertness.github.io/coap-rs/coap/index.html)

## Installation

First add this to your `Cargo.toml`:

```toml
[dependencies]
xoap = "0.5"
```

Then, add this to your crate root:

```rust
extern crate xoap;
```

## Example

### Server:
```rust
extern crate xoap;

use std::io;
use xoap::{CoAPServer, CoAPResponse, CoAPRequest};

fn request_handler(req: CoAPRequest) -> Option<CoAPResponse> {
    println!("Receive request: {:?}", req);

    // Return the auto-generated response
    req.response
}

fn main() {
    let addr = "127.0.0.1:5683";

    let mut server = CoAPServer::new(addr).unwrap();
    server.handle(request_handler).unwrap();

    println!("Server up on {}", addr);
    println!("Press any key to stop...");
    io::stdin().read_line(&mut String::new()).unwrap();

    println!("Server shutdown");
}
```

### Client:
```rust
extern crate coap;

use coap::{CoAPClient, CoAPResponse};

fn main() {
    let url = "coap://127.0.0.1:5683/Rust";
    println!("Client request: {}", url);

    let response: CoAPResponse = CoAPClient::request(url).unwrap();
    println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());
}
```

## Benchmark
```bash
$ cargo run --example server
$ cargo bench
```
