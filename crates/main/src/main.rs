#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;
use wio::entry;
use wio::pac::Peripherals;

use driver::{Button1, Led};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = wio::Pins::new(peripherals.PORT).split();

    let mut led = Led::new(pins.user_led);
    let mut button1 = Button1::new(pins.buttons.button1);

    loop {
        if button1.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}
