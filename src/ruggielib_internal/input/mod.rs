pub mod errors;
pub mod gyroscope;
pub mod accelerometer;
pub mod buttons;

use ctru::services::hid::{KeyPad};
use ctru_sys::osGet3DSliderState;
use crate::{RuggieLib, input::buttons::Button};

///Approximated max value of the circlepad
const CIRCLEPAD_MAX_ABS: i16 = 156;

impl RuggieLib{

    ///Scans the input to gather the input information. 
    /// 
    /// Should be used once per frame, and before checking any input value.
    /// 
    /// # Example
    /// ```
    /// let rl = RuggieLib::new().unwrap();
    /// while(rl.is_running()) {
    ///     rl.wait_for_vblank();
    ///     rl.scan_input();
    /// 
    ///     /* Check for input */
    /// }
    /// ```
    /// 
    pub fn scan_input(&mut self){
        self.hid.scan_input();
    }

    ///Returns true if the button has been pressed in that frame.
    pub fn button_pressed(&self, button: Button) -> bool{
        self.hid.keys_down().contains(button.into())
    }

    ///Returns true if the button is being held.
    pub fn button_held(&self, button: Button) -> bool{
        self.hid.keys_held().contains(button.into())
    }

    ///Returns true if the button has been released in that frame.
    pub fn button_released(&self, button: Button) -> bool{
        self.hid.keys_up().contains(button.into())
    }

    ///Value of 3d slider from 0 to 1. This can be used to manage the 3D Stereoscopic strength manually.
    pub fn get_3d_slider_state(&self) -> f32{
        unsafe{
            osGet3DSliderState()
        }
    }

    ///Value of volume slider from 0 to 1. The 3ds automatically changes the program's volume, so don't use this for mixing the audio.
    pub fn get_volume_slider_state(&self) -> f32{
        self.hid.volume_slider()
    }

    ///Value of the circlepad in format (x,y) normalized into (-1,1) range
    pub fn circlepad_direction(&self) -> (f32,f32){
        let circlepad_pos = self.hid.circlepad_position();

        let x = (circlepad_pos.0 as f32 / CIRCLEPAD_MAX_ABS as f32).clamp(-1.0, 1.0);
        let y = (circlepad_pos.1 as f32 / CIRCLEPAD_MAX_ABS as f32).clamp(-1.0, 1.0);

        (x,y)
    }

    ///Returns a tuple based on normalized c_stick direction
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

        let mut normalized_mul = 1.0;
        if !(c_stick_y == 0.0 && c_stick_x == 0.0) {
            normalized_mul = 1.0/f32::sqrt(f32::powi(c_stick_x, 2) + f32::powi(c_stick_y, 2));
        }
        (c_stick_x * normalized_mul, c_stick_y * normalized_mul)
    }

    ///If the touch pad is being pressed, returns the coordinates.
    /// # Example
    /// ```
    /// let rl = RuggieLib::new().unwrap();
    /// while(rl.is_running()) {
    ///     rl.wait_for_vblank();
    ///     rl.scan_input();
    /// 
    ///     let Some(coord) = rl.touch_pad_position(){
    ///         printl!("x_pos: {}, y_pos: {}", coord.0 , coord.1);
    ///     }
    /// }
    /// ```
    pub fn touch_pad_position(&self) -> Option<(u16,u16)>{
        if self.hid.keys_held().contains(KeyPad::TOUCH){
            return Some(self.hid.touch_position())
        }
        None
    }
    
    
}