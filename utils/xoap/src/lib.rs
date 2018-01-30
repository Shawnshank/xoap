//! Implementation of the [CoAP Protocol][spec].
//!
//! This library provides both a client interface (`CoAPClient`)
//!   and a server interface (`CoAPServer`).
//!
//! [spec]: https://tools.ietf.org/html/rfc7252
//!

#![deny(missing_docs)]
#![deny(warnings)]
#[allow(deprecated)]
#![feature(collections)]
#![no_std]
#![allow(unused_mut)]

//#[macro_use]
extern crate smoltcp;
extern crate nb;
extern crate cast;
extern crate alloc_cortex_m;
#[macro_use]
extern crate collections;

