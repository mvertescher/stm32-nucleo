//! Board support crate for STM32 Nucleo devices

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

#[cfg(stm32f4xx)]
pub extern crate stm32f4xx_hal as hal;

#[cfg(stm32f4xx)]
pub mod led;
