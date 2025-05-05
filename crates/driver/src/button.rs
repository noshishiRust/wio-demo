use wio_terminal as wio;

use wio::hal::gpio::{Disabled, Floating, Input, Pin, PC26};
use wio::prelude::*;

pub struct Button1 {
    pin: Pin<PC26, Input<Floating>>,
}

impl Button1 {
    pub fn new(pin: Pin<PC26, Disabled<Floating>>) -> Self {
        Self {
            pin: pin.into_floating_input(),
        }
    }

    pub fn is_pressed(&mut self) -> bool {
        self.pin.is_low().unwrap()
    }

    pub fn is_releaseed(&mut self) -> bool {
        self.pin.is_high().unwrap()
    }
}
