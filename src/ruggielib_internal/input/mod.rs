pub mod errors;
pub mod gyroscope;
pub mod accelerometer;
pub mod buttons;

use ctru::services::hid::{KeyPad};
use ctru_sys::osGet3DSliderState;
use crate::{RuggieLib, input::buttons::Button};


impl RuggieLib{

    pub fn scan_input(&mut self){
        self.hid.scan_input();
    }

    pub fn button_pressed(&self, button: Button) -> bool{
        self.hid.keys_down().contains(button.into())
    }

    pub fn button_held(&self, button: Button) -> bool{
        self.hid.keys_held().contains(button.into())
    }

    pub fn button_released(&self, button: Button) -> bool{
        self.hid.keys_up().contains(button.into())
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

