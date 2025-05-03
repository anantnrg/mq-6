#![no_std]

use core::marker::PhantomData;
use embedded_hal::adc::{Channel, OneShot};
use nb::block;

#[derive(Debug)]
pub enum MQ6Error<E> {
    ReadError(E),
    InvalidR0,
}

pub struct MQ6<ADC, PIN, ADCWORD>
where
    PIN: Channel<ADC>,
    ADC: OneShot<ADC, ADCWORD, PIN>,
    ADCWORD: Into<u32>,
{
    adc: ADC,
    pin: PIN,
    vref_mv: u32,
    rl_ohms: f32,
    _phantom: PhantomData<ADCWORD>,
}

impl<ADC, PIN, ADCWORD> MQ6<ADC, PIN, ADCWORD>
where
    PIN: Channel<ADC>,
    ADC: OneShot<ADC, ADCWORD, PIN>,
    ADCWORD: Into<u32>,
{
    pub fn new(adc: ADC, pin: PIN, vref_mv: u32, rl_ohms: f32) -> Self {
        Self {
            adc,
            pin,
            vref_mv,
            rl_ohms,
            _phantom: PhantomData,
        }
    }

    pub fn read_raw(&mut self) -> Result<ADCWORD, MQ6Error<ADC::Error>> {
        block!(self.adc.read(&mut self.pin)).map_err(MQ6Error::ReadError)
    }

    pub fn read_voltage_mv(&mut self) -> Result<u32, MQ6Error<ADC::Error>> {
        let raw: u32 = self.read_raw()?.into();
        Ok((raw * self.vref_mv) / 4095)
    }

    /// Calculates Rs, the sensor resistance
    pub fn read_rs(&mut self) -> Result<f32, MQ6Error<ADC::Error>> {
        let vout = self.read_voltage_mv()? as f32;
        if vout == 0.0 {
            return Ok(f32::INFINITY);
        }

        let vs = self.vref_mv as f32;
        let rs = self.rl_ohms * (vs - vout) / vout;
        Ok(rs)
    }

    /// Estimate PPM using Rs/R0 ratio
    pub fn read_ppm(&mut self, r0: f32) -> Result<f32, MQ6Error<ADC::Error>> {
        if r0 <= 0.0 {
            return Err(MQ6Error::InvalidR0);
        }

        let rs = self.read_rs()?;
        let ratio = rs / r0;

        // Approximation based on MQ-6 datasheet curve
        let a = 1000.0;
        let b = -0.47;
        let ppm = a * ratio.powf(b);

        Ok(ppm)
    }
}
