use std::ptr::{null, null_mut};

use citro2d_sys::{
    C2D_DrawParams, C2D_DrawParams__bindgen_ty_1, C2D_DrawParams__bindgen_ty_2, C2D_Image,
    C2D_Sprite, C2D_SpriteRotate, C2D_SpriteRotateDegrees,
};

pub struct Sprite(C2D_Sprite);

impl Sprite {
    // SAFETY: Just creating a sprite with null fields is safe, it is trying to use the sprite
    // that will be unsafe
    #[must_use]
    pub const unsafe fn default() -> Self {
        Self(C2D_Sprite {
            image: C2D_Image {
                tex: null_mut(),
                subtex: null(),
            },
            params: C2D_DrawParams {
                pos: C2D_DrawParams__bindgen_ty_1 {
                    x: 0.0,
                    y: 0.0,
                    w: 0.0,
                    h: 0.0,
                },
                center: C2D_DrawParams__bindgen_ty_2 { x: 0.0, y: 0.0 },
                depth: 0.0,
                angle: 0.0,
            },
        })
    }

    // SAFETY: Accessing the internal pointer is safe as long as it doesn't mutate
    #[must_use]
    pub const unsafe fn get_raw(&self) -> &C2D_Sprite {
        &self.0
    }

    // SAFETY: Accessing the internal pointer is safe as long as it doesn't mutate
    pub const unsafe fn get_raw_mut(&mut self) -> &mut C2D_Sprite {
        &mut self.0
    }

    pub fn rotate(&mut self, radians: f32) {
        unsafe {
            C2D_SpriteRotate(&mut self.0, radians);
        }
    }

    pub fn rotate_degrees(&mut self, degrees: f32) {
        unsafe {
            C2D_SpriteRotateDegrees(&mut self.0, degrees);
        }
    }
}
