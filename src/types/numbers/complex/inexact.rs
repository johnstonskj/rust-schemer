/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::newtype::NewType;
use crate::types::numbers::{BaseFloat, SchemeNum};
use crate::types::SchemeValue;
use num::traits::{FromPrimitive, Num, One, ToPrimitive, Zero};
use num::Complex;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type InexactComplex = NewType<Complex<BaseFloat>>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for InexactComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl FromStr for InexactComplex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Complex::from_str(s).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_INEXACT_COMPLEX.to_string(),
                    value: s.to_string(),
                },
            )
        })?))
    }
}
scheme_value!(InexactComplex, TYPE_NAME_INEXACT_COMPLEX, "inexact-complex");

impl SchemeNum for InexactComplex {
    fn is_exact(&self) -> bool {
        false
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
        false
    }
}

num_ops!(InexactComplex, Complex<BaseFloat>);

num_primitives!(InexactComplex, Complex);

impl Num for InexactComplex {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(Complex::from_str_radix(str, radix).map_err(|_| {
            Error::from(ErrorKind::Value {
                kind: TYPE_NAME_INEXACT_COMPLEX.to_string(),
                value: str.to_string(),
            })
        })?))
    }
}

impl InexactComplex {
    pub fn new(real: BaseFloat, imaginary: BaseFloat) -> Self {
        Self(Complex::new(real, imaginary))
    }

    pub fn from_polar(r: BaseFloat, theta: BaseFloat) -> Self {
        Self(Complex::from_polar(r, theta))
    }
}

// impl TryFrom<Integer> for InexactComplex {
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
//         Ok(Self(Complex::new(f64::from(v), f64::zero())))
//     }
// }

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
