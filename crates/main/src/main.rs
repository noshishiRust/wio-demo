#![no_std]
#![no_main]

use wio_terminal as wio;

use cortex_m::asm;
use driver::button::Button1;
use driver::led::Led;
use driver::println_uart;
use driver::uart::init_uart;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::pac::Peripherals;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_uart!("Panic: {}", info);

    loop {
        asm::wfi();
    }
}

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

    init_uart(
        pins.uart,
        &mut clocks,
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
    );

    let mut led = Led::new(pins.user_led);
    let mut button1 = Button1::new(pins.buttons.button1);

    let mut output_count = 0;

    println_uart!("Hello, Wio Terminal! Uart is initialized.");

    loop {
        if button1.is_pressed() {
            led.toggle();

            if output_count == 0 {
                output_count = 1;
                println_uart!("Button 1 pressed");
            }
        } else {
            led.turn_off();

            if output_count == 1 {
                output_count = 0;
            }
        }
    }
}
