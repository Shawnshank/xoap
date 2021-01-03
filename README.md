# XoAP
Rust implementation of the [CoAP protocol](https://tools.ietf.org/html/rfc7252) for embedded systems.

Note: This library is NOT a full protocol implementation.

## Features
* `#![no_std]`
* Does not require a allocator.
* Focus on easy of use

### Current status

## Restrictions
* Only supports Piggybacked responses
* Only one URI-Path per request
