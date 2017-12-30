// Nucleo 
// D7018E - Embedded rust
// Joakim Lundberg <joakim@joakimlundberg.com>

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;

fn main() {
    // get a handle to the *host* standard output
    let mut stdout = hio::hstdout().unwrap();

    for n in 1..101 {
        if n % 15 == 0 {
            writeln!(stdout,"fizzbuzz");
        } else if n % 3 == 0 {
            writeln!(stdout,"fizz");
        } else if n % 5 == 0 {
            writeln!(stdout,"buzz");
        } else {
            writeln!(stdout,"{}", n);
        }
    }

    // write "Hello, world!" to it
    writeln!(stdout, "Hello, world!").unwrap();
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
