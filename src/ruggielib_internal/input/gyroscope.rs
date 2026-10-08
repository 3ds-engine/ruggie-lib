use crate::{RuggieLib, input::errors::InputError};

impl RuggieLib{
    ///Enables or disables the gyroscope. 
    ///
    /// Enabling it does not result in almost any performance penalty. 
    ///
    /// Should be okay to keep it on during all the program's execution.
    pub fn set_gyroscope(&mut self, val: bool) -> Result<(), InputError>{
        self.hid.set_gyroscope(val).map_err(|_| InputError::GyroscopeSet)
    }

    ///If the gyroscope is enabled, retrieves its info.
    pub fn get_gyroscope_info(&self) -> Result<GyroscopeInfo, InputError>{

        //Reads raw gyroscope information
        let raw_angular_rate = self.hid.gyroscope_rate().map_err(|_| InputError::GyroscopeRead)?;
        let raw_info = <(i16, i16, i16)>::from(raw_angular_rate);

        //Reads the coefficient of Gyroscope, to parse to Degrees per Second
        let mut coeff: f32 = 0.0;
        unsafe{
            if ctru_sys::HIDUSER_GetGyroscopeRawToDpsCoefficient(&mut coeff) != 0{
                return Err(InputError::GyroscopeCoefficientRead)
            }
        }

        //Returns the gyroscope info
        Ok(GyroscopeInfo { 
            roll: f32::from(raw_info.0) * coeff, 
            pitch: f32::from(raw_info.1) * coeff, 
            yaw: f32::from(raw_info.2) * coeff
        })
    }
}

///Angular velocity of Gyroscope at this point in time. 
/// 
/// Represented in deg/s
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct GyroscopeInfo {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}