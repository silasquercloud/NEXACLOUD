//! Maps egui keys to Windows virtual-key codes for xCloud native keyboard input.
//!
//! The xCloud `KEYBOARD` input report carries a Windows VKey byte; this table is
//! the bridge between a desktop key event and that byte. Values come from the
//! Win32 `VK_*` constants (the same ones the browser client sends).

use egui::Key;

/// Converts an egui key into the Windows virtual-key code xCloud expects.
/// Returns `None` for keys with no meaningful desktop VKey (media, launch, IME).
pub fn egui_key_to_vkey(key: Key) -> Option<u8> {
    use Key::*;
    Some(match key {
        // Control / navigation
        Backspace => 0x08,
        Tab => 0x09,
        Enter => 0x0D,
        ShiftLeft | ShiftRight => 0x10,
        ControlLeft | ControlRight => 0x11,
        AltLeft | AltRight => 0x12,
        Escape => 0x1B,
        Space => 0x20,
        PageUp => 0x21,
        PageDown => 0x22,
        End => 0x23,
        Home => 0x24,
        ArrowLeft => 0x25,
        ArrowUp => 0x26,
        ArrowRight => 0x27,
        ArrowDown => 0x28,
        Insert => 0x2D,
        Delete => 0x2E,
        // Digits 0-9
        Num0 => 0x30,
        Num1 => 0x31,
        Num2 => 0x32,
        Num3 => 0x33,
        Num4 => 0x34,
        Num5 => 0x35,
        Num6 => 0x36,
        Num7 => 0x37,
        Num8 => 0x38,
        Num9 => 0x39,
        // Letters A-Z
        A => 0x41,
        B => 0x42,
        C => 0x43,
        D => 0x44,
        E => 0x45,
        F => 0x46,
        G => 0x47,
        H => 0x48,
        I => 0x49,
        J => 0x4A,
        K => 0x4B,
        L => 0x4C,
        M => 0x4D,
        N => 0x4E,
        O => 0x4F,
        P => 0x50,
        Q => 0x51,
        R => 0x52,
        S => 0x53,
        T => 0x54,
        U => 0x55,
        V => 0x56,
        W => 0x57,
        X => 0x58,
        Y => 0x59,
        Z => 0x5A,
        // Windows / Meta
        SuperLeft | SuperRight => 0x5B,
        // Function keys F1-F24
        F1 => 0x70,
        F2 => 0x71,
        F3 => 0x72,
        F4 => 0x73,
        F5 => 0x74,
        F6 => 0x75,
        F7 => 0x76,
        F8 => 0x77,
        F9 => 0x78,
        F10 => 0x79,
        F11 => 0x7A,
        F12 => 0x7B,
        F13 => 0x7C,
        F14 => 0x7D,
        F15 => 0x7E,
        F16 => 0x7F,
        F17 => 0x80,
        F18 => 0x81,
        F19 => 0x82,
        F20 => 0x83,
        F21 => 0x84,
        F22 => 0x85,
        F23 => 0x86,
        F24 => 0x87,
        // Lock keys
        CapsLock => 0x14,
        NumLock => 0x90,
        // Punctuation (US layout)
        Semicolon => 0xBA,
        Equals => 0xBB,
        Comma => 0xBC,
        Minus => 0xBD,
        Period => 0xBE,
        Slash => 0xBF,
        Backtick => 0xC0,
        OpenBracket => 0xDB,
        Backslash => 0xDC,
        CloseBracket => 0xDD,
        Quote => 0xDE,
        // Shifted punctuation variants map to the same physical VKey
        Colon => 0xBA,
        Plus => 0xBB,
        Questionmark => 0xBF,
        Exclamationmark => 0x31,
        OpenCurlyBracket => 0xDB,
        CloseCurlyBracket => 0xDD,
        Pipe => 0xDC,
        // Everything else (media keys, IME, launcher keys) has no game VKey.
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasd_and_space_map_to_vkeys() {
        assert_eq!(egui_key_to_vkey(Key::W), Some(0x57));
        assert_eq!(egui_key_to_vkey(Key::A), Some(0x41));
        assert_eq!(egui_key_to_vkey(Key::S), Some(0x53));
        assert_eq!(egui_key_to_vkey(Key::D), Some(0x44));
        assert_eq!(egui_key_to_vkey(Key::Space), Some(0x20));
    }

    #[test]
    fn f1_and_escape_map_to_vkeys() {
        assert_eq!(egui_key_to_vkey(Key::F1), Some(0x70));
        assert_eq!(egui_key_to_vkey(Key::Escape), Some(0x1B));
    }

    #[test]
    fn digits_and_function_keys_map() {
        assert_eq!(egui_key_to_vkey(Key::Num1), Some(0x31));
        assert_eq!(egui_key_to_vkey(Key::F12), Some(0x7B));
    }
}
