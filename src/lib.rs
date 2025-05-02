#![no_std]

use core::marker::PhantomData;
use embedded_hal::adc::{Channel, OneShot};

#[derive(Debug)]
pub enum AdcError {
    ReadError,
}

pub struct MQ6<ADC, PIN, ADCWORD>
where
    PIN: Channel<ADC>,
    ADC: OneShot<ADC, ADCWORD, PIN>,
    ADCWORD: Into<u32>,
{
    pub adc: ADC,
    pub pin: PIN,
    pub vref_mv: u32,
    _phantom: PhantomData<ADCWORD>,
}

impl<ADC, PIN, ADCWORD> MQ6<ADC, PIN, ADCWORD>
where
    PIN: Channel<ADC>,
    ADC: OneShot<ADC, ADCWORD, PIN>,
    ADCWORD: Into<u32>,
{
    pub fn new(adc: ADC, pin: PIN, vref_mv: u32) -> Self {
        Self {
            adc,
            pin,
            vref_mv,
            _phantom: PhantomData,
        }
    }

    pub fn read_raw(&mut self) -> Result<ADCWORD, AdcError> {
        self.adc
            .read(&mut self.pin)
            .map_err(|_| AdcError::ReadError)
    }

    pub fn read_voltage_mv(&mut self) -> Result<u32, AdcError> {
        let raw: u32 = self.read_raw()?.into();
        Ok((raw * self.vref_mv) / 4095)
    }
}
