use core::fmt::Debug;
use embedded_hal::digital::v2::OutputPin;

pub struct Pump<T> {
    pin: T,
    bool: bool,
}

impl<T> Pump<T>
where
    T: OutputPin,
    T::Error: Debug,
{
    pub fn new(pin: T) -> Pump<T> {
        Pump { pin, bool: true }
    }

    pub fn toggle(&mut self) {
        if self.bool {
            self.pin.set_high().unwrap();
        } else {
            self.pin.set_low().unwrap();
        }
        self.bool = !self.bool;
    }
}
