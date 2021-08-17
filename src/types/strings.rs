/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::newtype::NewType;
use crate::types::SchemeValue;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
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

impl Display for SchemeString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for SchemeString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: actually check the string
        Ok(Self(s.to_string()))
    }
}

scheme_value!(SchemeString, TYPE_NAME_STRING, "string");

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
