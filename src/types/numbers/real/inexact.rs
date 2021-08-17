/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::newtype::NewType;
use crate::types::numbers::{BaseFloat, InfNan, SchemeNum};
use crate::types::SchemeValue;
use num::traits::{FromPrimitive, Num, One, ToPrimitive, Zero};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type InexactReal = NewType<BaseFloat>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for InexactReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl FromStr for InexactReal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(f64::from_str(s).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_INEXACT_REAL.to_string(),
                    value: s.to_string(),
                },
            )
        })?))
    }
}

impl From<InfNan> for InexactReal {
    fn from(v: InfNan) -> Self {
        InexactReal::from(BaseFloat::from(v))
    }
}

scheme_value!(InexactReal, TYPE_NAME_INEXACT_REAL, "inexact-real");

impl SchemeNum for InexactReal {
    fn is_exact(&self) -> bool {
        false
    }

    fn is_finite(&self) -> bool {
        self.deref().is_finite()
    }

    fn is_nan(&self) -> bool {
        self.deref().is_nan()
    }

    fn is_real(&self) -> bool {
        true
    }

    fn is_rational(&self) -> bool {
        true
    }

    fn is_integer(&self) -> bool {
        self.deref().fract() == BaseFloat::zero()
    }
}

num_ops!(InexactReal, BaseFloat);

num_primitives!(InexactReal, BaseFloat);

impl Num for InexactReal {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(BaseFloat::from_str_radix(str, radix).map_err(
            |_| {
                Error::from(ErrorKind::Value {
                    kind: TYPE_NAME_INEXACT_REAL.to_string(),
                    value: str.to_string(),
                })
            },
        )?))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
