//! Port of qt/src/streaming/InputModels.h
//!
//! Controller button bits and the sampled input frames that the binary input
//! protocol serialises. `GamepadButtons` is a transparent `u16` bitmask (the
//! Qt side is a `QFlags<GamepadButton>`), not an enum, because the wire format
//! carries the raw mask.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// Xbox-style controller button bits carried over the input channel.
///
/// Bit 0 is unused by the protocol (`None = 0`); the first real button is
/// `NEXUS` at bit 1.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
#[repr(transparent)]
pub struct GamepadButtons(pub u16);

impl GamepadButtons {
    pub const NONE: Self = Self(0);
    pub const NEXUS: Self = Self(1 << 1);
    pub const MENU: Self = Self(1 << 2);
    pub const VIEW: Self = Self(1 << 3);
    pub const A: Self = Self(1 << 4);
    pub const B: Self = Self(1 << 5);
    pub const X: Self = Self(1 << 6);
    pub const Y: Self = Self(1 << 7);
    pub const DPAD_UP: Self = Self(1 << 8);
    pub const DPAD_DOWN: Self = Self(1 << 9);
    pub const DPAD_LEFT: Self = Self(1 << 10);
    pub const DPAD_RIGHT: Self = Self(1 << 11);
    pub const LEFT_SHOULDER: Self = Self(1 << 12);
    pub const RIGHT_SHOULDER: Self = Self(1 << 13);
    pub const LEFT_THUMB: Self = Self(1 << 14);
    pub const RIGHT_THUMB: Self = Self(1 << 15);

    /// Every named button, in bit order. Useful for iteration in UIs and tests.
    pub const ALL: [Self; 15] = [
        Self::NEXUS,
        Self::MENU,
        Self::VIEW,
        Self::A,
        Self::B,
        Self::X,
        Self::Y,
        Self::DPAD_UP,
        Self::DPAD_DOWN,
        Self::DPAD_LEFT,
        Self::DPAD_RIGHT,
        Self::LEFT_SHOULDER,
        Self::RIGHT_SHOULDER,
        Self::LEFT_THUMB,
        Self::RIGHT_THUMB,
    ];

    /// Constructs a mask from its raw wire value.
    #[inline]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// The raw wire value.
    #[inline]
    pub const fn bits(self) -> u16 {
        self.0
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// `QFlags::testFlag`: true when every bit of `flag` is set.
    #[inline]
    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0 && (flag.0 != 0 || self.0 == 0)
    }

    /// `QFlags::setFlag`: sets or clears the bits of `flag`.
    #[inline]
    pub fn set(&mut self, flag: Self, on: bool) {
        if on {
            self.0 |= flag.0;
        } else {
            self.0 &= !flag.0;
        }
    }

    /// Returns a copy with the bits of `flag` set or cleared.
    #[inline]
    pub const fn with(self, flag: Self, on: bool) -> Self {
        if on {
            Self(self.0 | flag.0)
        } else {
            Self(self.0 & !flag.0)
        }
    }

    #[inline]
    pub const fn insert(&mut self, flag: Self) {
        self.0 |= flag.0;
    }

    #[inline]
    pub const fn remove(&mut self, flag: Self) {
        self.0 &= !flag.0;
    }

    /// The short name of a single-button mask, `None` for combinations or unknown bits.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::NEXUS => "Nexus",
            Self::MENU => "Menu",
            Self::VIEW => "View",
            Self::A => "A",
            Self::B => "B",
            Self::X => "X",
            Self::Y => "Y",
            Self::DPAD_UP => "DpadUp",
            Self::DPAD_DOWN => "DpadDown",
            Self::DPAD_LEFT => "DpadLeft",
            Self::DPAD_RIGHT => "DpadRight",
            Self::LEFT_SHOULDER => "LeftShoulder",
            Self::RIGHT_SHOULDER => "RightShoulder",
            Self::LEFT_THUMB => "LeftThumb",
            Self::RIGHT_THUMB => "RightThumb",
            _ => return None,
        })
    }
}

impl std::fmt::Debug for GamepadButtons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "GamepadButtons(None)");
        }
        write!(f, "GamepadButtons(")?;
        let mut first = true;
        let mut rest = self.0;
        for button in Self::ALL {
            if self.contains(button) {
                if !first {
                    write!(f, " | ")?;
                }
                first = false;
                write!(f, "{}", button.name().unwrap_or("?"))?;
                rest &= !button.0;
            }
        }
        if rest != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{rest:#06x}")?;
        }
        write!(f, ")")
    }
}

impl BitOr for GamepadButtons {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for GamepadButtons {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for GamepadButtons {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for GamepadButtons {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for GamepadButtons {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl From<u16> for GamepadButtons {
    #[inline]
    fn from(bits: u16) -> Self {
        Self(bits)
    }
}

impl From<GamepadButtons> for u16 {
    #[inline]
    fn from(buttons: GamepadButtons) -> Self {
        buttons.0
    }
}

/// A 2-D float pair (stick axes or trigger pair).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// One sampled controller state ready to serialise into the streaming input protocol.
///
/// Y axes follow the browser/xCloud convention: up is `-1`, down is `+1`
/// (`GamepadHandler::read_frame` performs the flip from GameController semantics).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GamepadInputFrame {
    pub gamepad_index: u8,
    pub buttons: GamepadButtons,
    pub left_thumb: Vec2,
    pub right_thumb: Vec2,
    /// `x` = left trigger, `y` = right trigger, both `0.0..=1.0`.
    pub triggers: Vec2,
}

/// One native keyboard event sent over the `input` data channel (report bit `KEYBOARD`).
///
/// The wire format is three bytes: key kind (`2` = virtual key / `VKey`), pressed flag,
/// and the Windows virtual-key code. Mirrors `xbox-xcloud-player`'s `KeyboardFrame`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyboardFrame {
    /// `1` = Known, `2` = VKey (what we send), `3` = AppCommand, `0` = Unknown.
    pub key_type: u8,
    pub pressed: bool,
    /// Windows virtual-key code (e.g. `0x57` for `W`).
    pub key_code: u8,
}

impl KeyboardFrame {
    /// A Windows virtual-key press/release event (`key_type = 2`).
    pub const fn vkey(key_code: u8, pressed: bool) -> Self {
        Self {
            key_type: 2,
            pressed,
            key_code,
        }
    }
}

/// One native mouse event (report bit `MOUSE`). Mirrors the browser client's
/// `MouseFrame`:absolute or relative coordinates, wheel and button bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MouseFrame {
    /// Absolute position or relative delta X, depending on [`Self::relative`].
    pub x: u32,
    pub y: u32,
    pub wheel_x: u32,
    pub wheel_y: u32,
    /// Bitmask: bit 0 = left, 1 = right, 2 = middle.
    pub buttons: u8,
    /// `1` for relative movement, `0` for absolute.
    pub relative: bool,
}

/// One native pointer/touch event (report bit `POINTER`). Used by the Android
/// client for native touch; coordinates are in server-video pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PointerEventFrame {
    pub x: u32,
    pub y: u32,
    /// Contact pressure `0..=255`.
    pub pressure: u8,
    pub twist: u16,
    /// `1` = down, `2` = up, `3` = move, `0` = cancel.
    pub phase: u8,
}

/// A pointer report containing one or more touch/pen contacts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PointerFrame {
    pub events: Vec<PointerEventFrame>,
}

/// Frame timing markers piggybacked on input packets for server diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameTimingMetadata {
    pub server_data_key: u32,
    pub first_frame_packet_arrival_time_ms: u32,
    pub frame_submitted_time_ms: u32,
    pub frame_decoded_time_ms: u32,
    pub frame_rendered_time_ms: u32,
}

/// How a trigger's analog value and digital "pressed" flag are combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TriggerInterpretationMode {
    /// Learn per side whether the controller reports analog travel; fall back
    /// to digital when it only ever reports pressed-with-zero-travel.
    #[default]
    Auto,
    /// Analog value, but a digital press counts as a full pull.
    DigitalFallback,
    /// Analog value only; the pressed flag is ignored.
    AnalogOnly,
}

/// User-configurable controller input settings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerSettings {
    /// Radial deadzone radius (0.0 - 1.0). Stick values below this are zeroed.
    pub deadzone: f32,
    pub invert_y: bool,
    pub swap_ab: bool,
    /// 0.5 means a half pull reaches full output (hair trigger).
    pub trigger_sensitivity: f32,
    pub trigger_interpretation_mode: TriggerInterpretationMode,
    pub vibration_intensity: f32,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            deadzone: 0.15,
            invert_y: false,
            swap_ab: false,
            trigger_sensitivity: 0.5,
            trigger_interpretation_mode: TriggerInterpretationMode::Auto,
            vibration_intensity: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_bits_match_protocol() {
        assert_eq!(GamepadButtons::NEXUS.bits(), 2);
        assert_eq!(GamepadButtons::MENU.bits(), 4);
        assert_eq!(GamepadButtons::VIEW.bits(), 8);
        assert_eq!(GamepadButtons::A.bits(), 0x10);
        assert_eq!(GamepadButtons::B.bits(), 0x20);
        assert_eq!(GamepadButtons::X.bits(), 0x40);
        assert_eq!(GamepadButtons::Y.bits(), 0x80);
        assert_eq!(GamepadButtons::DPAD_UP.bits(), 0x100);
        assert_eq!(GamepadButtons::DPAD_DOWN.bits(), 0x200);
        assert_eq!(GamepadButtons::DPAD_LEFT.bits(), 0x400);
        assert_eq!(GamepadButtons::DPAD_RIGHT.bits(), 0x800);
        assert_eq!(GamepadButtons::LEFT_SHOULDER.bits(), 0x1000);
        assert_eq!(GamepadButtons::RIGHT_SHOULDER.bits(), 0x2000);
        assert_eq!(GamepadButtons::LEFT_THUMB.bits(), 0x4000);
        assert_eq!(GamepadButtons::RIGHT_THUMB.bits(), 0x8000);
        assert_eq!((GamepadButtons::A | GamepadButtons::B).bits(), 0x30);
    }

    #[test]
    fn test_flag_and_set_flag_mirror_qflags() {
        let mut buttons = GamepadButtons::A | GamepadButtons::MENU;
        assert!(buttons.contains(GamepadButtons::A));
        assert!(!buttons.contains(GamepadButtons::B));
        assert!(buttons.contains(GamepadButtons::A | GamepadButtons::MENU));
        assert!(!buttons.contains(GamepadButtons::A | GamepadButtons::B));
        // QFlags::testFlag(None) is true only for an empty mask.
        assert!(!buttons.contains(GamepadButtons::NONE));
        assert!(GamepadButtons::NONE.contains(GamepadButtons::NONE));

        buttons.set(GamepadButtons::A, false);
        buttons.set(GamepadButtons::B, true);
        assert_eq!(buttons, GamepadButtons::B | GamepadButtons::MENU);
    }

    #[test]
    fn debug_lists_button_names() {
        let buttons = GamepadButtons::MENU | GamepadButtons::VIEW;
        assert_eq!(format!("{buttons:?}"), "GamepadButtons(Menu | View)");
        assert_eq!(
            format!("{:?}", GamepadButtons::NONE),
            "GamepadButtons(None)"
        );
        assert_eq!(
            format!("{:?}", GamepadButtons(0x0011)),
            "GamepadButtons(A | 0x0001)"
        );
    }

    #[test]
    fn controller_settings_defaults_match_cpp() {
        let settings = ControllerSettings::default();
        assert_eq!(settings.deadzone, 0.15);
        assert!(!settings.invert_y);
        assert!(!settings.swap_ab);
        assert_eq!(settings.trigger_sensitivity, 0.5);
        assert_eq!(
            settings.trigger_interpretation_mode,
            TriggerInterpretationMode::Auto
        );
        assert_eq!(settings.vibration_intensity, 1.0);
    }
}
