use crate::draw::sprite::Sprite;
use citro2d_sys::{
    C2D_SpriteFromSheet, C2D_SpriteSheet, C2D_SpriteSheetCount, C2D_SpriteSheetFree,
    C2D_SpriteSheetLoad,
};
use std::ffi::CString;

/// `RuggieLib`'s representation of a `SpriteSheet`
///
/// `SpriteSheets` are collections of Sprites that may contain one or more of them. Sprites are
/// indexed starting from 0. 
///
/// To create an instance of a `SpriteSheet` you will need to enable `RuggieLib`'s `RomFS` feature
///
/// # Example
///
/// ```
/// let rl = RuggieLib::new().unwrap().with([Feature::RomFS]);
/// let sprite_sheet = SpriteSheet::new("romfs:/some-dir/file.t3x");
/// ```
pub struct SpriteSheet {
    sprite_sheet: C2D_SpriteSheet,
    size: usize,
}

/// `SpriteSheet` implementation
impl SpriteSheet {
    /// Creates a new sprite
    ///
    /// To create an instance of a `SpriteSheet` you will need to enable `RuggieLib`'s `RomFS` feature
    ///
    /// # Example
    ///
    /// ```
    /// let rl = RuggieLib::new().unwrap().with([Feature::RomFS]);
    /// let sprite_sheet = SpriteSheet::new("romfs:/some-dir/file.t3x");
    /// ```
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

    /// Obtains a Sprite from a `SpriteSheet`
    ///
    /// This is the only safe way to create a sprite using `RuggieLib`
    ///
    /// # Example
    ///
    /// ```
    /// let sprite_sheet = SpriteSheet::new("romfs:/some/file.t3x").unwrap();
    /// let mut sprite = sprite_sheet.get_sprite(0).unwrap();
    /// ```
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

/// Ensures the internal data of the `SpriteSheet` is freed after dropping
impl Drop for SpriteSheet {
    fn drop(&mut self) {
        unsafe {
            C2D_SpriteSheetFree(self.sprite_sheet);
        }
    }
}
