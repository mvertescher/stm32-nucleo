//! Board support crate for STM32 Nucleo devices

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

#[cfg(stm32f0xx)]
pub extern crate stm32f0xx_hal as hal;

#[cfg(stm32f4xx)]
pub extern crate stm32f4xx_hal as hal;

#[cfg(stm32f7xx)]
pub extern crate stm32f7xx_hal as hal;

#[cfg(stm32l1xx)]
pub extern crate stm32l1xx_hal as hal;

#[cfg(stm32l4xx)]
pub extern crate stm32l4xx_hal as hal;

#[cfg(stm32f4xx)]
pub mod led;

/// Configure the default serial port with the default configuration.
///
/// The baud rate of the serial port should be 19200.
#[cfg(nucleo64)]
#[macro_export]
macro_rules! serial {
    ($p:ident, $clocks:ident) => {
        {
            let mut gpioa = $p.GPIOA.split();
            let tx = gpioa.pa2.into_alternate_af7();
            let rx = gpioa.pa3.into_alternate_af7();

            let config = stm32_nucleo::hal::serial::config::Config::default();
            stm32_nucleo::hal::serial::Serial::usart2($p.USART2, (tx, rx), config, $clocks)
                .unwrap()
        }
    };
}
