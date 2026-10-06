use ctru::services::hid::{Acceleration, AngularRate, KeyPad};
use ctru_sys::osGet3DSliderState;

use crate::{RuggieLib, input::errors::InputError};

pub mod errors;

impl RuggieLib{

    pub fn scan_input(&mut self){
        self.hid.scan_input();
    }

    pub fn button_pressed(&self, button: KeyPad) -> bool{
        self.hid.keys_down().contains(button)
    }

    pub fn button_held(&self, button: KeyPad) -> bool{
        self.hid.keys_held().contains(button)
    }

    pub fn button_released(&self, button: KeyPad) -> bool{
        self.hid.keys_up().contains(button)
    }

    pub fn get_3d_slider_state(&self) -> f32{
        unsafe{
            osGet3DSliderState()
        }
    }

    pub fn get_volume_slider_state(&self) -> f32{
        self.hid.volume_slider()
    }

    pub fn circlepad_direction(&self) -> (f32,f32){
        let circlepad_info = self.hid.circlepad_position();

        (normalize_i16(circlepad_info.0),
        normalize_i16(circlepad_info.1))
    }

    pub fn c_stick_direction(&self) -> (f32, f32){
        let mut c_stick_x = 0.0;
        let mut c_stick_y =  0.0;

        let buttons_held = self.hid.keys_held();

        if buttons_held.contains(KeyPad::CSTICK_RIGHT){
            c_stick_x = 1.0;
        }
        else if buttons_held.contains(KeyPad::CSTICK_LEFT){
            c_stick_x = -1.0;
        }

        if buttons_held.contains(KeyPad::CSTICK_UP){
            c_stick_y = 1.0;
        }
        else if buttons_held.contains(KeyPad::CSTICK_DOWN){
            c_stick_y = -1.0;
        }

        (c_stick_x, c_stick_y)
    }

    pub fn touch_pad_position(&self) -> Option<(u16,u16)>{
        if self.hid.keys_held().contains(KeyPad::TOUCH){
            return Some(self.hid.touch_position())
        }
        None
    }

    pub fn set_accelerometer(&mut self, val: bool) -> Result<(), InputError>{
        self.hid.set_accelerometer(val).map_err(|_| InputError::AccelerometerSet)
    }

    pub fn set_gyroscope(&mut self, val: bool) -> Result<(), InputError>{
        self.hid.set_gyroscope(val).map_err(|_| InputError::GyroscopeSet)
    }

    
    pub fn get_accelerometer_vector(&self) -> Result<AccelerometerInfo, InputError>{
        self.hid.accelerometer_vector()
            .map(AccelerometerInfo::from)
            .map_err(|_| InputError::AccelerometerRead)
    }
    
    pub fn get_gyroscope_info(&self) -> Result<GyroscopeInfo, InputError>{
        self.hid.gyroscope_rate()
            .map(GyroscopeInfo::from)
            .map_err(|_| InputError::GyroscopeRead)
    }
    
}

//Necessary for passing the value from (-32768 , 32767) to (-1 , 1)
fn normalize_i16(value: i16) -> f32{
    let normalized = if value < 0 {
    value as f32 / 32768.0
    } else {
    value as f32 / 32767.0
    };

    normalized
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