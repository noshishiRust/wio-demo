#![no_std]

use wio_terminal as wio;
use wio::hal::gpio::*;
use wio::prelude::*;

pub struct Led {
    pin: Pin<PA15, Output<PushPull>>,
}

impl Led {
    pub fn new(pin: Pin<PA15, Disabled<Floating>>) -> Self {
        Self { pin: pin.into_push_pull_output() }
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

pub struct Button1 {
    pin: Pin<PC26, Input<Floating>>,
}

impl Button1 {
    pub fn new(pin: Pin<PC26, Disabled<Floating>>) -> Self {
        Self { pin: pin.into_floating_input() }
    }

    pub fn is_pressed(&mut self) -> bool {
        self.pin.is_low().unwrap()
    }

    pub fn is_releaseed(&mut self) -> bool {
        self.pin.is_high().unwrap()
    }
}
