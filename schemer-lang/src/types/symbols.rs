/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::types::{SchemeRepr, SchemeValue};
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Identifier(String);

pub type Symbol = Identifier;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Identifier {
    fn to_repr_string(&self) -> String {
        self.0.clone()
    }
}

impl From<Identifier> for String {
    fn from(v: Identifier) -> Self {
        v.0
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: actually check the string
        Ok(Self(s.to_string()))
    }
}

impl Identifier {
    pub fn from_str_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn is_valid(s: &str) -> bool {
        Self::from_str(s).is_ok()
    }
}

// ------------------------------------------------------------------------------------------------

scheme_value!(Symbol, TYPE_NAME_SYMBOL, "symbol");

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
