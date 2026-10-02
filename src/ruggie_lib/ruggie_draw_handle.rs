use citro2d_sys::*;
use citro3d_sys::*;

use crate::ruggie_lib::{RuggieLib, ruggie_screen_target::RuggieScreenTarget};

pub struct RuggieDrawHandle<'a> {
    lib: &'a mut RuggieLib,
    current_screen: *const RuggieScreenTarget,
}

impl<'a> RuggieDrawHandle<'a> {
    pub fn new(lib: &'a mut RuggieLib) -> Self {
        let current_screen : *const RuggieScreenTarget = &lib.top_left_screen;

        // SAFETY: Calling ffi initialization functions is safe as long as they are deinitialized in
        // the drop function of the object
        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW);

            // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
            // never mutate after they are built, so using them while having a valid mutable
            // reference to RuggieLib ensures the pointer is valid and can't be mutated
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen());
        }

        Self { 
            lib, 
            current_screen
        }
    }

    pub fn draw_top_left(&mut self) {
        let current_screen : *const RuggieScreenTarget = &self.lib.top_left_screen;
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe {
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen());
        }

        self.current_screen = current_screen;
    }

    pub fn draw_top_right(&mut self) {
        let current_screen : *const RuggieScreenTarget = &self.lib.top_right_screen;
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe {
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen());
        }

        self.current_screen = current_screen;
    }

    pub fn draw_bottom(&mut self) {
        let current_screen : *const RuggieScreenTarget = &self.lib.bottom_screen;

        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe {
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen());
        }

        self.current_screen = current_screen;
    }


    pub fn clear_screen(&mut self, clear_color: Color) {

        unsafe {
            // SAFETY: The only way to call this ffi function is by creating an instance of
            // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
            C2D_TargetClear(
                // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
                // never mutate after they are built, so using them while having a valid mutable
                // reference to RuggieLib ensures the pointer is valid and can't be mutated
                self.current_screen.as_ref_unchecked().get_screen(),
                C2D_Color32(clear_color.r, clear_color.g, clear_color.b, clear_color.a),
            );
        }
    }

    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            let col = C2D_Color32(color.r, color.g, color.b, color.a);
            C2D_DrawRectangle(x, y, 0.0, w, h, col, col, col, col);
        }
    }
}

impl Drop for RuggieDrawHandle<'_> {
    fn drop(&mut self) {
        // SAFETY: Calling ffi deinitialization is always save as long as the only way to obtain a
        // RuggieDrawHandle is by calling initialization
        unsafe {
            C3D_FrameEnd(0);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}
