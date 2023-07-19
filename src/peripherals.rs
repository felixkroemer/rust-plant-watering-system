use crate::{PumpType, YL69Type, DisplayType, WaterLevelSensorType};

pub struct Peripherals {
    pump: PumpType,
    yl69: YL69Type,
    display: DisplayType,
    water_level_sensor: WaterLevelSensorType,
}

impl Peripherals {
    pub fn new (pump: PumpType, yl69: YL69Type, display: DisplayType, water_level_sensor: WaterLevelSensorType) -> Peripherals {
        Peripherals { pump, yl69, display, water_level_sensor }
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

    pub fn get_water_level_sensor(&mut self) -> &mut WaterLevelSensorType {
        &mut self.water_level_sensor
    }
}