use std::ffi::CString;

use citro2d_sys::{C2D_SpriteFromSheet, C2D_SpriteSheet, C2D_SpriteSheetLoad};

use crate::draw::sprite::Sprite;
pub struct SpriteSheet(C2D_SpriteSheet);

impl SpriteSheet {
    #[must_use]
    pub fn new(filename: &str) -> Option<Self> {
        let filename_cstr = CString::new(filename).ok()?;
        unsafe {
            let sprite_sheet = C2D_SpriteSheetLoad(filename_cstr.as_ptr());

            if sprite_sheet.is_null() {
                None
            } else {
                Some(Self(sprite_sheet))
            }
        }
    }

    #[must_use]
    pub fn get_sprite(&self, index: usize) -> Sprite {
        unsafe {
            let mut sprite = Sprite::default();
            C2D_SpriteFromSheet(sprite.get_raw_mut(), self.0, index);

            sprite
        }
    }
}
