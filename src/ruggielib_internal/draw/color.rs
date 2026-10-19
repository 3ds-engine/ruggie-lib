/// Ruggie's color representation
///
/// Uses RGBA values as an integer between 0 and 255
///
/// # Example
///
/// ```
/// let red = Color::red();
/// let white = Color::white();
/// let custom = Color::new(100, 25, 0, 255);
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Main builder implementation
impl Color {
    /// The main way to build a color with custom RGBA values
    ///
    /// # Example
    ///
    /// ```
    /// let white = Color::new(255, 255, 255, 255);
    /// assert_eq!(white, Color::white());
    /// ```
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Building a color from a custom hex RGBA value
    ///
    /// # Errors
    /// Fails to build a Color if the hex value provided is not parseable into 4 u8 values, one for
    /// each color field
    ///
    /// # Example
    ///
    /// ```
    /// let yellow = Color::from_hex(0xFFFF00FF);
    /// assert_eq!(white, Color::yellow());
    /// ```
    #[must_use]
    pub const fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as u8;
        let g = ((hex >> 16) & 0xFF) as u8;
        let b = ((hex >> 8) & 0xFF) as u8;
        let a = (hex & 0xFF) as u8;

        Self { r, g, b, a }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::from_hex(value)
    }
}

/// Commonly used color implementations
impl Color {
    /// Builds a red color
    ///
    /// # Example
    ///
    /// ```
    /// let red = Color::red();
    /// assert_eq!(red, Color::new(255, 0, 0, 255));
    /// ```
    #[must_use]
    pub const fn red() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    /// Builds a green color
    ///
    /// # Example
    ///
    /// ```
    /// let green = Color::green();
    /// assert_eq!(green, Color::new(0, 255, 0, 255));
    /// ```
    #[must_use]
    pub const fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    /// Builds a blue color
    ///
    /// # Example
    ///
    /// ```
    /// let blue = Color::blue();
    /// assert_eq!(blue, Color::new(0, 0, 255, 255));
    /// ```
    #[must_use]
    pub const fn blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    /// Builds a cyan color
    ///
    /// # Example
    ///
    /// ```
    /// let cyan = Color::cyan();
    /// assert_eq!(cyan, Color::new(0, 255, 255, 255));
    /// ```
    #[must_use]
    pub const fn cyan() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    /// Builds a magenta color
    ///
    /// # Example
    ///
    /// ```
    /// let magenta = Color::magenta();
    /// assert_eq!(magenta, Color::new(255, 0, 255, 255));
    /// ```
    #[must_use]
    pub const fn magenta() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    /// Builds a yellow color
    ///
    /// # Example
    ///
    /// ```
    /// let yellow = Color::yellow();
    /// assert_eq!(yellow, Color::new(255, 255, 0, 255));
    /// ```
    #[must_use]
    pub const fn yellow() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    /// Builds an orange color
    ///
    /// # Example
    ///
    /// ```
    /// let orange = Color::orange();
    /// assert_eq!(orange, Color::new(255, 165, 0, 255));
    /// ```
    #[must_use]
    pub const fn orange() -> Self {
        Self {
            r: 255,
            g: 165,
            b: 0,
            a: 255,
        }
    }

    /// Builds a brown color
    ///
    /// # Example
    ///
    /// ```
    /// let brown = Color::brown();
    /// assert_eq!(brown, Color::new(165, 42, 42, 255));
    /// ```
    #[must_use]
    pub const fn brown() -> Self {
        Self {
            r: 165,
            g: 42,
            b: 42,
            a: 255,
        }
    }

    /// Builds a white color
    ///
    /// # Example
    ///
    /// ```
    /// let white = Color::white();
    /// assert_eq!(white, Color::new(255, 255, 255, 255));
    /// ```
    #[must_use]
    pub const fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    /// Builds a black color
    ///
    /// # Example
    ///
    /// ```
    /// let black = Color::black();
    /// assert_eq!(black, Color::new(0, 0, 0, 255));
    /// ```
    #[must_use]
    pub const fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    /// Builds a pink color
    ///
    /// # Example
    ///
    /// ```
    /// let pink = Color::pink();
    /// assert_eq!(pink, Color::new(255, 192, 203, 255));
    /// ```
    #[must_use]
    pub const fn pink() -> Self {
        Self {
            r: 255,
            g: 192,
            b: 203,
            a: 255,
        }
    }

    /// Builds a gray color
    ///
    /// # Example
    ///
    /// ```
    /// let gray = Color::gray();
    /// assert_eq!(gray, Color::new(128, 128, 128, 255));
    /// ```
    #[must_use]
    pub const fn gray() -> Self {
        Self {
            r: 128,
            g: 128,
            b: 128,
            a: 255,
        }
    }
}
