use core::{fmt::Debug, marker::PhantomData};

use embedded_hal::adc::{Channel, OneShot};

pub struct YL69<ADC, PIN: Channel<ADC>, OS: OneShot<ADC, u16, PIN>> {
    pin: PIN,
    adc: OS,
    pd: PhantomData<ADC>,
}

impl<ADC, PIN, OS> YL69<ADC, PIN, OS>
where
    PIN: Channel<ADC>,
    OS: OneShot<ADC, u16, PIN>,
    OS::Error: Debug,
{
    pub fn new(pin: PIN, adc: OS) -> Self {
        YL69 {
            pin,
            adc,
            pd: PhantomData,
        }
    }

    pub fn read(&mut self) -> u16 {
        self.adc.read(&mut self.pin).unwrap()
    }
}
