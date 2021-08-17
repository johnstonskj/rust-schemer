/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::syntax::parser::parse_number_str;
use crate::types::numbers::{
    BaseFixed, BaseFloat, BaseInt, ExactComplex, ExactReal, InexactComplex, InexactReal, InfNan,
    Integer, Rational, SchemeNum,
};
use crate::types::SchemeValue;
use num::traits::{Num, One, Zero};
use num::{FromPrimitive, ToPrimitive};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    ExactComplex(ExactComplex),
    InexactComplex(InexactComplex),
    ExactReal(ExactReal),
    InexactReal(InexactReal),
    Rational(Rational),
    Integer(Integer),
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! num_match_fn {
    ($number:expr, $fn_name:ident) => {
        match $number {
            Number::ExactComplex(v) => v.$fn_name(),
            Number::InexactComplex(v) => v.$fn_name(),
            Number::ExactReal(v) => v.$fn_name(),
            Number::InexactReal(v) => v.$fn_name(),
            Number::Rational(v) => v.$fn_name(),
            Number::Integer(v) => v.$fn_name(),
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", num_match_fn!(self, to_string))
    }
}

impl FromStr for Number {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_number_str(s)
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

impl From<BaseFloat> for Number {
    fn from(v: BaseFloat) -> Self {
        Self::InexactReal(v.into())
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

impl From<BaseFixed> for Number {
    fn from(v: BaseFixed) -> Self {
        Self::ExactReal(v.into())
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

impl From<BaseInt> for Number {
    fn from(v: BaseInt) -> Self {
        Self::Integer(v.into())
    }
}

impl SchemeValue for Number {
    fn type_name(&self) -> &'static str {
        num_match_fn!(self, type_name)
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Self::Integer(Integer::zero())
    }

    fn is_zero(&self) -> bool {
        num_match_fn!(self, is_zero)
    }
}

impl One for Number {
    fn one() -> Self {
        Self::Integer(Integer::one())
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Num for Number {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_i128(&self) -> Option<i128> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }

    fn to_u128(&self) -> Option<u128> {
        todo!()
    }

    fn to_f32(&self) -> Option<f32> {
        todo!()
    }

    fn to_f64(&self) -> Option<f64> {
        todo!()
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

impl SchemeNum for Number {
    fn is_exact(&self) -> bool {
        num_match_fn!(self, is_exact)
    }

    fn is_finite(&self) -> bool {
        match self {
            Number::InexactReal(v) => v.is_finite(),
            _ => true,
        }
    }

    fn is_nan(&self) -> bool {
        match self {
            Number::InexactReal(v) => v.is_nan(),
            _ => false,
        }
    }

    fn is_real(&self) -> bool {
        num_match_fn!(self, is_real)
    }

    fn is_rational(&self) -> bool {
        num_match_fn!(self, is_rational)
    }

    fn is_integer(&self) -> bool {
        num_match_fn!(self, is_integer)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
