/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::newtype::NewType;
use crate::types::SchemeValue;
pub use char_names::name_to_char;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Char = NewType<char>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn char_to_name(c: char) -> String {
    if let Some(name) = char_names::char_to_name(c) {
        name
    } else if c.is_ascii() && !c.is_ascii_control() {
        format!("#\\{}", c)
    } else {
        let escaped = c.escape_unicode().to_string();
        format!("#\\x{}", &escaped[3..escaped.len() - 1].to_uppercase())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl FromStr for Char {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#\\x") {
            println!("character_from_str numeric");
            u32::from_str_radix(&s[3..], 16)
                .map_err(|e| {
                    Error::chain(
                        Box::new(e),
                        ErrorKind::Value {
                            kind: TYPE_NAME_CHAR.to_string(),
                            value: s.to_string(),
                        }
                        .into(),
                    )
                })
                .and_then(|cv| Char::try_from(cv))
        } else if s.len() == 3 {
            let cs = &s[2..];
            let c = cs.chars().next().unwrap();
            Ok(c.into())
        } else {
            if let Some(c) = name_to_char(s) {
                Ok(c.into())
            } else {
                Err(ErrorKind::Value {
                    kind: TYPE_NAME_CHAR.to_string(),
                    value: s.to_string(),
                }
                .into())
            }
        }
    }
}

impl TryFrom<u32> for Char {
    type Error = Error;

    fn try_from(c: u32) -> Result<Self, Self::Error> {
        char::from_u32(c)
            .ok_or(
                ErrorKind::Value {
                    kind: TYPE_NAME_CHAR.to_string(),
                    value: format!("\\x{:X}", c),
                }
                .into(),
            )
            .map(|c| Self(c))
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.is_ascii() && !self.is_ascii_control() {
                format!("#\\{}", self)
            } else if let Some(name) = char_names::char_to_name(**self) {
                name
            } else {
                let escaped = self.escape_unicode().to_string();
                format!("#\\x{}", &escaped[3..escaped.len() - 1].to_uppercase())
            }
        )
    }
}

scheme_value!(Char, TYPE_NAME_CHAR, "char");

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "char-names")]
mod char_names {
    use unicode_names2 as unicode;

    // --------------------------------------------------------------------------------------------
    // Public Functions
    // --------------------------------------------------------------------------------------------

    pub fn name_to_char(name: &str) -> Option<char> {
        if let Some(c) = super::default_char_names::name_to_char(name) {
            Some(c)
        } else {
            let name = name[2..].to_string().replace('_', " ");
            unicode::character(&name)
        }
    }

    pub fn char_to_name(c: char) -> Option<String> {
        if let Some(name) = super::default_char_names::char_to_name(c) {
            Some(name)
        } else {
            unicode::name(c)
                .map(|n| String::from("#\\") + &n.to_string().to_lowercase().replace(' ', "_"))
        }
    }
}

#[cfg(not(feature = "char-names"))]
use default_char_names as char_names;

mod default_char_names {
    // --------------------------------------------------------------------------------------------
    // Public Functions
    // --------------------------------------------------------------------------------------------

    pub fn name_to_char(name: &str) -> Option<char> {
        if name == "#\\null" {
            Some('\0')
        } else if name == "#\\alarm" {
            Some('\u{7}')
        } else if name == "#\\backspace" {
            Some('\u{8}')
        } else if name == "#\\tab" {
            Some('\u{9}')
        } else if name == "#\\return" {
            Some('\u{d}')
        } else if name == "#\\escape" {
            Some('\u{1b}')
        } else if name == "#\\space" {
            Some('\u{20}')
        } else if name == "#\\delete" {
            Some('\u{7f}')
        } else {
            None
        }
    }

    pub fn char_to_name(c: char) -> Option<String> {
        if c == '\u{0}' {
            Some("#\\null".to_string())
        } else if c == '\u{7}' {
            Some("#\\alarm".to_string())
        } else if c == '\u{8}' {
            Some("#\\backspace".to_string())
        } else if c == '\u{9}' {
            Some("#\\tab".to_string())
        } else if c == '\u{d}' {
            Some("#\\return".to_string())
        } else if c == '\u{1b}' {
            Some("#\\escape".to_string())
        } else if c == '\u{20}' {
            Some("#\\space".to_string())
        } else if c == '\u{7f}' {
            Some("#\\delete".to_string())
        } else {
            None
        }
    }
}
