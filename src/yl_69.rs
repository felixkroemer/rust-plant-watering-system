use core::{fmt::Debug, marker::PhantomData};

use embedded_hal::adc::{Channel, OneShot};

pub struct YL69<ADC, PIN, OS> {
    pin: PIN,
    os: OS,
    adc: PhantomData<ADC>
}

impl<ADC, PIN, OS> YL69<ADC, PIN, OS>
where
    PIN: Channel<ADC>,
    OS: OneShot<ADC, u16, PIN>,
    OS::Error: Debug,
{
    pub fn new(pin: PIN, os: OS) -> Self {
        YL69 {
            pin,
            os,
            adc: PhantomData,
        }
    }

    pub fn read(&mut self) -> u16 {
        self.os.read(&mut self.pin).unwrap()
    }
}
