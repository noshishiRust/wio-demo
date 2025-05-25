#![no_std]
#![no_main]

use wio_terminal as wio;

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m::peripheral::NVIC;
use driver::led::Led;
use driver::println_uart;
use driver::uart::init_uart;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::timer::TimerCounter;
use wio::pac::gclk::pchctrl::GEN_A;
use wio::pac::interrupt;
use wio::pac::{CorePeripherals, Peripherals, TC3};
use wio::prelude::*;

struct Ctx {
    led: Led,
    tc3: TimerCounter<TC3>,
}

static mut CTX: Option<Ctx> = None;

#[allow(static_mut_refs)]
#[interrupt]
fn TC3() {
    unsafe {
        let ctx = CTX.as_mut().unwrap();
        ctx.tc3.wait().unwrap();
        ctx.led.toggle();
    }
}

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

    let gclk5 = clocks.get_gclk(GEN_A::GCLK5).unwrap();
    let timer_clock = clocks.tc2_tc3(&gclk5).unwrap();
    let mut tc3 = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);

    unsafe {
        NVIC::unmask(interrupt::TC3);
    }

    tc3.start(1.secs());
    tc3.enable_interrupt();

    let led = Led::new(pins.user_led);

    unsafe { CTX = Some(Ctx { led, tc3 }) }

    init_uart(
        pins.uart,
        &mut clocks,
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
    );

    println_uart!("Hello, Wio Terminal! Uart is initialized.");

    loop {
        println_uart!("Hello?!");
        delay.delay_ms(1000_u16);
    }
}
