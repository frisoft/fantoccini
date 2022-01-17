//! Key codes for use with Actions.

use std::ops::{Add, AddAssign};

/// Key codes for use with Actions.
#[derive(Debug)]
pub enum Key {
    /// Null
    Null,
    /// Cancel
    Cancel,
    /// Help
    Help,
    /// Backspace key
    Backspace,
    /// Tab key
    Tab,
    /// Clear
    Clear,
    /// Return key
    Return,
    /// Enter key
    Enter,
    /// Shift key
    Shift,
    /// Control key
    Control,
    /// Alt key
    Alt,
    /// Pause key
    Pause,
    /// Escape key
    Escape,
    /// Space bar
    Space,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,
    /// End key
    End,
    /// Home key
    Home,
    /// Left arrow key
    Left,
    /// Up arrow key
    Up,
    /// Right arrow key
    Right,
    /// Down arrow key
    Down,
    /// Insert key
    Insert,
    /// Delete key
    Delete,
    /// Semicolon key
    Semicolon,
    /// Equals key
    Equals,
    /// Numpad 0 key
    NumPad0,
    /// Numpad 1 key
    NumPad1,
    /// Numpad 2 key
    NumPad2,
    /// Numpad 3 key
    NumPad3,
    /// Numpad 4 key
    NumPad4,
    /// Numpad 5 key
    NumPad5,
    /// Numpad 6 key
    NumPad6,
    /// Numpad 7 key
    NumPad7,
    /// Numpad 8 key
    NumPad8,
    /// Numpad 9 key
    NumPad9,
    /// Multiply key
    Multiply,
    /// Add key
    Add,
    /// Separator key
    Separator,
    /// Subtract key
    Subtract,
    /// Decimal key
    Decimal,
    /// Divide key
    Divide,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// Meta key
    Meta,
    /// Command key
    Command,
}

impl From<Key> for char {
    fn from(k: Key) -> char {
        match k {
            Key::Null => '\u{e000}',
            Key::Cancel => '\u{e001}',
            Key::Help => '\u{e002}',
            Key::Backspace => '\u{e003}',
            Key::Tab => '\u{e004}',
            Key::Clear => '\u{e005}',
            Key::Return => '\u{e006}',
            Key::Enter => '\u{e007}',
            Key::Shift => '\u{e008}',
            Key::Control => '\u{e009}',
            Key::Alt => '\u{e00a}',
            Key::Pause => '\u{e00b}',
            Key::Escape => '\u{e00c}',
            Key::Space => '\u{e00d}',
            Key::PageUp => '\u{e00e}',
            Key::PageDown => '\u{e00f}',
            Key::End => '\u{e010}',
            Key::Home => '\u{e011}',
            Key::Left => '\u{e012}',
            Key::Up => '\u{e013}',
            Key::Right => '\u{e014}',
            Key::Down => '\u{e015}',
            Key::Insert => '\u{e016}',
            Key::Delete => '\u{e017}',
            Key::Semicolon => '\u{e018}',
            Key::Equals => '\u{e019}',
            Key::NumPad0 => '\u{e01a}',
            Key::NumPad1 => '\u{e01b}',
            Key::NumPad2 => '\u{e01c}',
            Key::NumPad3 => '\u{e01d}',
            Key::NumPad4 => '\u{e01e}',
            Key::NumPad5 => '\u{e01f}',
            Key::NumPad6 => '\u{e020}',
            Key::NumPad7 => '\u{e021}',
            Key::NumPad8 => '\u{e022}',
            Key::NumPad9 => '\u{e023}',
            Key::Multiply => '\u{e024}',
            Key::Add => '\u{e025}',
            Key::Separator => '\u{e026}',
            Key::Subtract => '\u{e027}',
            Key::Decimal => '\u{e028}',
            Key::Divide => '\u{e029}',
            Key::F1 => '\u{e031}',
            Key::F2 => '\u{e032}',
            Key::F3 => '\u{e033}',
            Key::F4 => '\u{e034}',
            Key::F5 => '\u{e035}',
            Key::F6 => '\u{e036}',
            Key::F7 => '\u{e037}',
            Key::F8 => '\u{e038}',
            Key::F9 => '\u{e039}',
            Key::F10 => '\u{e03a}',
            Key::F11 => '\u{e03b}',
            Key::F12 => '\u{e03c}',
            Key::Meta => '\u{e03d}',
            Key::Command => '\u{e03d}',
        }
    }
}

impl Add<Key> for String {
    type Output = String;

    fn add(mut self, rhs: Key) -> Self::Output {
        self.push(rhs.into());
        self
    }
}

impl AddAssign<Key> for String {
    fn add_assign(&mut self, rhs: Key) {
        self.push(rhs.into());
    }
}
