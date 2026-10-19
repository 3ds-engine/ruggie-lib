use ctru::services::hid::KeyPad;

///Button codes
pub enum Button{
    /// A button.
    A,
    /// B button.
    B,
    /// Select button.
    Select,
    /// Start button.
    Start,
    /// D-Pad Right.
    DPadRight,
    /// D-Pad Left.
    DPadLeft,
    /// D-Pad Up.
    DPadUp,
    /// D-Pad Down.
    DPadDown,
    /// R button.
    R,
    /// L button.
    L,
    /// X button.
    X,
    /// Y button.
    Y,
    /// ZL button.
    ZL,
    /// ZR button.
    ZR
}

impl From<Button> for KeyPad {
    fn from(button: Button) -> Self {
        match button {
            Button::A         => Self::A,
            Button::B         => Self::B,
            Button::Select    => Self::SELECT,
            Button::Start     => Self::START,
            Button::DPadRight => Self::DPAD_RIGHT,
            Button::DPadLeft  => Self::DPAD_LEFT,
            Button::DPadUp    => Self::DPAD_UP,
            Button::DPadDown  => Self::DPAD_DOWN,
            Button::R         => Self::R,
            Button::L         => Self::L,
            Button::X         => Self::X,
            Button::Y         => Self::Y,
            Button::ZL        => Self::ZL,
            Button::ZR        => Self::ZR,
        }
    }
}
