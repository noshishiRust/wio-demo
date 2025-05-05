use wio_terminal as wio;

use wio::hal::gpio::{Disabled, Floating, Output, Pin, PushPull, PA15};
use wio::prelude::*;

pub struct Led {
    pin: Pin<PA15, Output<PushPull>>,
}

impl Led {
    pub fn new(pin: Pin<PA15, Disabled<Floating>>) -> Self {
        Self {
            pin: pin.into_push_pull_output(),
        }
    }

    pub fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    pub fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }

    pub fn toggle(&mut self) {
        self.pin.toggle().unwrap();
    }
}
