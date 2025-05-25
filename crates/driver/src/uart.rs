use wio_terminal as wio;

use core::cell::RefCell;
use core::fmt;
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m::interrupt::{self, Mutex};
use wio::hal::clock::GenericClockController;
use wio::hal::gpio::{Alternate, Pin, C, PB26, PB27};
use wio::hal::sercom::uart::{Config, Duplex, Pads, Uart};
use wio::hal::sercom::{IoSet2, Sercom2};
use wio::pac::MCLK;
use wio::prelude::*;

type UartConfig = Config<Pads<Sercom2, IoSet2, Pin<PB27, Alternate<C>>, Pin<PB26, Alternate<C>>>>;

pub static SERIAL: Mutex<RefCell<Option<Uart<UartConfig, Duplex>>>> =
    Mutex::new(RefCell::new(None));

pub fn init_uart(
    uart: wio::Uart,
    clocks: &mut GenericClockController,
    selcom: wio::pac::SERCOM2,
    mclk: &mut MCLK,
) {
    let serials = uart.init(clocks, 115200.Hz(), selcom, mclk);
    interrupt::free(|cs| {
        let mut serial = SERIAL.borrow(cs).borrow_mut();
        *serial = Some(serials);
    });
}

/// Uart writer for printing to the serial console.
/// <https://tomoyuki-nakabayashi.github.io/embedded-rust-techniques/03-bare-metal/print.html>
struct UartWriter;

impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            write_byte(c);
        }
        Ok(())
    }
}

fn write_byte(c: u8) {
    interrupt::free(|cs| {
        if let Some(ref mut s) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
            let _ = nb::block!(s.write(c));
        }
    });
}

pub fn _print_uart(args: fmt::Arguments) {
    let mut writer = UartWriter {};
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print_uart {
    ($($arg:tt)*) => ($crate::uart::_print_uart(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_uart {
    ($fmt:expr) => ($crate::print_uart!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_uart!(concat!($fmt, "\n"), $($arg)*));
}
