#![no_main]
#![no_std]
#![deny(unsafe_code)]

use panic_semihosting as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{adc, pac, prelude::*};

use rtt_target::{rprintln, rtt_init, rtt_init_print};

use mq_6::{Adc as Mq6Adc, AdcError, MQ6};

#[entry]
fn main() -> ! {
    rtt_init!(); // rprintln! be ready, mate

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);

    // Setup ADC1
    let mut adc1 = adc::Adc::new(dp.ADC1, &clocks);

    // Setup GPIOB
    let mut gpiob = dp.GPIOB.split();
    let mut pb0 = gpiob.pb0.into_analog(&mut gpiob.crl);

    // Wrap it into our MQ6 struct
    let mut mq6 = MQ6::new(Adc1Wrapper { adc: adc1 }, pb0, 3300);

    loop {
        match mq6.read_voltage_mv() {
            Ok(v) => rprintln!("MQ6 voltage: {} mV", v),
            Err(_) => rprintln!("Read error, ye lubber!"),
        }
    }
}

pub struct Adc1Wrapper {
    pub adc: adc::Adc<pac::ADC1>,
}

impl<PIN> Mq6Adc<PIN> for Adc1Wrapper
where
    PIN: embedded_hal::adc::Channel<pac::ADC1, ID = u8>,
{
    fn read(&mut self, pin: &mut PIN) -> Result<u16, AdcError> {
        embedded_hal::adc::OneShot<pac::ADC1, u16, PIN>::read(&mut self.adc, pin)
            .map_err(|_| AdcError::ReadError)
    }
}
