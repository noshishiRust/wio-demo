#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK, // https://www.intel.co.jp/content/www/jp/ja/docs/programmable/683047/16-0/global-clock-control-block.html
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = wio::Pins::new(peripherals.PORT).split();

    let mut serials = pins.uart.init(
        &mut clocks,
        115200.Hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
    );

    for c in b"Hello, world!" {
        nb::block!(serials.write(*c)).unwrap();
    }

    loop {}
}
