/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::newtype::NewType;
use crate::types::numbers::{BaseFixed, SchemeNum};
use crate::types::SchemeValue;
use num::traits::{FromPrimitive, Num, One, ToPrimitive, Zero};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type ExactReal = NewType<BaseFixed>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ExactReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl FromStr for ExactReal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BaseFixed::from_str(s).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_EXACT_REAL.to_string(),
                    value: s.to_string(),
                },
            )
        })?))
    }
}

scheme_value!(ExactReal, TYPE_NAME_EXACT_REAL, "exact-real");

impl SchemeNum for ExactReal {
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

    #[cfg(not(feature = "big-numbers"))]
    fn is_integer(&self) -> bool {
        self.deref().fract() == BaseFixed::zero()
    }

    #[cfg(feature = "big-numbers")]
    fn is_integer(&self) -> bool {
        self.deref().is_integer()
    }
}

num_ops!(ExactReal, BaseFixed);

num_primitives!(ExactReal, BaseFixed);

impl Num for ExactReal {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(BaseFixed::from_str_radix(str, radix).map_err(
            |e| {
                Error::chain(
                    Box::new(e),
                    ErrorKind::Value {
                        kind: TYPE_NAME_EXACT_REAL.to_string(),
                        value: str.to_string(),
                    },
                )
            },
        )?))
    }
}

//
// impl TryFrom<Integer> for ExactReal {
//     type Error = Error;
//
//     fn try_from(v: Integer) -> Result<Self, Self::Error> {
//         let v: i64 = v.into();
//         let v = u32::try_from(v).map_err(|e| {
//             Error::chain(
//                 Box::new(e),
//                 ErrorKind::NumericTruncation {
//                     from: Integer::type_name().to_string(),
//                     to: Self::type_name().to_string(),
//                 },
//             )
//         })?;
//         Ok(Self(f64::from(v)))
//     }
// }
//
// impl From<Integer> for ExactReal {
//     fn from(v: Integer) -> Self {
//         let v: i64 = v.into();
//         Self(Decimal::from(v))
//     }
// }

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
