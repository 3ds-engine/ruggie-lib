pub mod color;
pub mod sprite;
pub mod sprite_sheet;

use citro2d_sys::{
    C2D_Color32, C2D_DrawCircleSolid, C2D_DrawEllipseSolid, C2D_DrawLine, C2D_DrawRectSolid,
    C2D_DrawSprite, C2D_DrawTriangle, C2D_SceneBegin, C2D_SpriteSetPos, C2D_TargetClear,
};
use citro3d_sys::{C3D_FRAME_SYNCDRAW, C3D_FrameBegin, C3D_FrameEnd};

use crate::{RuggieLib, draw::sprite::Sprite};
use color::Color;

impl RuggieLib {
    /// Initialize the drawing capabilities for this frame
    ///
    /// # Important
    /// This function **must be called before using any functionality of the library that renders to
    /// the screen**, otherwise each and every one of those function calls will be undefined behaviour
    ///
    /// Also, if this function is called, end_drawing must be called after the last drawing
    /// instruction. It is recommended to wrap this calls in a struct that manages them using
    /// lifetimes, via implementing the Default and Drop traits
    ///
    /// # Example
    /// ```
    /// let rl = RuggieLib::new().unwrap();
    /// while(rl.is_running()) {
    ///     rl.wait_for_vblank();
    ///
    ///     // Rendering loop starts (does not need to be a block)
    ///     {
    ///         rl.begin_drawing();
    ///         render.clear_screen(Color::white());
    ///         render.draw_rectangle(0.0, 0.0, 100.0, 100.0, Color::red());
    ///         rl.end_drawing() // This is mandatory
    ///     }
    /// }
    /// ```
    pub fn begin_drawing(&self) {
        // SAFETY: Calling ffi initialization functions is safe as long as they are deinitialized
        // after a call to end_drawing
        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW);

            // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
            // never mutate after they are built, so using them while having a valid mutable
            // reference to RuggieLib ensures the pointer is valid and can't be mutated
            C2D_SceneBegin(self.screens[self.current_screen_index].get_target_raw_mut());
        }
    }

    /// Present to the screen and unitiliaze the rendering capabilities
    ///
    /// # Important
    /// This function should **only be called after calling begin_drawing**, otherwise it will
    /// trigger undefined behaviour
    pub fn end_drawing(&self) {
        // SAFETY: Calling ffi deinitialization is only safe as long as begin_drawing was called
        // before calling this function, otherwise it is UNDEFINED BEHAVIOUR
        unsafe {
            C3D_FrameEnd(0);
        }
    }

    /// Waits until screen renderer is drawing vblank.
    ///
    /// Used to apply vsync to frame rendering. Should be used before rendering anything to screen
    ///
    /// # Example
    ///
    /// ```
    /// let rl = RuggieLib::new().unwrap();
    /// while(rl.is_running()) {
    ///     rl.wait_for_blank();
    ///     rl.begin_drawing();
    ///     // Your render code here
    ///     rl.end_drawing();
    /// }
    /// ```
    pub fn wait_for_vblank(&self) {
        self.gfx.wait_for_vblank();
    }
}

#[doc(hidden)]
fn c2d_color(color: Color) -> u32 {
    unsafe {
        // SAFETY: This function does simple arithmetic to get a u32 from the four u8 fields of
        // a color. Calling it is always safe
        C2D_Color32(color.r, color.g, color.b, color.a)
    }
}

impl RuggieLib {
    #[doc(hidden)]
    unsafe fn scene_begin(&self) {
        // SAFETY: Dereferencing the current screen is safe as long as begin_drawing was called 
        // because inside RuggieLib, screens never mutate after they are built
        unsafe {
            C2D_SceneBegin(self.screens[self.current_screen_index].get_target_raw_mut());
        }
    }

    /// Switches the target screen for all render functionality to be the top left screen
    pub fn draw_top_left(&mut self) {
        self.current_screen_index = 0usize;
        unsafe {
            self.scene_begin();
        }
    }

    /// Switches the target screen for all render functionality to be the top right screen
    pub fn draw_top_right(&mut self) {
        self.current_screen_index = 1usize;
        unsafe {
            self.scene_begin();
        }
    }

    /// Switches the target screen for all render functionality to be the bottom screen
    pub fn draw_bottom(&mut self) {
        self.current_screen_index = 2usize;
        unsafe {
            self.scene_begin();
        }
    }
}

impl RuggieLib {
    /// Clears the target screen with the given render color
    pub fn clear_screen(&mut self, clear_color: Color) {
        unsafe {
            // SAFETY: Calling this ffi function is safe as long as begin_drawing was called.
            // Otherwise it is undefined behaviour
            C2D_TargetClear(
                // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
                // never mutate after they are built                
                self.screens[self.current_screen_index].get_target_raw_mut(),
                C2D_Color32(clear_color.r, clear_color.g, clear_color.b, clear_color.a),
            );
        }
    }
}

impl RuggieLib {
    /// Draws a line from point (x0, y0) to point (x1, y1), with a thickness of `thickness` pixels
    /// and a given color
    pub fn draw_line(&self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: u16, color: Color) {
        let color = c2d_color(color);

        // SAFETY: Calling this ffi function is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
        }
    }
}

impl RuggieLib {
    /// Draws a rectangle.
    ///
    /// Its top-left corner will be situated at (x, y)
    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let color = c2d_color(color);

        // SAFETY: Calling this ffi function is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            C2D_DrawRectSolid(x, y, 0.0, w, h, color);
        }
    }

    /// Draws a triangle.
    ///
    /// The three vertices of the triangle will be situated
    /// at (x0, y0), (x1, y1) and (x2, y2).
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
        let color = c2d_color(color);

        // SAFETY: Calling this ffi function is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            C2D_DrawTriangle(x0, y0, color, x1, y1, color, x2, y2, color, 0.0);
        }
    }

    /// Draws a circle.
    ///
    /// Its center will be situated at (x, y)
    pub fn draw_circle(&self, x: f32, y: f32, r: f32, color: Color) {
        let color = c2d_color(color);

        // SAFETY: Calling this ffi function is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            C2D_DrawCircleSolid(x, y, 0.0, r, color);
        }
    }

    /// Draws an ellipse
    ///
    /// Its center will be situated at (x, y), rx will be it's x-axis radius and ry will be its
    /// y-axis radius
    pub fn draw_ellipse(&self, x: f32, y: f32, rx: f32, ry: f32, color: Color) {
        let color = c2d_color(color);

        // SAFETY: Calling this ffi function is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
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

impl RuggieLib {
    /// Draws the outline of a rectangle
    ///
    /// The logistics of its position are the same that its full shape analog
    pub fn draw_rectangle_outline(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        thickness: u16,
        color: Color,
    ) {
        let color = c2d_color(color);
        // SAFETY: Calling these ffi functions is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            C2D_DrawLine(x, y, color, x + w, y, color, thickness.into(), 0.0);
            C2D_DrawLine(x + w, y, color, x + w, y + h, color, thickness.into(), 0.0);
            C2D_DrawLine(x + w, y + h, color, x, y + h, color, thickness.into(), 0.0);
            C2D_DrawLine(x, y + h, color, x, y, color, thickness.into(), 0.0);
        }
    }

    /// Draws the outline of a triangle
    ///
    /// The logistics of its position are the same that its full shape analog
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
        let color = c2d_color(color);
        // SAFETY: Calling these ffi functions is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
            C2D_DrawLine(x1, y1, color, x2, y2, color, thickness.into(), 0.0);
            C2D_DrawLine(x2, y2, color, x0, y0, color, thickness.into(), 0.0);
        }
    }

    /// Draws the outline of a circle
    ///
    /// The logistics of its position are the same that its full shape analog
    pub fn draw_circle_outline(
        &self,
        x: f32,
        y: f32,
        r: f32,
        sides: u16,
        thickness: u16,
        color: Color,
    ) {
        let color = c2d_color(color);
        let angle_step = 2.0 * std::f32::consts::PI / f32::from(sides);

        // SAFETY: Calling these ffi functions is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            for i in 0..sides {
                let a0 = angle_step * f32::from(i);
                let a1 = angle_step * (f32::from(i) + 1.0);

                let x0 = r.mul_add(x, a0.cos());
                let y0 = r.mul_add(y, a0.sin());
                let x1 = r.mul_add(x, a1.cos());
                let y1 = r.mul_add(y, a1.sin());

                C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
            }
        }
    }

    /// Draws the outline of an ellipse
    ///
    /// The logistics of its position are the same that its full shape analog
    pub fn draw_ellipse_outline(
        &self,
        x: f32,
        y: f32,
        rx: f32,
        ry: f32,
        sides: u16,
        thickness: u16,
        color: Color,
    ) {
        let color = c2d_color(color);
        let angle_step = 2.0 * std::f32::consts::PI / f32::from(sides);

        // SAFETY: Calling these ffi functions is safe as long as begin_drawing was called.
        // Otherwise it is undefined behaviour
        unsafe {
            for i in 0..sides {
                let a0 = angle_step * f32::from(i);
                let a1 = angle_step * (f32::from(i) + 1.0);

                let x0 = rx.mul_add(x + rx / 2.0, a0.cos());
                let y0 = ry.mul_add(y + ry / 2.0, a0.sin());
                let x1 = rx.mul_add(x + rx / 2.0, a1.cos());
                let y1 = ry.mul_add(y + ry / 2.0, a1.sin());

                C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
            }
        }
    }
}

impl RuggieLib {
    pub fn draw_sprite(&self, sprite: &mut Sprite, x: f32, y: f32) {
        unsafe {
            // SAFETY: Dereferencing a raw sprite can be unsafe, only if the user creates the raw
            // sprite themselves, otherwise it is impossible to create an invalid sprite
            let raw_sprite = sprite.get_raw_mut();

            // SAFETY: Calling these ffi functions is safe as long as begin_drawing was called.
            // Otherwise it is undefined behaviour
            C2D_SpriteSetPos(raw_sprite, x, y);
            C2D_DrawSprite(raw_sprite);
        }
    }
}
