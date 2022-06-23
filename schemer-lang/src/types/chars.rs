/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::{Environment, Evaluate, Expression};
use crate::read::syntax_str::{SYNTAX_CHAR_PREFIX, SYNTAX_HEX_CHAR_PREFIX};
use crate::types::new_type::NewType;
use crate::types::{MutableRef, SchemeRepr, SchemeValue};
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use unic_ucd_name::Name;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Char = NewType<char>;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Codepoint(u32);

pub const CHAR_NAME_ALARM: &str = "alarm";
pub const CHAR_NAME_BACKSPACE: &str = "backspace";
pub const CHAR_NAME_DELETE: &str = "delete";
pub const CHAR_NAME_ESCAPE: &str = "escape";
pub const CHAR_NAME_NEWLINE: &str = "newline";
pub const CHAR_NAME_NULL: &str = "null";
pub const CHAR_NAME_RETURN: &str = "return";
pub const CHAR_NAME_SPACE: &str = "space";
pub const CHAR_NAME_TAB: &str = "tab";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Char {
    fn to_repr_string(&self) -> String {
        if self.is_ascii() && !self.is_ascii_control() {
            format!("{}{}", SYNTAX_CHAR_PREFIX, **self)
        } else if self.is_alphanumeric() {
            format!("{}{}", SYNTAX_CHAR_PREFIX, **self)
        } else if let Some(name) = self.to_scheme_name() {
            name
        } else {
            format!(
                "{}{:X}",
                SYNTAX_HEX_CHAR_PREFIX,
                self.to_unicode_codepoint()
            )
        }
    }
}

scheme_value!(Char, TYPE_NAME_CHAR, "char");

impl Evaluate for Char {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Character(self.clone()))
    }
}

impl Char {
    pub fn from_unicode_codepoint(c: u32) -> Result<Self, Error> {
        if !Self::is_valid_codepoint(c) || !Self::is_valid_char(c) {
            Err(Error::from(ErrorKind::UnexpectedValue {
                type_name: TYPE_NAME_CHAR.to_string(),
                actual: format!("{}{:X}", SYNTAX_HEX_CHAR_PREFIX, c),
                expected: TYPE_NAME_CODEPOINT.to_string(),
            }))
        } else {
            char::from_u32(c)
                .ok_or(
                    ErrorKind::ParseValue {
                        kind: TYPE_NAME_CHAR.to_string(),
                        value: format!("{}{:X}", SYNTAX_CHAR_PREFIX, c),
                    }
                    .into(),
                )
                .map(|c| Self::from(c))
        }
    }

    pub fn to_unicode_codepoint(&self) -> u32 {
        **self as u32
    }

    pub fn to_scheme_name(&self) -> Option<String> {
        Self::named(match **self {
            '\u{0}' => Some(CHAR_NAME_NULL.to_string()),
            '\u{7}' => Some(CHAR_NAME_ALARM.to_string()),
            '\u{8}' => Some(CHAR_NAME_BACKSPACE.to_string()),
            '\u{9}' => Some(CHAR_NAME_TAB.to_string()),
            '\u{a}' => Some(CHAR_NAME_NEWLINE.to_string()),
            '\u{d}' => Some(CHAR_NAME_RETURN.to_string()),
            '\u{1b}' => Some(CHAR_NAME_ESCAPE.to_string()),
            '\u{20}' => Some(CHAR_NAME_SPACE.to_string()),
            '\u{7f}' => Some(CHAR_NAME_DELETE.to_string()),
            _ => None,
        })
    }

    pub fn from_scheme_name(name: &str) -> Option<Self> {
        match if name.starts_with(SYNTAX_CHAR_PREFIX) {
            &name[2..]
        } else {
            name
        } {
            CHAR_NAME_NULL => Some(Char::from('\u{0}')),
            CHAR_NAME_ALARM => Some(Char::from('\u{7}')),
            CHAR_NAME_BACKSPACE => Some(Char::from('\u{8}')),
            CHAR_NAME_TAB => Some(Char::from('\u{9}')),
            CHAR_NAME_NEWLINE => Some(Char::from('\u{a}')),
            CHAR_NAME_RETURN => Some(Char::from('\u{d}')),
            CHAR_NAME_ESCAPE => Some(Char::from('\u{1b}')),
            CHAR_NAME_SPACE => Some(Char::from('\u{20}')),
            CHAR_NAME_DELETE => Some(Char::from('\u{7f}')),
            _ => None,
        }
    }

    pub fn to_unicode_name(&self) -> Option<String> {
        Self::named(unic_ucd_name::Name::of(**self).map(|n| {
            match n {
                Name::NR1(c) => c.to_string(),
                Name::NR2(n, _) => n.replace(' ', "_").to_ascii_lowercase(),
                Name::NR3(ns) => ns
                    .iter()
                    .map(|n| n.to_ascii_lowercase())
                    .collect::<Vec<String>>()
                    .join("_"),
            }
        }))
    }

    pub fn from_unicode_name(_name: &str) -> Option<Self> {
        None
    }

    pub fn to_name(&self) -> Option<String> {
        self.to_scheme_name().or_else(|| self.to_unicode_name())
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_scheme_name(name).or_else(|| Self::from_unicode_name(name))
    }

    fn named(name: Option<String>) -> Option<String> {
        name.map(|name| format!("{}{}", SYNTAX_CHAR_PREFIX, name))
    }

    fn is_valid_codepoint(cp: u32) -> bool {
        cp < 0x10FFFF
    }

    fn is_valid_char(cp: u32) -> bool {
        match cp {
            0xD7FF..=0xE000
            | 0xFDD0..=0xFDEF
            | 0xFFFE..=0xFFFF
            | 0x1FFFE..=0x1FFFF
            | 0x2FFFE..=0x2FFFF
            | 0x3FFFE..=0x3FFFF
            | 0x4FFFE..=0x4FFFF
            | 0x5FFFE..=0x5FFFF
            | 0x6FFFE..=0x6FFFF
            | 0x7FFFE..=0x7FFFF
            | 0x8FFFE..=0x8FFFF
            | 0x9FFFE..=0x9FFFF
            | 0x10FFFE..=0x10FFFF => false,
            _ => true,
        }
    }
}

// ------------------------------------------------------------------------------------------------

scheme_value!(Codepoint, TYPE_NAME_CODEPOINT, "unicode-codepoint");

impl Debug for Codepoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\U+{:04X}", self.0)
    }
}

impl From<Codepoint> for u32 {
    fn from(cp: Codepoint) -> Self {
        cp.0
    }
}

impl TryFrom<u32> for Codepoint {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < MAX_VALID_CODEPOINT.into() {
            Ok(Self(value))
        } else {
            Err(ErrorKind::UnexpectedValue {
                type_name: TYPE_NAME_CODEPOINT.to_string(),
                expected: "".to_string(),
                actual: value.to_string(),
            }
            .into())
        }
    }
}

impl From<Char> for Codepoint {
    fn from(value: Char) -> Self {
        Self(value.into_inner() as u32)
    }
}

impl TryFrom<Codepoint> for Char {
    type Error = Error;

    fn try_from(value: Codepoint) -> Result<Self, Self::Error> {
        match value.0 {
            0xD7FF..=0xE000
            | 0xFDD0..=0xFDEF
            | 0xFFFE..=0xFFFF
            | 0x1FFFE..=0x1FFFF
            | 0x2FFFE..=0x2FFFF
            | 0x3FFFE..=0x3FFFF
            | 0x4FFFE..=0x4FFFF
            | 0x5FFFE..=0x5FFFF
            | 0x6FFFE..=0x6FFFF
            | 0x7FFFE..=0x7FFFF
            | 0x8FFFE..=0x8FFFF
            | 0x9FFFE..=0x9FFFF
            | 0x10FFFE..=0x10FFFF => Err(ErrorKind::UnexpectedValue {
                type_name: TYPE_NAME_CODEPOINT.to_string(),
                expected: "".to_string(),
                actual: value.to_repr_string(),
            }
            .into()),
            _ => Ok(Char::from(char::from_u32(value.0).unwrap())),
        }
    }
}

impl SchemeRepr for Codepoint {
    fn to_repr_string(&self) -> String {
        format!("#x{:04X}", self.0)
    }
}

impl Evaluate for Codepoint {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Character(Char::try_from(*self)?))
    }
}

pub const MAX_VALID_CODEPOINT: Codepoint = Codepoint(0x10FFFE);
pub const INVALID_CODEPOINT_VALUE: Codepoint = Codepoint(u32::MAX);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::types::chars::*;
    use crate::types::SchemeRepr;

    #[test]
    fn test_repr_single() {
        assert_eq!("#\\A".to_string(), Char::from('A').to_repr_string());
    }

    #[test]
    fn test_repr_non_ascii() {
        assert_eq!("#\\ༀ".to_string(), Char::from('ༀ').to_repr_string());
        assert_eq!("#\\x1F02A".to_string(), Char::from('🀪').to_repr_string());
    }

    #[test]
    fn test_repr_named() {
        assert_eq!(
            "#\\delete".to_string(),
            Char::from('\u{7f}').to_repr_string()
        );
    }

    #[test]
    fn test_char_to_unicode_name() {
        assert_eq!(Some("#\\black_star".to_string()), Char::from('★').to_name());
    }

    #[test]
    #[ignore]
    fn test_char_from_unicode_name() {
        assert_eq!(Some(Char::from('★')), Char::from_name("#\\black_star"));
    }
}
