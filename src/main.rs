#![no_main]
#![no_std]

use cortex_m::{
    asm,
    interrupt::{self, Mutex},
};
use cortex_m_rt::entry;
use peripherals::Peripherals;
// use panic_halt as _;
use core::{cell::RefCell, panic::PanicInfo};
use rtt_target::{rprintln, rtt_init_print};
use stm32f3xx_hal::{
    adc::{self, Adc},
    gpio::{Analog, Gpioa, Output, PXx, Pin, PushPull, U},
    pac::{self, ADC1},
    prelude::*,
};

use crate::{pump::Pump, yl_69::YL69};

mod peripherals;
mod pump;
mod yl_69;

type PumpType = Pump<PXx<Output<PushPull>>>;
type YL69Type = YL69<ADC1, Pin<Gpioa, U<1>, Analog>, Adc<ADC1>>;

static PERI: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(Option::None));

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

    let mut yl69 = YL69::new(analog, adc);

    let pump_pin = gpioa
        .pa3
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
        .downgrade()
        .downgrade();

    let pump = Pump::new(pump_pin);

    interrupt::free(|cs| {
        PERI.borrow(cs)
            .replace(Option::Some(Peripherals::new(pump, yl69)))
    });

    loop {
        led.toggle().unwrap();
        interrupt::free(|cs| {
            get_mut!(PERI, cs).get_pump().toggle();
        });
        asm::delay(5_000_000);
    }
}

#[macro_export]
macro_rules! get_mut {
    ( $x:ident , $cs:ident) => {{
        $x.borrow($cs).borrow_mut().as_mut().unwrap()
    }};
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}
