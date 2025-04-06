#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;
use wio::entry;
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = wio::Pins::new(peripherals.PORT).split();

    let mut user_led = pins.user_led.into_push_pull_output();
    let button1 = pins.buttons.button1.into_floating_input();


    loop {
        if button1.is_high().unwrap() {
            user_led.set_high().unwrap();
        } else {
            user_led.set_low().unwrap();
        }
    }
}
