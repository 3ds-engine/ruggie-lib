use citro2d_sys::{
    C2D_Sprite, C2D_SpriteRotate, C2D_SpriteRotateDegrees, C2D_SpriteSetCenter, C2D_SpriteSetScale,
};

/// Ruggie's sprite representation
///
/// They can only be safely obtained from `SpriteSheets`, they are used to render images represented
/// as a 2D plane
pub struct Sprite(C2D_Sprite);

/// Unsafe functionality
#[doc(hidden)]
impl Sprite {
    // SAFETY: Just creating a sprite with null fields is safe, it is trying to use the sprite
    // that will be unsafe
    #[must_use]
    pub const unsafe fn zeroed() -> Self {
        unsafe { std::mem::zeroed() }
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
}

/// Sprite transformation
impl Sprite {
    /// Rotates a sprite a set amount of radians relative to its current rotation
    pub fn rotate(&mut self, radians: f32) {
        // SAFETY: This is only unsafe if ruggielib has not been created, and it should be the first
        // thing to happen in a program
        unsafe {
            C2D_SpriteRotate(&raw mut self.0, radians);
        }
    }

    /// Rotates a sprite a set amount of degrees relative to its current rotation
    pub fn rotate_degrees(&mut self, degrees: f32) {
        // SAFETY: This is only unsafe if ruggielib has not been created, and it should be the first
        // thing to happen in a program
        unsafe {
            C2D_SpriteRotateDegrees(&raw mut self.0, degrees);
        }
    }

    /// Sets the point from where the image will be positioned, scaled and rotated
    ///
    /// A pivot point is represented with two coordenate values ranging from 0 to 1, where (0, 0) is
    /// the top left corner of the image, (0.5, 0.5) is the center of the image and (1, 1) is the
    /// bottom right corner of the image.
    ///
    /// Values for x and y must be in the [0.0, 1.0] range or else the pivot won't be set. If the
    /// pivot has switched succesfully this function returns true, otherwise it returns false.
    pub fn set_pivot(&mut self, x: f32, y: f32) -> bool {
        if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
            return false;
        }

        // SAFETY: This is only unsafe if ruggielib has not been created, and it should be the first
        // thing to happen in a program
        unsafe {
            C2D_SpriteSetCenter(&raw mut self.0, x, y);
        }

        true
    }

    /// Changes the scale of the image
    ///
    /// This scaling is absolute, meaning that a Sprite who's scale is set to (1.0, 1.0) will always
    /// have the same width and height no matter what transformations it suffered.
    ///
    /// # Example 
    /// ```
    /// let sprite_sheet = SpriteSheet::new("romfs:/some/file.t3x").unwrap();
    /// let mut sprite = sprite_sheet.get_sprite(0).unwrap();
    ///
    /// let (x, y) = sprite.get_size();
    /// sprite.set_scale(0.5, 0.5);
    /// assert_eq!(x, sprite.get_width() * 2.0);
    /// assert_eq!(y, sprite.get_height() * 2.0);
    /// ```
    pub fn set_scale(&mut self, width: f32, height: f32) {
        // SAFETY: This is only unsafe if ruggielib has not been created, and it should be the first
        // thing to happen in a program
        unsafe {
            C2D_SpriteSetScale(&raw mut self.0, width, height);
        }
    }

    /// Get the size of a Sprite
    ///
    /// The size will be returned in a tuple with the shape (width, height)
    #[must_use]
    pub const fn get_size(&self) -> (f32, f32) {
        (self.0.params.pos.w, self.0.params.pos.h)
    }

    /// Get the width of a Sprite
    #[must_use]
    pub const fn get_width(&self) -> f32 {
        self.0.params.pos.w
    }

    /// Get the height of a Sprite
    #[must_use]
    pub const fn get_height(&self) -> f32 {
        self.0.params.pos.h
    }
}
