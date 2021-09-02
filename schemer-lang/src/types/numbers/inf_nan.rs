/*!
One-line description.

More detailed description, with

# Example

*/

use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use num::Float;

use crate::error::{Error, ErrorKind};
use crate::read::syntax_str::{
    VALUE_MATH_INFINITY_NEGATIVE, VALUE_MATH_INFINITY_POSITIVE, VALUE_MATH_NAN_NEGATIVE,
    VALUE_MATH_NAN_POSITIVE,
};
use crate::types::numbers::{InexactReal, TYPE_NAME_INEXACT_REAL};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum InfNan {
    PositiveInfinity,
    NegativeInfinity,
    PositiveNan,
    NegativeNan,
}

const TYPE_NAME_INF_NAN: &str = "inf-nan";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for InfNan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<InfNan> for InexactReal {
    fn from(v: InfNan) -> Self {
        match v {
            InfNan::PositiveInfinity => InexactReal::infinity(),
            InfNan::NegativeInfinity => InexactReal::neg_infinity(),
            InfNan::PositiveNan => InexactReal::nan(),
            InfNan::NegativeNan => -InexactReal::nan(),
        }
    }
}

impl TryFrom<InexactReal> for InfNan {
    type Error = Error;

    fn try_from(v: InexactReal) -> Result<Self, Self::Error> {
        match (v.is_sign_negative(), v.is_infinite(), v.is_nan()) {
            (false, true, _) => Ok(InfNan::PositiveInfinity),
            (true, true, _) => Ok(InfNan::NegativeInfinity),
            (false, _, true) => Ok(InfNan::PositiveNan),
            (true, _, true) => Ok(InfNan::NegativeNan),
            _ => Err(ErrorKind::TypeCast {
                from: TYPE_NAME_INEXACT_REAL.to_string(),
                to: TYPE_NAME_INF_NAN.to_string(),
            }
            .into()),
        }
    }
}

impl FromStr for InfNan {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == VALUE_MATH_INFINITY_POSITIVE {
            Ok(InfNan::PositiveInfinity)
        } else if s == VALUE_MATH_INFINITY_NEGATIVE {
            Ok(InfNan::NegativeInfinity)
        } else if s == VALUE_MATH_NAN_POSITIVE {
            Ok(InfNan::PositiveNan)
        } else if s == VALUE_MATH_NAN_NEGATIVE {
            Ok(InfNan::NegativeNan)
        } else {
            Err(ErrorKind::ParseValue {
                kind: TYPE_NAME_INF_NAN.to_string(),
                value: s.to_string(),
            }
            .into())
        }
    }
}

impl InfNan {
    pub fn as_str(&self) -> &'static str {
        match self {
            InfNan::PositiveInfinity => VALUE_MATH_INFINITY_POSITIVE,
            InfNan::NegativeInfinity => VALUE_MATH_INFINITY_NEGATIVE,
            InfNan::PositiveNan => VALUE_MATH_NAN_POSITIVE,
            InfNan::NegativeNan => VALUE_MATH_NAN_NEGATIVE,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
