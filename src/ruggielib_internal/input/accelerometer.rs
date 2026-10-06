use ctru::services::hid::Acceleration;

use crate::{RuggieLib, input::errors::InputError};

impl RuggieLib{
    pub fn set_accelerometer(&mut self, val: bool) -> Result<(), InputError>{
        self.hid.set_accelerometer(val).map_err(|_| InputError::AccelerometerSet)
    }
    
    pub fn get_accelerometer_vector(&self) -> Result<AccelerometerInfo, InputError>{
        self.hid.accelerometer_vector()
            .map(AccelerometerInfo::from)
            .map_err(|_| InputError::AccelerometerRead)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct AccelerometerInfo {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl From<Acceleration> for AccelerometerInfo{
    fn from(value: Acceleration) -> Self {
        let raw_info = <(i16, i16, i16)>::from(value);

        AccelerometerInfo { 
            x: raw_info.0, 
            y: raw_info.1, 
            z: raw_info.2 
        }
    }
}