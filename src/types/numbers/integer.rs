/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::newtype::NewType;
use crate::types::numbers::{BaseInt, SchemeNum};
use crate::types::SchemeValue;
use num::traits::{Num, One, Zero};
use num::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, Div, Mul, Rem, Sub};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Integer = NewType<BaseInt>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl FromStr for Integer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BaseInt::from_str(s).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_INTEGER.to_string(),
                    value: s.to_string(),
                },
            )
        })?))
    }
}

scheme_value!(Integer, TYPE_NAME_INTEGER, "integer");

impl SchemeNum for Integer {
    fn is_exact(&self) -> bool {
        true
    }

    fn is_finite(&self) -> bool {
        true
    }

    fn is_nan(&self) -> bool {
        false
    }

    fn is_real(&self) -> bool {
        true
    }

    fn is_rational(&self) -> bool {
        true
    }

    fn is_integer(&self) -> bool {
        true
    }
}

num_ops!(Integer, BaseInt);

num_primitives!(Integer);

impl FromPrimitive for Integer {
    fn from_i64(n: i64) -> Option<Self> {
        if let Some(v) = <BaseInt>::from_i64(n) {
            Some(Self(v))
        } else {
            None
        }
    }

    fn from_i128(n: i128) -> Option<Self> {
        if let Some(v) = <BaseInt>::from_i128(n) {
            Some(Self(v))
        } else {
            None
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        if let Some(v) = <BaseInt>::from_u64(n) {
            Some(Self(v))
        } else {
            None
        }
    }

    fn from_u128(n: u128) -> Option<Self> {
        if let Some(v) = <BaseInt>::from_u128(n) {
            Some(Self(v))
        } else {
            None
        }
    }
}

impl Num for Integer {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(BaseInt::from_str_radix(str, radix).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_INTEGER.to_string(),
                    value: str.to_string(),
                },
            )
        })?))
    }
}
