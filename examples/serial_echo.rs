//! Serial interface echo server for all Nucleo-64 boards

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32_nucleo::hal::{prelude::*, stm32};
use nb::block;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let serial = stm32_nucleo::serial!(p, clocks);
    let (mut tx, mut rx) = serial.split();

    loop {
        let byte = block!(rx.read()).unwrap();
        block!(tx.write(byte)).ok();
    }
}
