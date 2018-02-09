//! Implementation of the [CoAP Protocol][spec].
//!
//! This library provides both a client interface (`CoAPClient`)
//!   and a server interface (`CoAPServer`).
//!
//! [spec]: https://tools.ietf.org/html/rfc7252
//!

#![allow(missing_docs)]
#![deny(warnings)]

//#![cfg(features = "baremetal")]
#![no_std]

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(features = "baremetal")]
    {
        // ---- Bare metal ----
        extern crate cast;
        extern crate nb;
        //extern crate core as std;
        extern crate heapless;
    } else if #[cfg(features = "default")]{
        // ---- Standard ----
        extern crate bincode;
        extern crate rustc_serialize;
        extern crate mio;
        extern crate url;
        extern crate num;
        extern crate rand;
        extern crate threadpool;
        #[macro_use] extern crate enum_primitive;
        #[macro_use] extern crate log;

        #[cfg(test)]
        extern crate quickcheck;

        pub use client::CoAPClient;
        pub use message::header::MessageType;
        pub use message::IsMessage;
        pub use message::packet::CoAPOption;
        pub use message::request::CoAPRequest;
        pub use message::response::CoAPResponse;
        pub use server::CoAPServer;
    }
}


pub mod utils;
pub mod message;
pub mod client;
pub mod server;