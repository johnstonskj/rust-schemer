/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::newtype::NewType;
use crate::types::numbers::{BaseInt, Integer, SchemeNum};
use crate::types::SchemeValue;
use num::rational::Ratio;
use num::traits::{FromPrimitive, Num, One, ToPrimitive, Zero};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Rational = NewType<Ratio<BaseInt>>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl FromStr for Rational {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Ratio::from_str(s).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_RATIONAL.to_string(),
                    value: s.to_string(),
                },
            )
        })?))
    }
}

scheme_value!(Rational, TYPE_NAME_RATIONAL, "rational");

// impl CastFrom<ExactReal> for Rational {
//     fn cast_from(value: ExactReal) -> Result<Self, Error> {
//         Ratio::from_f64(
//             value
//                 .into_inner()
//                 .to_f64()
//                 .ok_or(Error::from(ErrorKind::TypeCast {
//                     from: TYPE_NAME_EXACT_REAL.to_string(),
//                     to: "float".to_string(),
//                 }))?,
//         )
//         .ok_or(Error::from(ErrorKind::TypeCast {
//             from: "float".to_string(),
//             to: TYPE_NAME_RATIONAL.to_string(),
//         }))
//         .map(|v| v.into())
//     }
// }
//
// impl CastFrom<InexactReal> for Rational {
//     fn cast_from(value: InexactReal) -> Result<Self, Error> {
//         Ratio::from_f64(
//             value
//                 .into_inner()
//                 .to_f64()
//                 .ok_or(Error::from(ErrorKind::TypeCast {
//                     from: TYPE_NAME_INEXACT_REAL.to_string(),
//                     to: "float".to_string(),
//                 }))?,
//         )
//         .ok_or(Error::from(ErrorKind::TypeCast {
//             from: "float".to_string(),
//             to: TYPE_NAME_RATIONAL.to_string(),
//         }))
//         .map(|v| v.into())
//     }
// }
//
// impl CastFrom<Integer> for Rational {
//     fn cast_from(value: Integer) -> Result<Self, Error> {
//         Ok(Rational::from(Ratio::new(
//             value.into_inner(),
//             BaseInt::from(1),
//         )))
//     }
// }

impl SchemeNum for Rational {
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
        todo!()
    }
}

num_ops!(Rational, Ratio<BaseInt>);

num_primitives!(Rational, Ratio);

impl Num for Rational {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(Ratio::from_str_radix(str, radix).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_RATIONAL.to_string(),
                    value: str.to_string(),
                },
            )
        })?))
    }
}

impl From<Integer> for Rational {
    fn from(v: Integer) -> Self {
        Self::new(v.into_inner(), BaseInt::one())
    }
}

impl From<BaseInt> for Rational {
    fn from(v: BaseInt) -> Self {
        Self::new(v, BaseInt::one())
    }
}

impl Rational {
    pub fn new(n: BaseInt, d: BaseInt) -> Self {
        Self(Ratio::new(n, d))
    }
}
