use ctru::services::hid::AngularRate;

use crate::{RuggieLib, input::errors::InputError};

impl RuggieLib{
    pub fn set_gyroscope(&mut self, val: bool) -> Result<(), InputError>{
        self.hid.set_gyroscope(val).map_err(|_| InputError::GyroscopeSet)
    }

    pub fn get_gyroscope_info(&self) -> Result<GyroscopeInfo, InputError>{
        self.hid.gyroscope_rate()
            .map(GyroscopeInfo::from)
            .map_err(|_| InputError::GyroscopeRead)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct GyroscopeInfo {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
}

impl From<AngularRate> for GyroscopeInfo{
    fn from(value: AngularRate) -> Self {
        let raw_info = <(i16, i16, i16)>::from(value);

        GyroscopeInfo { 
            roll: raw_info.0, 
            pitch: raw_info.1, 
            yaw: raw_info.2 
        }
    }
}