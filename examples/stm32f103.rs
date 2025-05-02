#![no_main]
#![no_std]
#![deny(unsafe_code)]

use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f1xx_hal::{adc, pac, prelude::*};

use rtt_target::{rprintln, rtt_init};

use mq_6::{AdcError, MQ6};

#[entry]
fn main() -> ! {
    rtt_init!();

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);

    // Setup ADC1
    let mut adc1 = adc::Adc::adc1(dp.ADC1, clocks);

    // Setup GPIOB
    let mut gpiob = dp.GPIOB.split();
    let mut pb0 = gpiob.pb0.into_analog(&mut gpiob.crl);

    // Initialize MQ6 sensor
    let mut mq6 = MQ6::new(adc1, pb0, 3300);

    loop {
        match mq6.read_voltage_mv() {
            Ok(v) => rprintln!("MQ6 voltage: {} mV", v),
            Err(_) => rprintln!("Read error!"),
        }
    }
}
