/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::types::numbers::{
    ExactComplex, ExactReal, InexactComplex, InexactReal, InfNan, Integer, Number, Rational,
    TYPE_NAME_EXACT_COMPLEX, TYPE_NAME_EXACT_REAL, TYPE_NAME_INEXACT_COMPLEX,
    TYPE_NAME_INEXACT_REAL, TYPE_NAME_INTEGER, TYPE_NAME_RATIONAL,
};
use num::traits::{FromPrimitive, One, ToPrimitive};
use std::convert::TryFrom;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn inexact_complex_to_exact_complex(from: InexactComplex) -> Result<ExactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_COMPLEX, TYPE_NAME_EXACT_COMPLEX);
    ExactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}
pub fn inexact_complex_to_inexact_real(from: InexactComplex) -> Result<InexactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_COMPLEX, TYPE_NAME_INEXACT_REAL);
    InexactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}
pub fn inexact_complex_to_exact_real(from: InexactComplex) -> Result<ExactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_COMPLEX, TYPE_NAME_EXACT_REAL);
    ExactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}
pub fn inexact_complex_to_rational(from: InexactComplex) -> Result<Rational, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_COMPLEX, TYPE_NAME_RATIONAL);
    Rational::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn inexact_complex_to_integer(from: InexactComplex) -> Result<Integer, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_COMPLEX, TYPE_NAME_INTEGER);
    from.to_i64().ok_or_else(&type_error)
}

pub fn exact_complex_to_inexact_complex(from: ExactComplex) -> Result<InexactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_COMPLEX, TYPE_NAME_INEXACT_COMPLEX);
    InexactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_complex_to_inexact_real(from: ExactComplex) -> Result<InexactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_COMPLEX, TYPE_NAME_INEXACT_REAL);
    InexactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_complex_to_exact_real(from: ExactComplex) -> Result<ExactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_COMPLEX, TYPE_NAME_EXACT_REAL);
    ExactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_complex_to_rational(from: ExactComplex) -> Result<Rational, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_COMPLEX, TYPE_NAME_RATIONAL);
    Rational::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_complex_to_integer(from: ExactComplex) -> Result<Integer, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_COMPLEX, TYPE_NAME_INTEGER);
    from.to_i64().ok_or_else(&type_error)
}

pub fn inexact_real_to_inexact_complex(from: InexactReal) -> Result<InexactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_REAL, TYPE_NAME_INEXACT_COMPLEX);
    InexactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn inexact_real_to_exact_complex(from: InexactReal) -> Result<ExactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_REAL, TYPE_NAME_INEXACT_COMPLEX);
    ExactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn inexact_real_to_exact_real(from: InexactReal) -> Result<ExactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_REAL, TYPE_NAME_EXACT_REAL);
    ExactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn inexact_real_to_rational(from: InexactReal) -> Result<Rational, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_REAL, TYPE_NAME_RATIONAL);
    Rational::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn inexact_real_to_integer(from: InexactReal) -> Result<Integer, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INEXACT_REAL, TYPE_NAME_INTEGER);
    from.to_i64().ok_or_else(&type_error)
}

pub fn exact_real_to_inexact_complex(from: ExactReal) -> Result<InexactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_REAL, TYPE_NAME_INEXACT_COMPLEX);
    InexactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_real_to_exact_complex(from: ExactReal) -> Result<ExactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_REAL, TYPE_NAME_EXACT_COMPLEX);
    ExactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_real_to_inexact_real(from: ExactReal) -> Result<InexactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_REAL, TYPE_NAME_INEXACT_REAL);
    InexactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_real_to_rational(from: ExactReal) -> Result<Rational, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_REAL, TYPE_NAME_RATIONAL);
    Rational::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn exact_real_to_integer(from: ExactReal) -> Result<Integer, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_EXACT_REAL, TYPE_NAME_INTEGER);
    from.to_i64().ok_or_else(&type_error)
}

pub fn rational_to_inexact_complex(from: Rational) -> Result<InexactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_RATIONAL, TYPE_NAME_INEXACT_COMPLEX);
    InexactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn rational_to_exact_complex(from: Rational) -> Result<ExactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_RATIONAL, TYPE_NAME_EXACT_COMPLEX);
    ExactComplex::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn rational_to_inexact_real(from: Rational) -> Result<InexactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_RATIONAL, TYPE_NAME_INEXACT_REAL);
    InexactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn rational_to_exact_real(from: Rational) -> Result<ExactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_RATIONAL, TYPE_NAME_EXACT_REAL);
    ExactReal::from_f64(from.to_f64().ok_or_else(&type_error)?).ok_or_else(&type_error)
}

pub fn rational_to_integer(from: Rational) -> Result<Integer, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_RATIONAL, TYPE_NAME_INTEGER);
    from.to_i64().ok_or_else(&type_error)
}

pub fn integer_to_inexact_complex(from: Integer) -> Result<InexactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INTEGER, TYPE_NAME_INEXACT_COMPLEX);
    InexactComplex::from_i64(from).ok_or_else(&type_error)
}

pub fn integer_to_exact_complex(from: Integer) -> Result<ExactComplex, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INTEGER, TYPE_NAME_EXACT_COMPLEX);
    ExactComplex::from_i64(from).ok_or_else(&type_error)
}

pub fn integer_to_inexact_real(from: Integer) -> Result<InexactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INTEGER, TYPE_NAME_INEXACT_REAL);
    InexactReal::from_i64(from).ok_or_else(&type_error)
}

pub fn integer_to_exact_real(from: Integer) -> Result<ExactReal, Error> {
    let type_error = make_type_cast_error(TYPE_NAME_INTEGER, TYPE_NAME_EXACT_REAL);
    ExactReal::from_i64(from).ok_or_else(&type_error)
}

pub fn integer_to_rational(from: Integer) -> Result<Rational, Error> {
    Ok(Rational::new(from, Integer::one()))
}

pub fn exact_to_inexact(from: Number) -> Result<Number, Error> {
    match from {
        Number::InexactComplex(_) => Ok(from),
        Number::ExactComplex(v) => exact_complex_to_inexact_complex(v).map(|v| v.into()),
        Number::InexactReal(_) => Ok(from),
        Number::ExactReal(v) => exact_real_to_inexact_real(v).map(|v| v.into()),
        Number::Rational(v) => rational_to_inexact_real(v).map(|v| v.into()),
        Number::Integer(v) => integer_to_inexact_real(v).map(|v| v.into()),
    }
}

pub fn inexact_to_inexact(from: Number) -> Result<Number, Error> {
    match from {
        Number::InexactComplex(v) => inexact_complex_to_exact_complex(v).map(|v| v.into()),
        Number::ExactComplex(_) => Ok(from),
        Number::InexactReal(v) => inexact_real_to_exact_real(v).map(|v| v.into()),
        Number::ExactReal(_) => Ok(from),
        Number::Rational(_) => Ok(from),
        Number::Integer(_) => Ok(from),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        num_match_fn!(self, to_i64)
    }

    fn to_i128(&self) -> Option<i128> {
        num_match_fn!(self, to_i128)
    }

    fn to_u64(&self) -> Option<u64> {
        num_match_fn!(self, to_u64)
    }

    fn to_u128(&self) -> Option<u128> {
        num_match_fn!(self, to_u128)
    }

    fn to_f32(&self) -> Option<f32> {
        num_match_fn!(self, to_f32)
    }

    fn to_f64(&self) -> Option<f64> {
        num_match_fn!(self, to_f64)
    }
}

impl FromPrimitive for Number {
    fn from_i64(n: i64) -> Option<Self> {
        Integer::from_i64(n).map(|v| v.into())
    }

    fn from_i128(n: i128) -> Option<Self> {
        Integer::from_i128(n).map(|v| v.into())
    }

    fn from_u64(n: u64) -> Option<Self> {
        Integer::from_u64(n).map(|v| v.into())
    }

    fn from_u128(n: u128) -> Option<Self> {
        Integer::from_u128(n).map(|v| v.into())
    }

    fn from_f32(n: f32) -> Option<Self> {
        InexactReal::from_f32(n).map(|v| v.into())
    }

    fn from_f64(n: f64) -> Option<Self> {
        InexactReal::from_f64(n).map(|v| v.into())
    }
}

impl From<InexactComplex> for Number {
    fn from(v: InexactComplex) -> Self {
        Self::InexactComplex(v)
    }
}

impl From<ExactComplex> for Number {
    fn from(v: ExactComplex) -> Self {
        Self::ExactComplex(v)
    }
}

impl From<InexactReal> for Number {
    fn from(v: InexactReal) -> Self {
        Self::InexactReal(v)
    }
}

impl From<ExactReal> for Number {
    fn from(v: ExactReal) -> Self {
        Self::ExactReal(v)
    }
}

impl From<InfNan> for Number {
    fn from(v: InfNan) -> Self {
        Self::InexactReal(v.into())
    }
}

impl From<Rational> for Number {
    fn from(v: Rational) -> Self {
        Self::Rational(v)
    }
}

impl From<Integer> for Number {
    fn from(v: Integer) -> Self {
        Self::Integer(v)
    }
}

impl TryFrom<Number> for Integer {
    type Error = Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::InexactComplex(v) => inexact_complex_to_integer(v),
            Number::ExactComplex(v) => exact_complex_to_integer(v),
            Number::InexactReal(v) => inexact_real_to_integer(v),
            Number::ExactReal(v) => exact_real_to_integer(v),
            Number::Rational(v) => rational_to_integer(v),
            Number::Integer(v) => Ok(v),
        }
    }
}

impl TryFrom<Number> for Rational {
    type Error = Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::InexactComplex(v) => inexact_complex_to_rational(v),
            Number::ExactComplex(v) => exact_complex_to_rational(v),
            Number::InexactReal(v) => inexact_real_to_rational(v),
            Number::ExactReal(v) => exact_real_to_rational(v),
            Number::Rational(v) => Ok(v),
            Number::Integer(v) => integer_to_rational(v),
        }
    }
}

impl TryFrom<Number> for InexactReal {
    type Error = Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::InexactComplex(v) => inexact_complex_to_inexact_real(v),
            Number::ExactComplex(v) => exact_complex_to_inexact_real(v),
            Number::InexactReal(v) => Ok(v),
            Number::ExactReal(v) => exact_real_to_inexact_real(v),
            Number::Rational(v) => rational_to_inexact_real(v),
            Number::Integer(v) => integer_to_inexact_real(v),
        }
    }
}

impl TryFrom<Number> for ExactReal {
    type Error = Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::InexactComplex(v) => inexact_complex_to_exact_real(v),
            Number::ExactComplex(v) => exact_complex_to_exact_real(v),
            Number::InexactReal(v) => inexact_real_to_exact_real(v),
            Number::ExactReal(v) => Ok(v),
            Number::Rational(v) => rational_to_exact_real(v),
            Number::Integer(v) => integer_to_exact_real(v),
        }
    }
}

impl TryFrom<Number> for InexactComplex {
    type Error = Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::InexactComplex(v) => Ok(v),
            Number::ExactComplex(v) => exact_complex_to_inexact_complex(v),
            Number::InexactReal(v) => inexact_real_to_inexact_complex(v),
            Number::ExactReal(v) => exact_real_to_inexact_complex(v),
            Number::Rational(v) => rational_to_inexact_complex(v),
            Number::Integer(v) => integer_to_inexact_complex(v),
        }
    }
}

impl TryFrom<Number> for ExactComplex {
    type Error = Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::InexactComplex(v) => inexact_complex_to_exact_complex(v),
            Number::ExactComplex(v) => Ok(v),
            Number::InexactReal(v) => inexact_real_to_exact_complex(v),
            Number::ExactReal(v) => exact_real_to_exact_complex(v),
            Number::Rational(v) => rational_to_exact_complex(v),
            Number::Integer(v) => integer_to_exact_complex(v),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn make_type_cast_error(from: &'static str, to: &'static str) -> impl Fn() -> Error {
    move || {
        Error::from(ErrorKind::TypeCast {
            from: from.to_string(),
            to: to.to_string(),
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
