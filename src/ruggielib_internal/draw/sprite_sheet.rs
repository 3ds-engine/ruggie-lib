use std::ffi::CString;

use citro2d_sys::{
    C2D_SpriteFromSheet, C2D_SpriteSheet, C2D_SpriteSheetCount, C2D_SpriteSheetFree, C2D_SpriteSheetLoad,
};

use crate::draw::sprite::Sprite;
pub struct SpriteSheet {
    sprite_sheet: C2D_SpriteSheet,
    size: usize,
}

impl SpriteSheet {
    #[must_use]
    pub fn new(filename: &str) -> Option<Self> {
        let filename_cstr = CString::new(filename).ok()?;
        // SAFETY: This is only unsafe if ruggielib has not been created, and it should be the first
        // thing to happen in a program
        unsafe {
            let sprite_sheet = C2D_SpriteSheetLoad(filename_cstr.as_ptr());

            if sprite_sheet.is_null() {
                None
            } else {
                Some(Self {
                    sprite_sheet,
                    size: C2D_SpriteSheetCount(sprite_sheet),
                })
            }
        }
    }

    #[must_use]
    pub fn get_sprite(&self, index: usize) -> Option<Sprite> {
        if index >= self.size {
            return None;
        }

        // SAFETY: This is only unsafe if ruggielib has not been created, and it should be the first
        // thing to happen in a program
        unsafe {
            let mut sprite = Sprite::zeroed();
            C2D_SpriteFromSheet(sprite.get_raw_mut(), self.sprite_sheet, index);

            Some(sprite)
        }
    }
}

impl Drop for SpriteSheet {
    fn drop(&mut self) {
        unsafe {
            C2D_SpriteSheetFree(self.sprite_sheet);
        }
    }
}
