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
    gpio::{Alternate, Analog, Gpioa, Gpiob, OpenDrain, Output, PXx, Pin, PushPull, U},
    i2c::I2c,
    pac::{self, ADC1, I2C1},
    prelude::*,
};

use crate::{display::Display, pump::Pump, yl_69::YL69};

mod display;
mod peripherals;
mod pump;
mod yl_69;

type PumpType = Pump<PXx<Output<PushPull>>>;
// TODO: check if types can be erased further
type YL69Type = YL69<ADC1, Pin<Gpioa, U<1>, Analog>, Adc<ADC1>>;

type SclType = Pin<Gpiob, U<6>, Alternate<OpenDrain, 4>>;
type SdaType = Pin<Gpiob, U<7>, Alternate<OpenDrain, 4>>;
type DisplayType = Display<I2c<I2C1, (SclType, SdaType)>>;

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

    let yl69 = YL69::new(analog, adc);

    let pump_pin = gpioa
        .pa3
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
        .downgrade()
        .downgrade();

    let pump = Pump::new(pump_pin);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let mut scl =
        gpiob
            .pb6
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);
    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        100.kHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb1,
    );

    let display = Display::new(i2c);

    interrupt::free(|cs| {
        PERI.borrow(cs)
            .replace(Option::Some(Peripherals::new(pump, yl69, display)))
    });

    loop {
        led.toggle().unwrap();
        interrupt::free(|cs| {
            //TODO find better solution for getting mut ref to peripherals
            get_mut!(PERI, cs).get_pump().toggle();
            let reading = get_mut!(PERI, cs).get_sensor().read();
            get_mut!(PERI, cs).get_display().display(reading);
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
