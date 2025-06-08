use core::ops::DerefMut;

use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::timer::TimerCounter;
use wio::pac::gclk::pchctrl::GEN_A;
use wio::pac::{MCLK, TC3};
use core::ops::Deref;

pub struct TimerCounterTC3(TimerCounter<TC3>);

impl TimerCounterTC3 {
    pub fn new(
        clocks: &mut GenericClockController,
        tc3: TC3,
        mclk: &mut MCLK,
    ) -> Self {
        let gclk5 = clocks.get_gclk(GEN_A::GCLK5).unwrap();
        let timer_clock = clocks.tc2_tc3(&gclk5).unwrap();

        Self(TimerCounter::tc3_(&timer_clock, tc3, mclk))
    }
}

impl Deref for TimerCounterTC3 {
    type Target = TimerCounter<TC3>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimerCounterTC3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
