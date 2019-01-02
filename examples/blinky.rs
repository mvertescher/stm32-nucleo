//! Blinks the user LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32_nucleo::{
    hal::{prelude::*, stm32},
    led::Led,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let gpioa = p.GPIOA.split();
    let mut led = Led::new(gpioa);

    loop {
        for _ in 0..10_000 {
            led.on();
        }
        for _ in 0..10_000 {
            led.off();
        }
    }
}
