/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::types::new_type::NewType;
use crate::types::{SchemeRepr, SchemeValue};
use std::borrow::Cow;
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type SchemeString = NewType<String>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn escape_string(s: &str) -> Cow<str> {
    if s.chars().any(char::is_control) {
        s.into()
    } else {
        s.into()
    }
}

pub fn unescape_string(s: &str) -> Cow<str> {
    s.into()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for SchemeString {
    fn to_repr_string(&self) -> String {
        format!("\"{}\"", self.deref())
    }
}

impl FromStr for SchemeString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') && s.ends_with('"') {
            let string = s[1..s.len() - 1].to_string();
            Ok(Self::from(string))
        } else {
            Err(ErrorKind::Value {
                kind: "String".to_string(),
                value: s.to_string(),
            }
            .into())
        }
    }
}

scheme_value!(SchemeString, TYPE_NAME_STRING, "string");

impl SchemeString {
    pub fn new_unchecked(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
