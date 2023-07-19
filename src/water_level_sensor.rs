use core::fmt::Debug;
use embedded_hal::{digital::v2::InputPin};


pub struct WaterLevelSensor<T> {
    pin: T,
}

impl<T> WaterLevelSensor<T>
where
    T: InputPin,
    T::Error: Debug,
{
    pub fn new(pin: T) -> WaterLevelSensor<T> {
        WaterLevelSensor { pin }
    }

    pub fn low(&mut self) -> bool{
        self.pin.is_high().unwrap_or(false)
    }
}
