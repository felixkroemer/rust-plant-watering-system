use crate::{PumpType, YL69Type};

pub struct Peripherals {
    pump: PumpType,
    yl69: YL69Type,
}

impl Peripherals {
    pub fn new (pump: PumpType, yl69: YL69Type) -> Peripherals {
        Peripherals { pump, yl69 }
    }

    pub fn get_pump(&mut self) -> &mut PumpType {
        &mut self.pump
    }
}