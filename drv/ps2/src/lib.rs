// Copyright 2024 Kevin Ludwig
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]

use acorn_api::{Axis, Error, InputDevice, InputEvent, Key, Result};

// 0x80| released
const SCAN_CODE_SET_1: [Key; 89] = [
    Key::Unidentified,
    Key::Escape,
    Key::Digit1,
    Key::Digit2,
    Key::Digit3,
    Key::Digit4,
    Key::Digit5,
    Key::Digit6,
    Key::Digit7,
    Key::Digit8,
    Key::Digit9,
    Key::Digit0,
    Key::Minus,
    Key::Equal,
    Key::Backspace,
    Key::Tab,
    Key::KeyQ,
    Key::KeyW,
    Key::KeyE,
    Key::KeyR,
    Key::KeyT,
    Key::KeyY,
    Key::KeyU,
    Key::KeyI,
    Key::KeyO,
    Key::KeyP,
    Key::BracketLeft,
    Key::BracketRight,
    Key::Enter,
    Key::ControlLeft,
    Key::KeyA,
    Key::KeyS,
    Key::KeyD,
    Key::KeyF,
    Key::KeyG,
    Key::KeyH,
    Key::KeyJ,
    Key::KeyK,
    Key::KeyL,
    Key::Semicolon,
    Key::Quote,
    Key::Backquote,
    Key::ShiftLeft,
    Key::Backslash,
    Key::KeyZ,
    Key::KeyX,
    Key::KeyC,
    Key::KeyV,
    Key::KeyB,
    Key::KeyN,
    Key::KeyM,
    Key::Comma,
    Key::Period,
    Key::Slash,
    Key::ShiftRight,
    Key::NumpadMultiply,
    Key::AltLeft,
    Key::Space,
    Key::CapsLock,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::NumLock,
    Key::ScrollLock,
    Key::Numpad7,
    Key::Numpad8,
    Key::Numpad9,
    Key::NumpadSubtract,
    Key::Numpad4,
    Key::Numpad5,
    Key::Numpad6,
    Key::NumpadAdd,
    Key::Numpad1,
    Key::Numpad2,
    Key::Numpad3,
    Key::Numpad0,
    Key::NumpadDecimal,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::F11,
    Key::F12,
];

// 0xF0, released
const SCAN_CODE_SET_2: [Key; 131] = [
    Key::Unidentified,
    Key::F9,
    Key::Unidentified,
    Key::F5,
    Key::F3,
    Key::F1,
    Key::F2,
    Key::F12,
    Key::Unidentified,
    Key::F10,
    Key::F8,
    Key::F6,
    Key::F4,
    Key::Tab,
    Key::Backquote,
    Key::Unidentified,
    Key::Unidentified,
    Key::AltLeft,
    Key::ShiftLeft,
    Key::ControlLeft,
    Key::KeyQ,
    Key::Digit1,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::KeyZ,
    Key::KeyS,
    Key::KeyA,
    Key::KeyW,
    Key::Digit2,
    Key::Unidentified,
    Key::Unidentified,
    Key::KeyC,
    Key::KeyX,
    Key::KeyD,
    Key::KeyE,
    Key::Digit4,
    Key::Digit3,
    Key::Unidentified,
    Key::Unidentified,
    Key::Space,
    Key::KeyV,
    Key::KeyF,
    Key::KeyT,
    Key::KeyR,
    Key::Digit5,
    Key::Unidentified,
    Key::Unidentified,
    Key::KeyN,
    Key::KeyB,
    Key::KeyH,
    Key::KeyG,
    Key::KeyY,
    Key::Digit6,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::KeyM,
    Key::KeyJ,
    Key::KeyU,
    Key::Digit7,
    Key::Digit8,
    Key::Unidentified,
    Key::Unidentified,
    Key::Comma,
    Key::KeyK,
    Key::KeyI,
    Key::KeyO,
    Key::Digit0,
    Key::Digit9,
    Key::Unidentified,
    Key::Unidentified,
    Key::Period,
    Key::Slash,
    Key::KeyL,
    Key::Semicolon,
    Key::KeyP,
    Key::Minus,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Quote,
    Key::Unidentified,
    Key::BracketLeft,
    Key::Equal,
    Key::Unidentified,
    Key::Unidentified,
    Key::CapsLock,
    Key::ShiftRight,
    Key::Enter,
    Key::BracketRight,
    Key::Unidentified,
    Key::Backslash,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Backspace,
    Key::Unidentified,
    Key::Unidentified,
    Key::Numpad1,
    Key::Unidentified,
    Key::Numpad4,
    Key::Numpad7,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Numpad0,
    Key::NumpadDecimal,
    Key::Numpad2,
    Key::Numpad5,
    Key::Numpad6,
    Key::Numpad8,
    Key::Escape,
    Key::NumLock,
    Key::F11,
    Key::NumpadAdd,
    Key::Numpad3,
    Key::NumpadSubtract,
    Key::NumpadMultiply,
    Key::Numpad9,
    Key::ScrollLock,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::F7,
];

// 0xF0, released
const SCAN_CODE_SET_3: [Key; 140] = [
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::F1,
    Key::Escape,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Tab,
    Key::Backquote,
    Key::F2,
    Key::Unidentified,
    Key::ControlLeft,
    Key::ShiftLeft,
    Key::Unidentified,
    Key::CapsLock,
    Key::KeyQ,
    Key::Digit1,
    Key::F3,
    Key::Unidentified,
    Key::AltLeft,
    Key::KeyZ,
    Key::KeyS,
    Key::KeyA,
    Key::KeyW,
    Key::Digit2,
    Key::F4,
    Key::Unidentified,
    Key::KeyC,
    Key::KeyX,
    Key::KeyD,
    Key::KeyE,
    Key::Digit4,
    Key::Digit3,
    Key::F5,
    Key::Unidentified,
    Key::Space,
    Key::KeyV,
    Key::KeyF,
    Key::KeyT,
    Key::KeyR,
    Key::Digit5,
    Key::F6,
    Key::KeyN,
    Key::KeyB,
    Key::KeyH,
    Key::KeyG,
    Key::KeyY,
    Key::Digit6,
    Key::F7,
    Key::Unidentified,
    Key::AltRight,
    Key::KeyM,
    Key::KeyJ,
    Key::KeyU,
    Key::Space,
    Key::Digit8,
    Key::F8,
    Key::Unidentified,
    Key::Comma,
    Key::KeyK,
    Key::KeyI,
    Key::KeyO,
    Key::Digit0,
    Key::Digit9,
    Key::F9,
    Key::Unidentified,
    Key::Period,
    Key::Slash,
    Key::KeyL,
    Key::Semicolon,
    Key::KeyP,
    Key::Minus,
    Key::F10,
    Key::Unidentified,
    Key::Unidentified,
    Key::Quote,
    Key::Unidentified,
    Key::BracketLeft,
    Key::Equal,
    Key::F11,
    Key::PrintScreen,
    Key::ControlRight,
    Key::ShiftRight,
    Key::Enter,
    Key::BracketRight,
    Key::Backslash,
    Key::Unidentified,
    Key::F12,
    Key::ScrollLock,
    Key::ArrowDown,
    Key::ArrowLeft,
    Key::Pause,
    Key::ArrowUp,
    Key::Delete,
    Key::End,
    Key::Backspace,
    Key::Insert,
    Key::Unidentified,
    Key::Numpad1,
    Key::ArrowRight,
    Key::Numpad4,
    Key::Numpad7,
    Key::PageDown,
    Key::Home,
    Key::PageUp,
    Key::Numpad0,
    Key::NumpadDecimal,
    Key::Numpad2,
    Key::Numpad5,
    Key::Numpad6,
    Key::Numpad8,
    Key::NumLock,
    Key::Unidentified,
    Key::Unidentified,
    Key::NumpadEnter,
    Key::Numpad3,
    Key::Unidentified,
    Key::NumpadAdd,
    Key::Numpad9,
    Key::NumpadMultiply,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::Unidentified,
    Key::SuperLeft,
    Key::SuperRight,
];

pub struct PS2Keyboard {}

impl InputDevice for PS2Keyboard {
    fn read(&self) -> Result<InputEvent> {
        let scancode_1 = 0u8;

        Err(Error::Unimplemented)
    }
}

pub struct PS2Mouse {}

impl InputDevice for PS2Mouse {
    fn read(&self) -> Result<InputEvent> {
        let flags = 0u8;
        let rel_x = 0u8;
        let rel_y = 0u8;
        let rel_z = 0u8;

        if flags & 1 << 0 != 0 {
            InputEvent::Key {
                key: Key::MouseLeft,
                value: true,
            };
        }
        if flags & 1 << 1 != 0 {
            InputEvent::Key {
                key: Key::MouseMiddle,
                value: true,
            };
        }
        if flags & 1 << 2 != 0 {
            InputEvent::Key {
                key: Key::MouseRight,
                value: true,
            };
        }
        InputEvent::Rel {
            axis: Axis::X,
            value: rel_x as i16 * if flags & 1 << 4 != 0 { -1 } else { 1 },
        };
        InputEvent::Rel {
            axis: Axis::Y,
            value: rel_y as i16 * if flags & 1 << 5 != 0 { -1 } else { 1 },
        };
        InputEvent::Rel {
            axis: Axis::Z,
            value: rel_z as i16,
        };
        if rel_z & 1 << 4 != 0 {
            InputEvent::Key {
                key: Key::BrowserBack,
                value: true,
            };
        }
        if rel_z & 1 << 5 != 0 {
            InputEvent::Key {
                key: Key::BrowserForward,
                value: true,
            };
        }

        Err(Error::Unimplemented)
    }
}