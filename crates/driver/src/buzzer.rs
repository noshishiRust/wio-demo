use wio_terminal as wio;

use atsamd_hal::pwm::Tcc0Pwm;
use core::ops::{Deref, DerefMut};
use wio::aliases::{BuzzerCtrlId, BuzzerCtrlMode};
use wio::hal::clock::GenericClockController;
use wio::pac::{MCLK, TCC0};
use wio::Buzzer as WioBuzzer;

pub struct Buzzer(Tcc0Pwm<BuzzerCtrlId, BuzzerCtrlMode>);

impl Buzzer {
    pub fn new(
        buzzer: WioBuzzer,
        clocks: &mut GenericClockController,
        tcc: TCC0,
        mclk: &mut MCLK,
    ) -> Self {
        let pvm = buzzer.init(clocks, tcc, mclk);
        Self(pvm)
    }
}

impl Deref for Buzzer {
    type Target = Tcc0Pwm<BuzzerCtrlId, BuzzerCtrlMode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Buzzer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
