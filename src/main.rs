#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// use panic_halt as _;
use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};
use stm32f3xx_hal::{adc, pac, prelude::*};

mod yl_69;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // panic!{"test123"};

    let dp: pac::Peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut led = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let adc_common = adc::CommonAdc::new(dp.ADC1_2, &clocks, &mut rcc.ahb);

    let adc = adc::Adc::new(
        dp.ADC1,
        adc::config::Config::default(),
        &clocks,
        &adc_common,
    );

    let analog = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    let mut yl69 = yl_69::YL69::new(analog, adc);

    loop {
        let adc_data: u16 = yl69.read();
        rprintln!("{}", adc_data);
        led.toggle().unwrap();
        asm::delay(1_000_000);
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}
