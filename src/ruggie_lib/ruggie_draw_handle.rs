use citro2d_sys::*;
use citro3d_sys::*;

use crate::ruggie_lib::RuggieLib;

pub struct RuggieDrawHandle<'a> {
    lib: &'a mut RuggieLib,
    screen_index: usize,
}

impl<'a> RuggieDrawHandle<'a> {
    pub fn new(lib: &'a mut RuggieLib) -> Self {
        let screen_index = 0usize;
        let target_screen = &mut lib.screens[screen_index];
        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW);
            C2D_SceneBegin(&mut target_screen.screen);
        }

        Self { 
            lib, 
            screen_index 
        }
    }

    pub fn draw_top_left(&mut self) {
        self.screen_index = 0;
    }

    pub fn draw_top_right(&mut self) {
        self.screen_index = 1;
    }

    pub fn draw_bottom(&mut self) {
        self.screen_index = 2;
    }


    pub fn clear_screen(&mut self, clear_color: Color) {
        unsafe {
            C2D_TargetClear(
                &mut self.lib.screens[self.screen_index].screen,
                C2D_Color32(clear_color.r, clear_color.g, clear_color.b, clear_color.a),
            );
        }
    }

    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        unsafe {
            let col = C2D_Color32(color.r, color.g, color.b, color.a);
            C2D_DrawRectangle(x, y, 0.0, w, h, col, col, col, col);
        }
    }
}

impl Drop for RuggieDrawHandle<'_> {
    fn drop(&mut self) {
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
