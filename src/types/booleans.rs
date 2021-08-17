/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::newtype::NewType;
use crate::types::SchemeValue;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Boolean = NewType<bool>;

pub const BOOLEAN_TRUE: &str = "#true";
pub const BOOLEAN_TRUE_SHORT: &str = "#t";
pub const BOOLEAN_FALSE: &str = "#false";
pub const BOOLEAN_FALSE_SHORT: &str = "#f";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if **self { "#t" } else { "#f" })
    }
}

impl FromStr for Boolean {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == BOOLEAN_TRUE || s == BOOLEAN_TRUE_SHORT {
            Ok(Self(true))
        } else if s == BOOLEAN_FALSE || s == BOOLEAN_FALSE_SHORT {
            Ok(Self(false))
        } else {
            Err(ErrorKind::Value {
                kind: TYPE_NAME_BOOLEAN.to_string(),
                value: s.to_string(),
            }
            .into())
        }
    }
}

scheme_value!(Boolean, TYPE_NAME_BOOLEAN, "boolean");

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
