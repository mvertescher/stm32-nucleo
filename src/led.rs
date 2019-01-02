//! On-board user LEDs

use crate::hal::prelude::*;

use crate::hal::gpio::gpioa::{self, PA, PA5};
use crate::hal::gpio::{Output, PushPull};

/// The lone user LED
pub type LD2 = PA5<Output<PushPull>>;

/// One of the on-board user LEDs
pub struct Led {
    pax: PA<Output<PushPull>>,
}

impl Into<Led> for LD2 {
    fn into(self) -> Led {
        Led {
            pax: self.downgrade(),
        }
    }
}

impl Led {
    /// Initialize the single user LED
    pub fn new(gpioa: gpioa::Parts) -> Self {
        gpioa.pa5.into_push_pull_output().into()
    }
}

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pax.set_low()
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pax.set_high()
    }
}
