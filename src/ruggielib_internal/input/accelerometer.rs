use crate::{RuggieLib, input::errors::InputError};

///The 3DS accelerometer varies between -2G to +2G. Dividing its raw data with this value normalizes it to that range.
const ACCELEROMETER_SCALE:f32 = 1560.0;

impl RuggieLib{
    ///Enables or disables the accelerometer. 
    ///
    /// Enabling it does not result in almost any performance penalty. 
    ///
    /// Should be okay to keep it on during all the program's execution.
    pub fn set_accelerometer(&mut self, val: bool) -> Result<(), InputError>{
        self.hid.set_accelerometer(val).map_err(|_| InputError::AccelerometerSet)
    }
    
    ///If the accelerometer is enabled, retrieves its info.
    pub fn get_accelerometer_vector(&self) -> Result<AccelerometerInfo, InputError>{
        //Read raw accelerometer data
        let raw_accelerometer = self.hid.accelerometer_vector().map_err(|_| InputError::AccelerometerRead)?;
        let raw_data = <(i16, i16, i16)>::from(raw_accelerometer);

        //Convert that data to G
        Ok(AccelerometerInfo { 
            x: f32::from(raw_data.0) / ACCELEROMETER_SCALE, 
            y: f32::from(raw_data.1) / ACCELEROMETER_SCALE, 
            z: f32::from(raw_data.2) / ACCELEROMETER_SCALE 
        })
    }
}

///Accelerometer vector. The values are measured in G.
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct AccelerometerInfo {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}