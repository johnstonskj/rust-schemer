/*!
One-line description.

More detailed description, with

# Example

| Base           | Alias | Not Big    | Big        |
|----------------|-------|------------|------------|
| Integer        | Int   | i64        | BigInt     |
| Fixed-Point    | FxP   | Decimal    | BigDecimal |
| Floating-Point | FlP   | f64        | f64        |


| Tower    | Exact        | Inexact       |
|----------|--------------|---------------|
| Complex  | Complex<FxP> | Complex<FlP>  |
| Real     | FxP          | FlP           |
| Rational | Ratio<Int>   | N/A           |
| Integer  | Int          | N/A           |

*/

use crate::read::syntax_str::{
    EMPTY_STR, SYNTAX_MATH_COMPLEX_CHAR, SYNTAX_MATH_MINUS, SYNTAX_MATH_MINUS_CHAR,
    SYNTAX_MATH_PLUS, SYNTAX_MATH_PLUS_CHAR,
};
use crate::types::{MutableRef, SchemeRepr, SchemeValue};
use num::complex::Complex;
use num::rational::Ratio;
use num::traits::{Num, Signed, ToPrimitive, Zero};
use rust_decimal::Decimal;
use std::fmt::Display;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "big-num-x")]
mod bignum {
    use bigdecimal::BigDecimal;
    use bigdecimal::BigInt;

    pub type Integer = BigInt;

    pub type Rational = Ratio<Integer>;

    pub type ExactReal = BigDecimal;
}

pub type Integer = i64;

pub type Rational = Ratio<Integer>;

pub type ExactReal = Decimal;

pub type InexactReal = f64;

pub type ExactComplex = Complex<ExactReal>;

pub type InexactComplex = Complex<InexactReal>;

pub const TYPE_NAME_NUMBER: &str = "number";

pub const TYPE_NAME_INTEGER: &str = "integer";
pub const TYPE_NAME_RATIONAL: &str = "rational";
pub const TYPE_NAME_REAL: &str = "real";
pub const TYPE_NAME_EXACT_REAL: &str = "exact-real";
pub const TYPE_NAME_INEXACT_REAL: &str = "inexact-real";
pub const TYPE_NAME_COMPLEX: &str = "complex";
pub const TYPE_NAME_EXACT_COMPLEX: &str = "exact-complex";
pub const TYPE_NAME_INEXACT_COMPLEX: &str = "inexact-complex";

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    InexactComplex(InexactComplex),
    ExactComplex(ExactComplex),
    InexactReal(InexactReal),
    ExactReal(ExactReal),
    Rational(Rational),
    Integer(Integer),
}

pub trait SchemeNum: Num + Sized + Display + FromStr {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeNum for Integer {}

impl SchemeRepr for Integer {
    fn to_repr_string(&self) -> String {
        self.to_string()
    }
}

impl SchemeValue for Integer {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_INTEGER
    }
}

impl Evaluate for Integer {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Number(Number::from(self.clone())))
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeNum for Rational {}

impl SchemeRepr for Rational {
    fn to_repr_string(&self) -> String {
        self.to_string()
    }
}

impl SchemeValue for Rational {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_RATIONAL
    }
}

impl Evaluate for Rational {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Number(Number::from(self.clone())))
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeNum for ExactReal {}

impl SchemeRepr for ExactReal {
    fn to_repr_string(&self) -> String {
        self.to_string()
    }
}

impl SchemeValue for ExactReal {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_EXACT_REAL
    }
}

impl Evaluate for ExactReal {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Number(Number::from(self.clone())))
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeNum for InexactReal {}

impl SchemeRepr for InexactReal {
    fn to_repr_string(&self) -> String {
        if self.is_infinite() && self.is_sign_positive() {
            InfNan::PositiveInfinity.to_string()
        } else if self.is_infinite() && self.is_sign_negative() {
            InfNan::NegativeInfinity.to_string()
        } else if self.is_nan() && self.is_sign_positive() {
            InfNan::PositiveNan.to_string()
        } else if self.is_nan() && self.is_sign_negative() {
            InfNan::NegativeNan.to_string()
        } else {
            self.to_string()
        }
    }
}

impl SchemeValue for InexactReal {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_INEXACT_REAL
    }
}

impl Evaluate for InexactReal {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Number(Number::from(self.clone())))
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeNum for ExactComplex {}

impl SchemeRepr for ExactComplex {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            if self.re.is_sign_negative() {
                SYNTAX_MATH_MINUS
            } else {
                EMPTY_STR
            },
            if self.re.is_sign_negative() || !self.re.is_zero() {
                self.re.to_repr_string()
            } else {
                String::new()
            },
            if self.im.is_sign_negative() {
                SYNTAX_MATH_MINUS_CHAR
            } else {
                SYNTAX_MATH_PLUS_CHAR
            },
            if !self.im.is_zero() {
                self.im.to_repr_string()
            } else {
                String::new()
            },
            SYNTAX_MATH_COMPLEX_CHAR
        )
    }
}

impl SchemeValue for ExactComplex {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_EXACT_COMPLEX
    }
}

impl Evaluate for ExactComplex {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Number(Number::from(self.clone())))
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeNum for InexactComplex {}

impl SchemeRepr for InexactComplex {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}{}",
            if self.re.is_sign_negative() || !self.re.is_zero() {
                self.re.to_repr_string()
            } else {
                String::new()
            },
            if self.im.is_sign_positive() && !self.im.is_infinite() && !self.im.is_nan() {
                SYNTAX_MATH_PLUS
            } else {
                EMPTY_STR
            },
            if !self.im.is_zero() {
                self.im.to_repr_string()
            } else {
                String::new()
            },
            SYNTAX_MATH_COMPLEX_CHAR
        )
    }
}

impl SchemeValue for InexactComplex {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_INEXACT_COMPLEX
    }
}

impl Evaluate for InexactComplex {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Number(Number::from(self.clone())))
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Number {
    fn to_repr_string(&self) -> String {
        num_match_fn!(self, to_repr_string)
    }
}

impl SchemeValue for Number {
    fn type_name(&self) -> &'static str {
        num_match_fn!(self, type_name)
    }
}

impl Evaluate for Number {
    fn eval(&self, environment: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        num_match_fn!(self, eval, environment)
    }
}

impl Number {
    pub fn is_exact(&self) -> bool {
        matches!(
            self,
            Number::ExactComplex(_)
                | Number::ExactReal(_)
                | Number::Rational(_)
                | Number::Integer(_)
        )
    }

    pub fn is_inexact(&self) -> bool {
        matches!(self, Number::InexactComplex(_) | Number::InexactReal(_))
    }

    pub fn is_finite(&self) -> bool {
        match self {
            Number::InexactReal(v) => v.is_finite(),
            _ => true,
        }
    }

    pub fn is_infinite(&self) -> bool {
        match self {
            Number::InexactReal(v) => v.is_infinite(),
            _ => true,
        }
    }

    pub fn is_nan(&self) -> bool {
        match self {
            Number::InexactReal(v) => v.is_nan(),
            _ => false,
        }
    }

    pub fn is_positive(&self) -> Option<bool> {
        match self {
            Number::ExactComplex(_) | Number::InexactComplex(_) => None,
            Number::ExactReal(v) => Some(v.is_sign_positive()),
            Number::InexactReal(v) => Some(v.is_sign_positive()),
            Number::Rational(v) => Some(v.is_positive()),
            Number::Integer(v) => Some(v.is_positive()),
        }
    }

    pub fn is_negative(&self) -> Option<bool> {
        match self {
            Number::ExactComplex(_) | Number::InexactComplex(_) => None,
            Number::ExactReal(v) => Some(v.is_sign_negative()),
            Number::InexactReal(v) => Some(v.is_sign_negative()),
            Number::Rational(v) => Some(v.is_negative()),
            Number::Integer(v) => Some(v.is_negative()),
        }
    }

    pub fn is_odd(&self) -> bool {
        !self.is_even()
    }

    pub fn is_even(&self) -> bool {
        match self {
            Number::ExactComplex(v) => {
                v % ExactComplex::from(ExactReal::from(2)) == ExactComplex::zero()
            }
            Number::InexactComplex(v) => {
                v % InexactComplex::from(InexactReal::from(2)) == InexactComplex::zero()
            }
            Number::ExactReal(v) => v % ExactReal::from(2) == ExactReal::zero(),
            Number::InexactReal(v) => v % InexactReal::from(2) == InexactReal::zero(),
            Number::Rational(v) => v % Rational::from(2) == Rational::zero(),
            Number::Integer(v) => v % Integer::from(2) == Integer::zero(),
        }
    }

    pub fn is_exact_complex(&self) -> bool {
        matches!(self, Self::ExactComplex(_)) || self.is_exact_real()
    }

    pub fn as_exact_complex(&self) -> Option<&ExactComplex> {
        match self {
            Self::ExactComplex(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_inexact_complex(&self) -> bool {
        matches!(self, Self::InexactComplex(_)) || self.is_inexact_real()
    }

    pub fn as_inexact_complex(&self) -> Option<&InexactComplex> {
        match self {
            Self::InexactComplex(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_complex(&self) -> bool {
        self.is_exact_complex() || self.is_inexact_complex()
    }

    pub fn is_exact_real(&self) -> bool {
        matches!(self, Self::ExactReal(_)) || self.is_rational()
    }

    pub fn as_exact_real(&self) -> Option<&ExactComplex> {
        match self {
            Self::ExactComplex(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_inexact_real(&self) -> bool {
        matches!(self, Self::InexactReal(_)) || self.is_rational()
    }

    pub fn as_inexact_real(&self) -> Option<&InexactComplex> {
        match self {
            Self::InexactComplex(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_real(&self) -> bool {
        self.is_exact_real() || self.is_inexact_real()
    }

    pub fn is_rational(&self) -> bool {
        matches!(self, Self::Rational(_)) || self.is_integer()
    }

    pub fn as_rational(&self) -> Option<&Rational> {
        match self {
            Self::Rational(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn real_part(&self) -> Number {
        match self {
            Number::InexactComplex(v) => v.re.into(),
            Number::ExactComplex(v) => v.re.into(),
            _ => self.clone().simplify(),
        }
    }

    pub fn imaginary_part(&self) -> Number {
        match self {
            Number::InexactComplex(v) => v.im.into(),
            Number::ExactComplex(v) => v.im.into(),
            _ => Self::zero(),
        }
    }

    pub fn is_integer_representation(&self) -> bool {
        match self {
            Number::ExactComplex(v) => v.re.fract().is_zero() && v.im.is_zero(),
            Number::InexactComplex(v) => v.re.fract().is_zero() && v.im.is_zero(),
            Number::ExactReal(v) => v.fract().is_zero(),
            Number::InexactReal(v) => v.fract().is_zero(),
            Number::Rational(v) => v.is_integer(),
            Number::Integer(_) => true,
        }
    }

    pub fn as_integer(&self) -> Option<&Integer> {
        match self {
            Self::Integer(v) => Some(v),
            _ => None,
        }
    }

    pub fn simplify(self) -> Self {
        match self {
            Number::InexactComplex(v) => {
                if v.im.is_zero() {
                    if v.re.fract().is_zero() {
                        Integer::from(v.re.to_i64().unwrap()).into()
                    } else {
                        v.re.into()
                    }
                } else {
                    self
                }
            }
            Number::ExactComplex(v) => {
                if v.im.is_zero() {
                    if v.re.fract().is_zero() {
                        Integer::from(v.re.to_i64().unwrap()).into()
                    } else {
                        v.re.into()
                    }
                } else {
                    self
                }
            }
            Number::InexactReal(v) => {
                if v.fract().is_zero() {
                    Integer::from(v.to_i64().unwrap()).into()
                } else {
                    v.into()
                }
            }
            Number::ExactReal(v) => {
                if v.fract().is_zero() {
                    Integer::from(v.to_i64().unwrap()).into()
                } else {
                    v.into()
                }
            }
            Number::Rational(v) => {
                if v.is_integer() {
                    Integer::from(v.to_integer()).into()
                } else {
                    self
                }
            }
            Number::Integer(_) => self,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod conv;

mod inf_nan;
use crate::error::Error;
use crate::eval::expression::Evaluate;
use crate::eval::{Environment, Expression};
pub use inf_nan::InfNan;

pub mod op;
