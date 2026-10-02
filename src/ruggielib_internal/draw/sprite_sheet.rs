use citro2d_sys::*;
use std::ffi::CString;

use crate::draw::sprite::Sprite;
pub struct SpriteSheet(C2D_SpriteSheet);

impl SpriteSheet {
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

    pub fn get_sprite(&self, index: usize) -> Sprite {
        unsafe {
            let sprite = Sprite::default();
            C2D_SpriteFromSheet(&mut sprite.get_raw(), self.0, index);

            sprite
        }
    }
}
