use crate::{PumpType, YL69Type, DisplayType};

pub struct Peripherals {
    pump: PumpType,
    yl69: YL69Type,
    display: DisplayType,
}

impl Peripherals {
    pub fn new (pump: PumpType, yl69: YL69Type, display: DisplayType) -> Peripherals {
        Peripherals { pump, yl69, display }
    }

    pub fn get_pump(&mut self) -> &mut PumpType {
        &mut self.pump
    }

    pub fn get_display(&mut self) -> &mut DisplayType {
        &mut self.display
    }

    pub fn get_sensor(&mut self) -> &mut YL69Type {
        &mut self.yl69
    }
}