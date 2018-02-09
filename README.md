# Embedded rust with CoAP
This project aims to make [CoAP](https://tools.ietf.org/html/rfc7252) on ARM embedded systems coded with [Rust](https://www.rust-lang.org) a thing.

This project is part of the course D7018E at [Luleå tekniska universitet](https://www.ltu.se) taught by [Per Lindgren](https://www.ltu.se/staff/p/pln-1.11258).

## Installation

## Crates used
- [coap](http://covertness.github.io/coap-rs/coap/index.html) 
- [smoltcp](https://crates.io/crates/smoltcp)


## ESP8266
As a WiFi bridge the ESP8266 is used and communicates with the CoAP server/client over USART using [SLIP](https://tools.ietf.org/html/rfc1055)
### Firmware
The firmware used for the esp8266 module is [esp-just-slip](https://github.com/krzychb/esp-just-slip)
