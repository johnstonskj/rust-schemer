/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::expression::Evaluate;
use crate::eval::{Environment, Expression};
use crate::types::{MutableRef, SchemeRepr, SchemeValue};
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Identifier(String);

pub const TYPE_NAME_SYMBOL: &str = "symbol";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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

impl SchemeRepr for Identifier {
    fn to_repr_string(&self) -> String {
        self.0.clone()
    }
}

impl SchemeValue for Identifier {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_SYMBOL
    }
}

impl Evaluate for Identifier {
    fn eval(&self, environment: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        if let Some(value) = environment.borrow().get(self) {
            Ok(value.clone())
        } else {
            Err(Error::from(ErrorKind::UnboundVariable { name: self.clone() }).into())
        }
    }
}

impl Identifier {
    pub fn from_str_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn is_valid(s: &str) -> bool {
        Self::from_str(s).is_ok()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
