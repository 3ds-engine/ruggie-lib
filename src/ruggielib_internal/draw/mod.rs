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

/// The API for 2D rendering in the 3DS
///
/// It holds functionality to draw 2D shapes, 2D outlines, 2D sprites, 2D primitives and clearing
/// the screen.
///
/// This struct operates like a state machine. When it's created it is rendering on the top-left
/// screen. You can use functions named `draw_<screen>()` to switch the target screen for rendering
///
/// # Example
/// ```
/// let rl = RuggieLib::new().unwrap
/// while(rl.is_running()) {
///     rl.wait_for_vblank();
///     let render = rl.start_drawing();
///
///     // Top left screen
///     rl.clear_screen(Color::white());
///     rl.draw_circle(100.0, 100.0, 50.0, Color::red()); 
///
///     // Bottom Screen
///     rl.draw_bottom();
///     rl.clear_screen(Color::white())
/// }
/// ```
pub struct RuggieDrawHandle<'a> {
    lib: &'a mut RuggieLib,
    current_screen: *const RuggieScreenTarget,
}

/// Creation of the Handle
///
/// This is meant to be accessed from RuggieLib's `start_drawing()` method
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

/// Private utilities
#[doc(hidden)]
impl RuggieDrawHandle<'_> {
    fn c2d_color(color: Color) -> u32 {
        unsafe {
            // SAFETY: This function does simple arithmetic to get a u32 from the four u8 fields of
            // a color. Calling it is always safe
            C2D_Color32(color.r, color.g, color.b, color.a)
        }
    }
}

/// Screen switching and information
impl RuggieDrawHandle<'_> {
    /// Switches the target screen for all render functionality to be the top left screen
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

    /// Switches the target screen for all render functionality to be the top right screen
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

    /// Switches the target screen for all render functionality to be the bottom screen
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

    /// Gets the width of the target screen
    #[must_use]
    pub const fn screen_witdh(&self) -> u32 {
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe { self.current_screen.as_ref_unchecked().width() }
    }

    /// Gets the height of the target screen
    #[must_use]
    pub const fn screen_height(&self) -> u32 {
        // SAFETY: Dereferencing the current screen is safe because inside RuggieLib, screens
        // never mutate after they are built, so using them while having a valid mutable
        // reference to RuggieLib ensures the pointer is valid and can't be mutated
        unsafe { self.current_screen.as_ref_unchecked().height() }
    }

    /// Gets the dimensions of the target screen in a (width, height) tuple
    ///
    /// # Example
    ///
    /// ```
    /// let (witdh, height) = render.screen_dimensions();
    /// assert_eq!(width, render.screen_width());
    /// assert_eq!(height, render.screen_height());
    /// ```
    #[must_use]
    pub const fn screen_dimensions(&self) -> (u32, u32) {
        (self.screen_witdh(), self.screen_height())
    }
}

// Screen clearing
impl RuggieDrawHandle<'_> {
    /// Clears the target screen with the given render color
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

/// 2D Primitives
impl RuggieDrawHandle<'_> {
    /// Draws a line from point (x0, y0) to point (x1, y1), with a thickness of `thickness` pixels
    /// and a given color
    pub fn draw_line(&self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: u16, color: Color) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawLine(x0, y0, color, x1, y1, color, thickness.into(), 0.0);
        }
    }
}

// 2D Shapes
impl RuggieDrawHandle<'_> {
    /// Draws a rectangle.
    ///
    /// Its top-left corner will be situated at (x, y)
    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
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
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawTriangle(x0, y0, color, x1, y1, color, x2, y2, color, 0.0);
        }
    }

    /// Draws a circle.
    ///
    /// Its center will be situated at (x, y)
    pub fn draw_circle(&self, x: f32, y: f32, r: f32, color: Color) {
        let color = Self::c2d_color(color);

        // SAFETY: The only way to call this ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
        unsafe {
            C2D_DrawCircleSolid(x, y, 0.0, r, color);
        }
    }

    /// Draws an ellipse
    ///
    /// Its center will be situated at (x, y), rx will be it's x-axis radius and ry will be its
    /// y-axis radius
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

/// 2D Outlines
impl RuggieDrawHandle<'_> {
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
        let color = Self::c2d_color(color);
        // SAFETY: The only way to call these ffi function is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
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
        let color = Self::c2d_color(color);
        let angle_step = 2.0 * std::f32::consts::PI / f32::from(sides);

        // SAFETY: The only way to call these ffi functions is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
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
        let color = Self::c2d_color(color);
        let angle_step = 2.0 * std::f32::consts::PI / f32::from(sides);

        // SAFETY: The only way to call these ffi functions is by creating an instance of
        // RuggieDrawHandle and, therefore, calling the citro2d and citro3d initializers
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

/// 2D Sprites
impl RuggieDrawHandle<'_> {
    /// Draws a sprite
    ///
    /// The position of its pivot will be located at (x, y). You can change the pivot of a sprite
    /// using `set_pivot()`
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

/// Destruction
///
/// This ensures that the resources acquired by RuggieDrawHandle are freed when it is dropped
impl Drop for RuggieDrawHandle<'_> {
    fn drop(&mut self) {
        // SAFETY: Calling ffi deinitialization is always save as long as the only way to obtain a
        // RuggieDrawHandle is by calling initialization
        unsafe {
            C3D_FrameEnd(0);
        }
    }
}
