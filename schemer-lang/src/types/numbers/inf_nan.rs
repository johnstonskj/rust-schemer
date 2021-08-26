/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::types::numbers::{InexactReal, TYPE_NAME_INEXACT_REAL};
use num::Float;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

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

const NUMERIC_INFINITY_NEGATIVE: &str = "-inf.0";
const NUMERIC_INFINITY_POSITIVE: &str = "+inf.0";
const NUMERIC_NAN_NEGATIVE: &str = "-nan.0";
const NUMERIC_NAN_POSITIVE: &str = "+nan.0";

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
        if s == NUMERIC_INFINITY_POSITIVE {
            Ok(InfNan::PositiveInfinity)
        } else if s == NUMERIC_INFINITY_NEGATIVE {
            Ok(InfNan::NegativeInfinity)
        } else if s == NUMERIC_NAN_POSITIVE {
            Ok(InfNan::PositiveNan)
        } else if s == NUMERIC_NAN_NEGATIVE {
            Ok(InfNan::NegativeNan)
        } else {
            Err(ErrorKind::Value {
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
            InfNan::PositiveInfinity => NUMERIC_INFINITY_POSITIVE,
            InfNan::NegativeInfinity => NUMERIC_INFINITY_NEGATIVE,
            InfNan::PositiveNan => NUMERIC_NAN_POSITIVE,
            InfNan::NegativeNan => NUMERIC_NAN_NEGATIVE,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
