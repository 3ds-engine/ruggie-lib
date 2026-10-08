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
            Button::A         => KeyPad::A,
            Button::B         => KeyPad::B,
            Button::Select    => KeyPad::SELECT,
            Button::Start     => KeyPad::START,
            Button::DPadRight => KeyPad::DPAD_RIGHT,
            Button::DPadLeft  => KeyPad::DPAD_LEFT,
            Button::DPadUp    => KeyPad::DPAD_UP,
            Button::DPadDown  => KeyPad::DPAD_DOWN,
            Button::R         => KeyPad::R,
            Button::L         => KeyPad::L,
            Button::X         => KeyPad::X,
            Button::Y         => KeyPad::Y,
            Button::ZL        => KeyPad::ZL,
            Button::ZR        => KeyPad::ZR,
        }
    }
}