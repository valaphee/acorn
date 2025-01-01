// Copyright 2025 Kevin Ludwig
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

use crate::Result;

pub trait InputDevice {
    fn next(&self) -> Result<InputEvent>;
}

pub enum InputEvent {
    Button {
        time: u32,
        button: Button,
        value: bool,
    },
    Axis {
        time: u32,
        axis: Axis,
        value: i16,
    },
}

pub enum Button {
    /// <kbd>`~</kbd> on a US keyboard. This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd> (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\\|</kbd> (on the 101-key layout) and also for the key located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-, 104- and 106-key layouts. Labeled <kbd>#~</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[{</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]}</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,<</kbd> on a US keyboard.
    Comma,
    /// <kbd>0)</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1!</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2@</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3#</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4$</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5%</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6^</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7&</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8*</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9(</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=+</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys. Labeled <kbd>\\|</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys. Labeled <kbd>\ろ</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys. Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\/</kbd> on a Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard. Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <kbd>b</kbd> on a US keyboard.
    KeyB,
    /// <kbd>c</kbd> on a US keyboard.
    KeyC,
    /// <kbd>d</kbd> on a US keyboard.
    KeyD,
    /// <kbd>e</kbd> on a US keyboard.
    KeyE,
    /// <kbd>f</kbd> on a US keyboard.
    KeyF,
    /// <kbd>g</kbd> on a US keyboard.
    KeyG,
    /// <kbd>h</kbd> on a US keyboard.
    KeyH,
    /// <kbd>i</kbd> on a US keyboard.
    KeyI,
    /// <kbd>j</kbd> on a US keyboard.
    KeyJ,
    /// <kbd>k</kbd> on a US keyboard.
    KeyK,
    /// <kbd>l</kbd> on a US keyboard.
    KeyL,
    /// <kbd>m</kbd> on a US keyboard.
    KeyM,
    /// <kbd>n</kbd> on a US keyboard.
    KeyN,
    /// <kbd>o</kbd> on a US keyboard.
    KeyO,
    /// <kbd>p</kbd> on a US keyboard.
    KeyP,
    /// <kbd>q</kbd> on a US keyboard. Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <kbd>r</kbd> on a US keyboard.
    KeyR,
    /// <kbd>s</kbd> on a US keyboard.
    KeyS,
    /// <kbd>t</kbd> on a US keyboard.
    KeyT,
    /// <kbd>u</kbd> on a US keyboard.
    KeyU,
    /// <kbd>v</kbd> on a US keyboard.
    KeyV,
    /// <kbd>w</kbd> on a US keyboard. Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <kbd>x</kbd> on a US keyboard.
    KeyX,
    /// <kbd>y</kbd> on a US keyboard. Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <kbd>z</kbd> on a US keyboard. Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <kbd>-_</kbd> on a US keyboard.
    Minus,
    /// <kbd>.></kbd> on a US keyboard.
    Period,
    /// <kbd>'"</kbd> on a US keyboard.
    Quote,
    /// <kbd>;:</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/?</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>. This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>. Labeled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right <kbd>Meta</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd> </kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,
    /// Japanese: <kbd>変</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd>
    /// (katakana/hiragana/romaji)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    Lang1,
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key. Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>End</kbd>, or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// On the Mac, the "NumLock" code should be used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard.
    ///
    /// <kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard.
    ///
    /// <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard.
    ///
    /// <kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard.
    ///
    /// <kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard.
    ///
    /// <kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard.
    ///
    /// <kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard.
    ///
    /// <kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard.
    ///
    /// <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard.
    ///
    /// <kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard.
    ///
    /// <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>AC</kbd> (All Clear). Also for use with numpads that have a <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the numpad <kbd>Clear</kbd> key should always be encoded as [`NumLock`].
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>CE</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g., Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M+</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>MC</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>MR</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>MS</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M-</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device. This key is typically found below the <kbd>7</kbd> key and to the left of the <kbd>0</kbd> key.
    ///
    /// Use "NumpadMultiply" for the <kbd>*</kbd> key on numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>F1</kbd>
    F1,
    /// <kbd>F2</kbd>
    F2,
    /// <kbd>F3</kbd>
    F3,
    /// <kbd>F4</kbd>
    F4,
    /// <kbd>F5</kbd>
    F5,
    /// <kbd>F6</kbd>
    F6,
    /// <kbd>F7</kbd>
    F7,
    /// <kbd>F8</kbd>
    F8,
    /// <kbd>F9</kbd>
    F9,
    /// <kbd>F10</kbd>
    F10,
    /// <kbd>F11</kbd>
    F11,
    /// <kbd>F12</kbd>
    F12,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code. Most keyboards do not place this key in the function section, but it is included here to keep it with related keys.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards, replacing the <kbd>Eject</kbd> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Hyper,
    Super,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    MouseLeft,
    MouseMiddle,
    MouseRight,
    GamepadA,
    GamepadB,
    GamepadX,
    GamepadY,
    GamepadTop,
    GamepadBottom,
    GamepadLeft,
    GamepadRight,
    GamepadL1,
    GamepadR1,
    GamepadL2,
    GamepadR2,
    GamepadL3,
    GamepadR3,
    /// This value code should be used when no other value given in this specification is appropriate.
    Unidentified = 0xFF,
}

pub enum Axis {
    MouseX,
    MouseY,
    MouseZ,
    TouchX,
    TouchY,
    TouchZ,
    Touch1X,
    Touch1Y,
    Touch1Z,
    Touch2X,
    Touch2Y,
    Touch2Z,
    Touch3X,
    Touch3Y,
    Touch3Z,
    Touch4X,
    Touch4Y,
    Touch4Z,
    Touch5X,
    Touch5Y,
    Touch5Z,
    Touch6X,
    Touch6Y,
    Touch6Z,
    Touch7X,
    Touch7Y,
    Touch7Z,
    Touch8X,
    Touch8Y,
    Touch8Z,
    Touch9X,
    Touch9Y,
    Touch9Z,
    GamepadXL,
    GamepadXR,
    GamepadYL,
    GamepadYR,
    GamepadZL,
    GamepadZR,
    Unidentified = 0xFF,
}
