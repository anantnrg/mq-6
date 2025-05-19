#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use rtt_target::rprintln;

use stm32f1xx_hal::{
    adc::{Adc, SampleTime},
    pac,
    prelude::*,
};

use mq_6::{MQ6, Mq6Adc};

struct MyAdc<'a> {
    adc: &'a mut Adc<pac::ADC1>,
    pin: &'a mut stm32f1xx_hal::gpio::gpioa::PA0<stm32f1xx_hal::gpio::Analog>,
}

impl<'a> Mq6Adc for MyAdc<'a> {
    type Error = ();

    fn read_raw(&mut self) -> Result<u16, Self::Error> {
        self.adc.read(self.pin).map_err(|_| ())
    }
}

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split();

    let mut adc = Adc::adc1(dp.ADC1, clocks);
    adc.set_sample_time(SampleTime::T_239); // max accuracy

    let mut pin = gpioa.pa0.into_analog(&mut gpioa.crl);

    let mut my_adc = MyAdc {
        adc: &mut adc,
        pin: &mut pin,
    };

    loop {
        let voltage = MQ6::read_voltage_mv(&mut my_adc, 3300, 4095).unwrap_or(0);
       rprintln!("Voltage (mV): {}", voltage);

        let rs_rl = MQ6::voltage_to_rs_over_rl(voltage as f32, 3300.0);
        rprintln!("Rs/RL ratio: {:.2}", rs_rl);
    }
}
