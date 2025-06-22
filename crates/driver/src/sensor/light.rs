use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::LightSensor as WioLightSensor;
use wio::pac::{MCLK, ADC1};
use wio::aliases::LightSensorAdc;
use atsamd_hal::adc::Adc;
use core::ops::{Deref, DerefMut};

pub struct LightSensor((Adc<ADC1>, LightSensorAdc));

impl LightSensor {
    pub fn new(
        sensor: WioLightSensor,
        adc: ADC1,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
    ) -> Self {
        let (adc, pd1) = sensor.init(adc, clocks, mclk);
        Self((adc, pd1))
    }
}

impl Deref for LightSensor {
    type Target = (Adc<ADC1>, LightSensorAdc);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LightSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
