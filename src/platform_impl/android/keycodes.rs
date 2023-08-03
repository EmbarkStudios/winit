use android_activity::{
    input::{KeyAction, KeyEvent, KeyMapChar, Keycode},
    AndroidApp,
};

use crate::event::{self, VirtualKeyCode};

pub fn to_physical_keycode(keycode: Keycode) -> Option<event::VirtualKeyCode> {
    match keycode {
        Keycode::A => Some(VirtualKeyCode::A),
        Keycode::B => Some(VirtualKeyCode::B),
        Keycode::C => Some(VirtualKeyCode::C),
        Keycode::D => Some(VirtualKeyCode::D),
        Keycode::E => Some(VirtualKeyCode::E),
        Keycode::F => Some(VirtualKeyCode::F),
        Keycode::G => Some(VirtualKeyCode::G),
        Keycode::H => Some(VirtualKeyCode::H),
        Keycode::I => Some(VirtualKeyCode::I),
        Keycode::J => Some(VirtualKeyCode::J),
        Keycode::K => Some(VirtualKeyCode::K),
        Keycode::L => Some(VirtualKeyCode::L),
        Keycode::M => Some(VirtualKeyCode::M),
        Keycode::N => Some(VirtualKeyCode::N),
        Keycode::O => Some(VirtualKeyCode::O),
        Keycode::P => Some(VirtualKeyCode::P),
        Keycode::Q => Some(VirtualKeyCode::Q),
        Keycode::R => Some(VirtualKeyCode::R),
        Keycode::S => Some(VirtualKeyCode::S),
        Keycode::T => Some(VirtualKeyCode::T),
        Keycode::U => Some(VirtualKeyCode::U),
        Keycode::V => Some(VirtualKeyCode::V),
        Keycode::W => Some(VirtualKeyCode::W),
        Keycode::X => Some(VirtualKeyCode::X),
        Keycode::Y => Some(VirtualKeyCode::Y),
        Keycode::Z => Some(VirtualKeyCode::Z),

        Keycode::Keycode0 => Some(VirtualKeyCode::Key0),
        Keycode::Keycode1 => Some(VirtualKeyCode::Key1),
        Keycode::Keycode2 => Some(VirtualKeyCode::Key2),
        Keycode::Keycode3 => Some(VirtualKeyCode::Key3),
        Keycode::Keycode4 => Some(VirtualKeyCode::Key4),
        Keycode::Keycode5 => Some(VirtualKeyCode::Key5),
        Keycode::Keycode6 => Some(VirtualKeyCode::Key6),
        Keycode::Keycode7 => Some(VirtualKeyCode::Key7),
        Keycode::Keycode8 => Some(VirtualKeyCode::Key8),
        Keycode::Keycode9 => Some(VirtualKeyCode::Key9),

        Keycode::Numpad0 => Some(VirtualKeyCode::Numpad0),
        Keycode::Numpad1 => Some(VirtualKeyCode::Numpad1),
        Keycode::Numpad2 => Some(VirtualKeyCode::Numpad2),
        Keycode::Numpad3 => Some(VirtualKeyCode::Numpad3),
        Keycode::Numpad4 => Some(VirtualKeyCode::Numpad4),
        Keycode::Numpad5 => Some(VirtualKeyCode::Numpad5),
        Keycode::Numpad6 => Some(VirtualKeyCode::Numpad6),
        Keycode::Numpad7 => Some(VirtualKeyCode::Numpad7),
        Keycode::Numpad8 => Some(VirtualKeyCode::Numpad8),
        Keycode::Numpad9 => Some(VirtualKeyCode::Numpad9),

        Keycode::NumpadAdd => Some(VirtualKeyCode::NumpadAdd),
        Keycode::NumpadSubtract => Some(VirtualKeyCode::NumpadSubtract),
        Keycode::NumpadMultiply => Some(VirtualKeyCode::NumpadMultiply),
        Keycode::NumpadDivide => Some(VirtualKeyCode::NumpadDivide),
        Keycode::NumpadEnter => Some(VirtualKeyCode::NumpadEnter),
        Keycode::NumpadEquals => Some(VirtualKeyCode::NumpadEquals),
        Keycode::NumpadComma => Some(VirtualKeyCode::NumpadComma),
        Keycode::NumpadDot => Some(VirtualKeyCode::NumpadDecimal),
        Keycode::NumLock => Some(VirtualKeyCode::Numlock),

        Keycode::DpadLeft => Some(VirtualKeyCode::Left),
        Keycode::DpadRight => Some(VirtualKeyCode::Right),
        Keycode::DpadUp => Some(VirtualKeyCode::Up),
        Keycode::DpadDown => Some(VirtualKeyCode::Down),

        Keycode::F1 => Some(VirtualKeyCode::F1),
        Keycode::F2 => Some(VirtualKeyCode::F2),
        Keycode::F3 => Some(VirtualKeyCode::F3),
        Keycode::F4 => Some(VirtualKeyCode::F4),
        Keycode::F5 => Some(VirtualKeyCode::F5),
        Keycode::F6 => Some(VirtualKeyCode::F6),
        Keycode::F7 => Some(VirtualKeyCode::F7),
        Keycode::F8 => Some(VirtualKeyCode::F8),
        Keycode::F9 => Some(VirtualKeyCode::F9),
        Keycode::F10 => Some(VirtualKeyCode::F10),
        Keycode::F11 => Some(VirtualKeyCode::F11),
        Keycode::F12 => Some(VirtualKeyCode::F12),

        Keycode::Space => Some(VirtualKeyCode::Space),
        Keycode::Escape => Some(VirtualKeyCode::Escape),
        Keycode::Enter => Some(VirtualKeyCode::Return), // not on the Numpad
        Keycode::Tab => Some(VirtualKeyCode::Tab),

        Keycode::PageUp => Some(VirtualKeyCode::PageUp),
        Keycode::PageDown => Some(VirtualKeyCode::PageDown),
        Keycode::MoveHome => Some(VirtualKeyCode::Home),
        Keycode::MoveEnd => Some(VirtualKeyCode::End),
        Keycode::Insert => Some(VirtualKeyCode::Insert),

        Keycode::Del => Some(VirtualKeyCode::Back), // Backspace (above Enter)
        Keycode::ForwardDel => Some(VirtualKeyCode::Delete), // Delete (below Insert)

        Keycode::Copy => Some(VirtualKeyCode::Copy),
        Keycode::Paste => Some(VirtualKeyCode::Paste),
        Keycode::Cut => Some(VirtualKeyCode::Cut),

        Keycode::VolumeUp => Some(VirtualKeyCode::VolumeUp),
        Keycode::VolumeDown => Some(VirtualKeyCode::VolumeDown),
        Keycode::VolumeMute => Some(VirtualKeyCode::Mute), // ???
        Keycode::Mute => Some(VirtualKeyCode::Mute),       // ???
        Keycode::MediaPlayPause => Some(VirtualKeyCode::PlayPause),
        Keycode::MediaStop => Some(VirtualKeyCode::MediaStop), // ??? simple "Stop"?
        Keycode::MediaNext => Some(VirtualKeyCode::NextTrack),
        Keycode::MediaPrevious => Some(VirtualKeyCode::PrevTrack),

        Keycode::Plus => Some(VirtualKeyCode::Plus),
        Keycode::Minus => Some(VirtualKeyCode::Minus),
        Keycode::Equals => Some(VirtualKeyCode::Equals),
        Keycode::Semicolon => Some(VirtualKeyCode::Semicolon),
        Keycode::Slash => Some(VirtualKeyCode::Slash),
        Keycode::Backslash => Some(VirtualKeyCode::Backslash),
        Keycode::Comma => Some(VirtualKeyCode::Comma),
        Keycode::Period => Some(VirtualKeyCode::Period),
        Keycode::Apostrophe => Some(VirtualKeyCode::Apostrophe),
        Keycode::Grave => Some(VirtualKeyCode::Grave),
        Keycode::At => Some(VirtualKeyCode::At),

        // TODO: Maybe mapping this to Snapshot makes more sense? See: "PrtScr/SysRq"
        Keycode::Sysrq => Some(VirtualKeyCode::Sysrq),
        // These are usually the same (Pause/Break)
        Keycode::Break => Some(VirtualKeyCode::Pause),
        // These are exactly the same
        Keycode::ScrollLock => Some(VirtualKeyCode::Scroll),

        Keycode::Yen => Some(VirtualKeyCode::Yen),
        Keycode::Kana => Some(VirtualKeyCode::Kana),

        Keycode::CtrlLeft => Some(VirtualKeyCode::LControl),
        Keycode::CtrlRight => Some(VirtualKeyCode::RControl),

        Keycode::ShiftLeft => Some(VirtualKeyCode::LShift),
        Keycode::ShiftRight => Some(VirtualKeyCode::RShift),

        Keycode::AltLeft => Some(VirtualKeyCode::LAlt),
        Keycode::AltRight => Some(VirtualKeyCode::RAlt),

        // Different names for the same keys
        Keycode::MetaLeft => Some(VirtualKeyCode::LWin),
        Keycode::MetaRight => Some(VirtualKeyCode::RWin),

        Keycode::LeftBracket => Some(VirtualKeyCode::LBracket),
        Keycode::RightBracket => Some(VirtualKeyCode::RBracket),

        Keycode::Power => Some(VirtualKeyCode::Power),
        Keycode::Sleep => Some(VirtualKeyCode::Sleep), // what about SoftSleep?
        Keycode::Wakeup => Some(VirtualKeyCode::Wake),

        Keycode::NavigateNext => Some(VirtualKeyCode::NavigateForward),
        Keycode::NavigatePrevious => Some(VirtualKeyCode::NavigateBackward),

        Keycode::Calculator => Some(VirtualKeyCode::Calculator),
        Keycode::Explorer => Some(VirtualKeyCode::MyComputer), // "close enough"
        Keycode::Envelope => Some(VirtualKeyCode::Mail),       // "close enough"

        Keycode::Star => Some(VirtualKeyCode::Asterisk), // ???
        Keycode::AllApps => Some(VirtualKeyCode::Apps),  // ???
        Keycode::AppSwitch => Some(VirtualKeyCode::Apps), // ???
        Keycode::Refresh => Some(VirtualKeyCode::WebRefresh), // ???

        _ => None,
    }
}

/// Tries to map the `key_event` to a `KeyMapChar` containing a unicode character or dead key accent
///
/// This takes a `KeyEvent` and looks up its corresponding `KeyCharacterMap` and
/// uses that to try and map the `key_code` + `meta_state` to a unicode
/// character or a dead key that can be combined with the next key press.
pub fn character_map_and_combine_key(
    app: &AndroidApp,
    key_event: &KeyEvent<'_>,
    combining_accent: &mut Option<char>,
) -> Option<KeyMapChar> {
    let device_id = key_event.device_id();

    let key_map = match app.device_key_character_map(device_id) {
        Ok(key_map) => key_map,
        Err(err) => {
            log::error!("Failed to look up `KeyCharacterMap` for device {device_id}: {err:?}");
            return None;
        }
    };

    match key_map.get(key_event.key_code(), key_event.meta_state()) {
        Ok(KeyMapChar::Unicode(unicode)) => {
            // Only do dead key combining on key down
            if key_event.action() == KeyAction::Down {
                let combined_unicode = if let Some(accent) = combining_accent {
                    match key_map.get_dead_char(*accent, unicode) {
                        Ok(Some(key)) => Some(key),
                        Ok(None) => None,
                        Err(err) => {
                            log::error!("KeyEvent: Failed to combine 'dead key' accent '{accent}' with '{unicode}': {err:?}");
                            None
                        }
                    }
                } else {
                    Some(unicode)
                };
                *combining_accent = None;
                combined_unicode.map(|unicode| KeyMapChar::Unicode(unicode))
            } else {
                Some(KeyMapChar::Unicode(unicode))
            }
        }
        Ok(KeyMapChar::CombiningAccent(accent)) => {
            if key_event.action() == KeyAction::Down {
                *combining_accent = Some(accent);
            }
            Some(KeyMapChar::CombiningAccent(accent))
        }
        Ok(KeyMapChar::None) => {
            // Leave any combining_accent state in tact (seems to match how other
            // Android apps work)
            None
        }
        Err(err) => {
            log::error!("KeyEvent: Failed to get key map character: {err:?}");
            *combining_accent = None;
            None
        }
    }
}
