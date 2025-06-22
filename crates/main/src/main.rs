#![no_std]
#![no_main]

use wio_terminal as wio;

use core::panic::PanicInfo;
use cortex_m::asm;
use driver::println_uart;
use driver::uart::init_uart;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

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
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK, // https://www.intel.co.jp/content/www/jp/ja/docs/programmable/683047/16-0/global-clock-control-block.html
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = wio::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut accel =
        pins.accelerometer
            .init(&mut clocks, peripherals.SERCOM4, &mut peripherals.MCLK);

    let accel_id = accel.get_device_id().unwrap();

    init_uart(
        pins.uart,
        &mut clocks,
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
    );

    println_uart!("Hello, Wio Terminal! Uart is initialized.");
    println_uart!("accel device id: 0x{:X}", accel_id);

    loop {
        let point = accel.accel_norm().unwrap();
        println_uart!("{:?}", point);
        delay.delay_ms(1000u16);
    }
}
