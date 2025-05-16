#![no_std]

pub trait Mq6Adc {
    type Error;

    /// Read raw ADC value (usually 0–4095)
    fn read_raw(&mut self) -> Result<u16, Self::Error>;
}

pub struct MQ6;

impl MQ6 {
    /// Convert raw ADC value to millivolts
    pub fn adc_to_mv(adc_value: u16, vref_mv: u32, max_adc: u16) -> u32 {
        (adc_value as u32 * vref_mv) / max_adc as u32
    }

    /// Get Rs/RL ratio from measured voltage
    pub fn voltage_to_rs_over_rl(voltage_mv: f32, vcc_mv: f32) -> f32 {
        if voltage_mv == 0.0 {
            f32::INFINITY
        } else {
            (vcc_mv - voltage_mv) / voltage_mv
        }
    }

    /// Convenience: Read voltage in mV from the sensor via an ADC provider
    pub fn read_voltage_mv<A: Mq6Adc>(
        adc: &mut A,
        vref_mv: u32,
        max_adc: u16,
    ) -> Result<u32, A::Error> {
        let raw = adc.read_raw()?;
        Ok(Self::adc_to_mv(raw, vref_mv, max_adc))
    }
}
