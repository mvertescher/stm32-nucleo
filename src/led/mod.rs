//! On-board user LEDs

use crate::hal::prelude::*;

use crate::hal::gpio::{Output, PushPull};

#[cfg(nucleo32)]
use crate::hal::gpio::gpiob::{self, PB3};

#[cfg(nucleo64)]
use crate::hal::gpio::gpioa::{self, PA5};

macro_rules! led {
    ($gpiox:ident, $pxx:ident, $PXX:ident, $LDX:ident) => {
        /// The lone user LED
        pub type $LDX = $PXX<Output<PushPull>>;

        /// One of the on-board user LEDs
        pub struct Led {
            pin: $LDX,
        }

        impl Into<Led> for $LDX {
            fn into(self) -> Led {
                Led {
                    pin: self,
                }
            }
        }

        impl Led {
            /// Initialize the single user LED
            pub fn new(gpio: $gpiox::Parts) -> Self {
                gpio.$pxx.into_push_pull_output().into()
            }
        }

        impl Led {
            /// Turns the LED off
            pub fn off(&mut self) {
                self.pin.set_low()
            }

            /// Turns the LED on
            pub fn on(&mut self) {
                self.pin.set_high()
            }
        }
    }
}

#[cfg(nucleo32)]
led!(gpiob, pb3, PB3, LD3);

#[cfg(nucleo64)]
led!(gpioa, pa5, PA5, LD2);
