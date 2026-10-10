use citro2d_sys::{
    C2D_Color32, C2D_DrawCircleSolid, C2D_DrawEllipseSolid, C2D_DrawLine, C2D_DrawRectSolid,
    C2D_DrawSprite, C2D_DrawTriangle, C2D_SceneBegin, C2D_SpriteSetPos, C2D_TargetClear,
};
use citro3d_sys::{C3D_FRAME_SYNCDRAW, C3D_FrameBegin, C3D_FrameEnd};

use crate::{RuggieLib, draw::sprite::Sprite, screen::RuggieScreenTarget};

pub mod color;
use color::Color;

pub mod sprite;
pub mod sprite_sheet;

pub struct RuggieDrawHandle<'a> {
    lib: &'a mut RuggieLib,
    current_screen: *const RuggieScreenTarget,
}

// --- Creation
impl<'a> RuggieDrawHandle<'a> {
    pub fn new(lib: &'a mut RuggieLib) -> Self {
        let current_screen = &raw const lib.top_left_screen;

        // SAFETY: Calling ffi initialization functions is safe as long as they are deinitialized in
        // the drop function of the object
        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW);

            // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
            // never mutate after they are built, so using them while having a valid mutable
            // reference to RuggieLib ensures the pointer is valid and can't be mutated
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen_raw_mut());
        }

        Self {
            lib,
            current_screen,
        }
    }
}

// --- Private utilities
impl RuggieDrawHandle<'_> {
    fn c2d_color(color: Color) -> u32 {
        unsafe {
            // SAFETY: This function does simple arithmetic to get a u32 from the four u8 fields of
            // a color. Calling it is always safe
            C2D_Color32(color.r, color.g, color.b, color.a)
        }
    }
}

// --- Screen functionality
impl RuggieDrawHandle<'_> {
    pub fn draw_top_left(&mut self) {
        let current_screen = &raw const self.lib.top_left_screen;
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe {
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen_raw_mut());
        }

        self.current_screen = current_screen;
    }

    pub fn draw_top_right(&mut self) {
        let current_screen = &raw const self.lib.top_right_screen;
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe {
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen_raw_mut());
        }

        self.current_screen = current_screen;
    }

    pub fn draw_bottom(&mut self) {
        let current_screen = &raw const self.lib.bottom_screen;

        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe {
            C2D_SceneBegin(current_screen.as_ref_unchecked().get_screen_raw_mut());
        }

        self.current_screen = current_screen;
    }

    #[must_use]
    pub const fn screen_witdh(&self) -> u32 {
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe { self.current_screen.as_ref_unchecked().width() }
    }

    #[must_use]
    pub const fn screen_height(&self) -> u32 {
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe { self.current_screen.as_ref_unchecked().height() }
    }

    #[must_use]
    pub const fn screen_dimensions(&self) -> (u32, u32) {
        (self.screen_witdh(), self.screen_height())
    }
}

// --- Screen clearing
impl RuggieDrawHandle<'_> {
    pub fn clear_screen(&mut self, clear_color: Color) {
        unsafe {
            // SAFETY: The only way to call this ffi function is by creating an instance of
            // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
            C2D_TargetClear(
                // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
                // never mutate after they are built, so using them while having a valid mutable
                // reference to RuggieLib ensures the pointer is valid and can't be mutated
                self.current_screen.as_ref_unchecked().get_screen_raw_mut(),
                C2D_Color32(clear_color.r, clear_color.g, clear_color.b, clear_color.a),
            );
        }
    }
}

// --- 2D Primitives
impl RuggieDrawHandle<'_> {
    pub fn draw_line(&self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: u16, color: Color) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
        }
    }
}

// --- 2D Shapes
impl RuggieDrawHandle<'_> {
    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawRectSolid(x, y, 0.0, w, h, color);
        }
    }

    pub fn draw_triangle(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: Color,
    ) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawTriangle(x0, y0, color, x1, y1, color, x2, y2, color, 0.0);
        }
    }

    pub fn draw_circle(&self, x: f32, y: f32, r: f32, color: Color) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawCircleSolid(x, y, 0.0, r, color);
        }
    }

    pub fn draw_ellipse(&self, x: f32, y: f32, rx: f32, ry: f32, color: Color) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawEllipseSolid(
                x - (rx / 2.0),
                y - (ry / 2.0),
                0.0,
                rx * 2.0,
                ry * 2.0,
                color,
            );
        }
    }
}

// --- 2D Outlines
impl RuggieDrawHandle<'_> {
    pub fn draw_rectangle_outline(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        thickness: u16,
        color: Color,
    ) {
        let color = Self::c2d_color(color);
        // SAFETY: The only way to call these ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawLine(x, y, color, x + w, y, color, thickness.into(), 0.0);
            C2D_DrawLine(x + w, y, color, x + w, y + h, color, thickness.into(), 0.0);
            C2D_DrawLine(x + w, y + h, color, x, y + h, color, thickness.into(), 0.0);
            C2D_DrawLine(x, y + h, color, x, y, color, thickness.into(), 0.0);
        }
    }

    pub fn draw_triangle_outline(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        thickness: u16,
        color: Color,
    ) {
        let color = Self::c2d_color(color);
        // SAFETY: The only way to call these ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
            C2D_DrawLine(x1, y1, color, x2, y2, color, thickness.into(), 0.0);
            C2D_DrawLine(x2, y2, color, x0, y0, color, thickness.into(), 0.0);
        }
    }

    pub fn draw_circle_outline(
        &self,
        x: f32,
        y: f32,
        r: f32,
        thickness: u16,
        sides: u32,
        color: Color,
    ) {
        let color = Self::c2d_color(color);
        let angle_step = 2.0 * std::f32::consts::PI / sides as f32;

        // SAFETY: The only way to call these ffi functions is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            for i in 0..sides {
                let a0 = angle_step * i as f32;
                let a1 = angle_step * (i + 1) as f32;

                let x0 = x + r * a0.cos();
                let y0 = y + r * a0.sin();
                let x1 = x + r * a1.cos();
                let y1 = y + r * a1.sin();

                C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
            }
        }
    }
}

// --- 2D Sprites
impl RuggieDrawHandle<'_> {
    pub fn draw_sprite(&self, sprite: &mut Sprite, x: f32, y: f32) {
        unsafe {
            // SAFETY: Dereferencing a raw sprite can be unsafe, only if the user creates the raw
            // sprite themselves, otherwise it is impossible to create an invalid sprite
            let raw_sprite = sprite.get_raw_mut();

            // SAFETY: The only way to call these ffi functions is by creating an instance of
            // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
            C2D_SpriteSetPos(raw_sprite, x, y);
            C2D_DrawSprite(raw_sprite);
        }
    }
}

// --- Destruction
impl Drop for RuggieDrawHandle<'_> {
    fn drop(&mut self) {
        // SAFETY: Calling ffi deinitialization is always save as long as the only way to obtain a
        // RuggieDrawHandle is by calling initialization
        unsafe {
            C3D_FrameEnd(0);
        }
    }
}
