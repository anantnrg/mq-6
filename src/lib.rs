#![no_std]

use core::marker::PhantomData;
use embedded_hal::adc::{Channel, OneShot};
use nb::block;

#[derive(Debug)]
pub enum MQ6Error<ADCERR> {
    Adc(ADCERR),
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
        let raw = self.read_raw()?;
        Ok((raw * self.vref_mv) / 4095)
    }

    /// Calculates the Rs value (sensor resistance) using measured voltage
    pub fn read_rs(&mut self) -> Result<f32, MQ6Error<ADC::Error>> {
        let vout = self.read_voltage_mv()? as f32;
        if vout == 0.0 {
            return Ok(f32::INFINITY); // open circuit maybe
        }

        let vs = self.vref_mv as f32;
        let rs = self.rl_ohms * (vs - vout) / vout;
        Ok(rs)
    }

    /// Estimate PPM using Rs/R0 ratio with approximation curve
    /// You must provide R0 (calibrated resistance in clean air)
    pub fn read_ppm(&mut self, r0: f32) -> Result<f32, MQ6Error<ADC::Error>> {
        if r0 <= 0.0 {
            return Err(MQ6Error::InvalidR0);
        }

        let rs = self.read_rs()?;
        let ratio = rs / r0;

        // MQ-6 typical approximation for LPG:
        // log(ppm) = (log(rs/r0) - b) / m
        // Curve data: (Rs/R0 vs PPM), from datasheet
        let a = 1000.0; // arbitrary fitting param
        let b = -0.47; // slope from datasheet log-log graph (approx)
        let ppm = a * ratio.powf(b);

        Ok(ppm)
    }
}
