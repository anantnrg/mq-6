#![no_std]

use embedded_hal::adc::OneShot;

pub struct MQ6<Adc, Pin> {
    pub adc: Adc,
    pub pin: Pin,
}

impl<Adc, Pin, Word> MQ6<Adc, Pin>
where
    Adc: OneShot<Adc, Word, Pin>,
    Word: Into<u32>,
{
    pub fn new(adc: Adc, pin: Pin) -> Self {
        Self { adc, pin }
    }

    pub fn read_raw(&mut self) -> Result<u32, Error> {
        self.adc
            .read(&mut self.pin)
            .map(|value| value.into())
            .map_err(|_| Error::Adc)
    }

    pub fn read_ppm(&mut self) -> Result<f32, Error> {
        let raw = self.read_raw()?; // ADC value, usually 0-4095 if 12-bit
        let voltage = (raw as f32 / 4095.0) * 3.3; // Assuming 3.3V ADC ref

        // Simplified linear-ish formula, real one needs a fucking calibration curve.
        let ppm = (voltage / 3.3) * 1000.0; // Dummy mapping: 0-3.3V -> 0-1000ppm

        Ok(ppm)
    }
}

#[derive(Debug)]
pub enum Error {
    Adc,
}
