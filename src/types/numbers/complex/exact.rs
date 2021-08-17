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
use num::Complex;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type ExactComplex = NewType<Complex<BaseFixed>>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ExactComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl FromStr for ExactComplex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Complex::from_str(s).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_EXACT_COMPLEX.to_string(),
                    value: s.to_string(),
                },
            )
        })?))
    }
}

scheme_value!(ExactComplex, TYPE_NAME_EXACT_COMPLEX, "exact-complex");

impl SchemeNum for ExactComplex {
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

num_ops!(ExactComplex, Complex<BaseFixed>);

num_primitives!(ExactComplex, Complex);

impl Num for ExactComplex {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(Complex::from_str_radix(str, radix).map_err(|e| {
            Error::chain(
                Box::new(e),
                ErrorKind::Value {
                    kind: TYPE_NAME_EXACT_COMPLEX.to_string(),
                    value: str.to_string(),
                },
            )
        })?))
    }
}

impl ExactComplex {
    pub fn new(real: BaseFixed, imaginary: BaseFixed) -> Self {
        Self(Complex::new(real, imaginary))
    }
}

// ------------------------------------------------------------------------------------------------

// impl From<Integer> for ExactComplex {
//     fn from(v: Integer) -> Self {
//         let v: i64 = v.into();
//         Self(Complex::new(Decimal::from(v), Decimal::zero()))
//     }
// }

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "more")]
mod conversion {

    impl TryFrom<InexactComplex> for ExactComplex {
        type Error = Error;

        fn try_from(value: InexactComplex) -> Result<Self, Self::Error> {
            let re: Decimal =
                value
                    .re
                    .to_f64()
                    .and_then(|v| Decimal::from_f64(v))
                    .ok_or(Error::from(ErrorKind::Value {
                        kind: Self::type_name().to_string(),
                        value: value.to_string(),
                    }))?;
            let im: Decimal =
                value
                    .im
                    .to_f64()
                    .and_then(|v| Decimal::from_f64(v))
                    .ok_or(Error::from(ErrorKind::Value {
                        kind: Self::type_name().to_string(),
                        value: value.to_string(),
                    }))?;
            Ok(ExactComplex::from(Complex::new(re, im)))
        }
    }

    impl TryFrom<InexactComplex> for InexactReal {
        type Error = Error;

        fn try_from(value: InexactComplex) -> Result<Self, Self::Error> {
            value
                .to_f64()
                .ok_or(Error::from(ErrorKind::Value {
                    kind: Self::type_name().to_string(),
                    value: value.to_string(),
                }))
                .map(|v| InexactReal::from(v))
        }
    }

    impl TryFrom<InexactComplex> for ExactReal {
        type Error = Error;

        fn try_from(value: InexactComplex) -> Result<Self, Self::Error> {
            let inexact = value.to_f64().ok_or(Error::from(ErrorKind::Value {
                kind: Self::type_name().to_string(),
                value: value.to_string(),
            }))?;
            Ok(ExactReal::from(Decimal::from_f64(inexact).ok_or(
                Error::from(ErrorKind::Value {
                    kind: Self::type_name().to_string(),
                    value: value.to_string(),
                }),
            )?))
        }
    }

    impl TryFrom<InexactComplex> for Rational {
        type Error = Error;

        fn try_from(value: InexactComplex) -> Result<Self, Self::Error> {
            value
                .to_i64()
                .ok_or(Error::from(ErrorKind::Value {
                    kind: Self::type_name().to_string(),
                    value: value.to_string(),
                }))
                .map(|v| Rational::new(v, 1))
        }
    }

    impl TryFrom<InexactComplex> for Integer {
        type Error = Error;

        fn try_from(value: InexactComplex) -> Result<Self, Self::Error> {
            value
                .to_i64()
                .ok_or(
                    ErrorKind::Value {
                        kind: Self::type_name().to_string(),
                        value: value.to_string(),
                    }
                    .into(),
                )
                .map(|v| Integer::from(v))
        }
    }
}
