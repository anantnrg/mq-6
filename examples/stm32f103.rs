#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

use stm32f1xx_hal::{adc::Adc, pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split();

    // Set PA0 to analog input
    let mut adc_pin = gpioa.pa0.into_analog(&mut gpioa.crl);

    // Create ADC instance properly
    let mut adc = Adc::adc1(dp.ADC1, clocks);

    loop {
        let val: u16 = adc.read(&mut adc_pin).unwrap();
        hprintln!("ADC reading: {}", val).unwrap();
    }
}
