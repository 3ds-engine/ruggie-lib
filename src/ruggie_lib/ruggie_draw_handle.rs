use citro2d_sys::*;
use citro3d_sys::*;

use crate::ruggie_lib::ruggie_screen_target::RuggieScreenTarget;

pub struct RuggieDrawHandle{
    target_screen: RuggieScreenTarget
}

impl RuggieDrawHandle{
    pub fn new(target_screen: &RuggieScreenTarget) -> Self{
        unsafe{
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW);
            C2D_SceneBegin(target_screen.screen);
        }
        
        RuggieDrawHandle { target_screen: target_screen.clone() }
    }

    pub fn clear_screen(&self, clear_color: Color){
        unsafe{
            C2D_TargetClear(self.target_screen.screen, C2D_Color32(clear_color.r, clear_color.g, clear_color.b, clear_color.a));
        }
    }

    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color){
        unsafe{
            let col = C2D_Color32(color.r, color.g, color.b, color.a);
            C2D_DrawRectangle(
                x, y, 0.0, w, h, col, col, col, col,
            );
        }
    }

}

impl Drop for RuggieDrawHandle{
    fn drop(&mut self) {
        unsafe{
            C3D_FrameEnd(0);
        }
    }
}

pub struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color{
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self{
        Color { r, g, b, a }
    }
}