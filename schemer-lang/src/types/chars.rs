/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::read::syntax_str::{SYNTAX_CHAR_PREFIX, SYNTAX_HEX_CHAR_PREFIX};
use crate::types::new_type::NewType;
use crate::types::{MutableRef, SchemeRepr, SchemeValue};
pub use char_names::name_to_char;
use std::convert::TryFrom;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Char = NewType<char>;

pub const CODE_POINT_ERROR: u32 = 0x1FFFFF;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn char_to_codepoint(c: char) -> u32 {
    c as u32
}

pub fn char_to_name(c: char) -> String {
    if let Some(name) = char_names::char_to_name(c) {
        name
    } else if c.is_ascii() && !c.is_ascii_control() {
        format!("{}{}", SYNTAX_CHAR_PREFIX, c)
    } else {
        let escaped = c.escape_unicode().to_string();
        format!(
            "{}{}",
            SYNTAX_HEX_CHAR_PREFIX,
            &escaped[3..escaped.len() - 1].to_uppercase()
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------

impl TryFrom<u32> for Char {
    type Error = Error;

    fn try_from(c: u32) -> Result<Self, Self::Error> {
        if !is_valid_codepoint(c) || !is_valid_char(c) {
            return Err(Error::from(ErrorKind::UnexpectedValue {
                type_name: TYPE_NAME_CHAR.to_string(),
                actual: format!("{}{:X}", SYNTAX_HEX_CHAR_PREFIX, c),
                expected: "valid Unicode codepoint".to_string(),
            }));
        }
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

impl SchemeRepr for Char {
    fn to_repr_string(&self) -> String {
        if self.is_ascii() && !self.is_ascii_control() {
            format!("{}{}", SYNTAX_CHAR_PREFIX, **self)
        } else if self.is_alphanumeric() {
            format!("{}{}", SYNTAX_CHAR_PREFIX, **self)
        } else if let Some(name) = char_names::char_to_name(**self) {
            name
        } else {
            let escaped = self.escape_unicode().to_string();
            format!(
                "{}{}",
                SYNTAX_HEX_CHAR_PREFIX,
                &escaped[3..escaped.len() - 1].to_uppercase()
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "char-names")]
mod char_names {
    use crate::read::syntax_str::{
        SYNTAX_CHAR_PREFIX, SYNTAX_SPACE, SYNTAX_SPACE_CHAR, SYNTAX_UNDERSCORE,
        SYNTAX_UNDERSCORE_CHAR,
    };
    use unicode_names2 as unicode;

    // --------------------------------------------------------------------------------------------
    // Public Functions
    // --------------------------------------------------------------------------------------------

    pub fn name_to_char(name: &str) -> Option<char> {
        if let Some(c) = super::default_char_names::name_to_char(name) {
            Some(c)
        } else {
            let name = name[2..]
                .to_string()
                .replace(SYNTAX_UNDERSCORE_CHAR, SYNTAX_SPACE);
            unicode::character(&name)
        }
    }

    pub fn char_to_name(c: char) -> Option<String> {
        if let Some(name) = super::default_char_names::char_to_name(c) {
            Some(name)
        } else {
            unicode::name(c).map(|n| {
                String::from(SYNTAX_CHAR_PREFIX)
                    + &n.to_string()
                        .to_lowercase()
                        .replace(SYNTAX_SPACE_CHAR, SYNTAX_UNDERSCORE)
            })
        }
    }
}

use crate::eval::expression::Evaluate;
use crate::eval::{Environment, Expression};
#[cfg(not(feature = "char-names"))]
use default_char_names as char_names;

mod default_char_names {

    use crate::read::syntax_str::SYNTAX_CHAR_PREFIX;

    // --------------------------------------------------------------------------------------------
    // Public Values
    // --------------------------------------------------------------------------------------------

    pub const CHAR_NAME_ALARM: &str = "alarm";
    pub const CHAR_NAME_BACKSPACE: &str = "backspace";
    pub const CHAR_NAME_DELETE: &str = "delete";
    pub const CHAR_NAME_ESCAPE: &str = "escape";
    pub const CHAR_NAME_NEWLINE: &str = "newline";
    pub const CHAR_NAME_NULL: &str = "null";
    pub const CHAR_NAME_RETURN: &str = "return";
    pub const CHAR_NAME_SPACE: &str = "space";
    pub const CHAR_NAME_TAB: &str = "tab";

    // --------------------------------------------------------------------------------------------
    // Public Functions
    // --------------------------------------------------------------------------------------------

    pub fn name_to_char(name: &str) -> Option<char> {
        let (prefix, name) = name.split_at(2);
        if prefix == SYNTAX_CHAR_PREFIX {
            if name == CHAR_NAME_NULL {
                Some('\0')
            } else if name == CHAR_NAME_ALARM {
                Some('\u{7}')
            } else if name == CHAR_NAME_BACKSPACE {
                Some('\u{8}')
            } else if name == CHAR_NAME_TAB {
                Some('\u{9}')
            } else if name == CHAR_NAME_NEWLINE {
                Some('\u{a}')
            } else if name == CHAR_NAME_RETURN {
                Some('\u{d}')
            } else if name == CHAR_NAME_ESCAPE {
                Some('\u{1b}')
            } else if name == CHAR_NAME_SPACE {
                Some('\u{20}')
            } else if name == CHAR_NAME_DELETE {
                Some('\u{7f}')
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn char_to_name(c: char) -> Option<String> {
        if c == '\u{0}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_NULL))
        } else if c == '\u{7}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_ALARM))
        } else if c == '\u{8}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_BACKSPACE))
        } else if c == '\u{9}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_TAB))
        } else if c == '\u{a}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_NEWLINE))
        } else if c == '\u{d}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_RETURN))
        } else if c == '\u{1b}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_ESCAPE))
        } else if c == '\u{20}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_SPACE))
        } else if c == '\u{7f}' {
            Some(format!("{}{}", SYNTAX_CHAR_PREFIX, CHAR_NAME_DELETE))
        } else {
            None
        }
    }
}

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
        assert_eq!("#\\xF00".to_string(), Char::from('ༀ').to_repr_string());
    }

    #[test]
    fn test_repr_named() {
        assert_eq!(
            "#\\delete".to_string(),
            Char::from('\u{7f}').to_repr_string()
        );
    }
}
