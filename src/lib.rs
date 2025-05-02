#![no_std]

use embedded_hal::digital::v2::{InputPin, OutputPin};

pub trait Adc<Pin> {
    fn read(&mut self, pin: &mut Pin) -> Result<u16, AdcError>;
}

#[derive(Debug)]
pub enum AdcError {
    ReadError,
}

pub struct MQ6<ADC, PIN> {
    pub adc: ADC,
    pub pin: PIN,
    pub vref_mv: u32,
}

impl<ADC, PIN> MQ6<ADC, PIN>
where
    PIN: InputPin,
    ADC: Adc<PIN>,
{
    pub fn new(adc: ADC, pin: PIN, vref_mv: u32) -> Self {
        Self { adc, pin, vref_mv }
    }

    pub fn read_raw(&mut self) -> Result<u16, AdcError> {
        self.adc.read(&mut self.pin)
    }

    pub fn read_voltage_mv(&mut self) -> Result<u32, AdcError> {
        let raw = self.read_raw()? as u32;
        Ok((raw * self.vref_mv) / 4095)
    }
}
