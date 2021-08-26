/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::types::numbers::{
    ExactComplex, ExactReal, InexactComplex, InexactReal, Integer, Number, Rational,
};
use num::traits::{One, Zero};
use std::cmp::max;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Rem, Sub};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NumberKind {
    InexactComplex = 6,
    ExactComplex = 5,
    InexactReal = 4,
    ExactReal = 3,
    Rational = 2,
    Integer = 1,
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! num_binary_op {
    ($op_trait:ident, $op_fn:ident) => {
        num_binary_op_impl!($op_trait, $op_fn);

        num_binary_op_inner_impl!($op_trait, ExactComplex, ExactComplex, $op_fn);

        num_binary_op_inner_impl!($op_trait, InexactComplex, InexactComplex, $op_fn);

        num_binary_op_inner_impl!($op_trait, ExactReal, ExactReal, $op_fn);

        num_binary_op_inner_impl!($op_trait, InexactReal, InexactReal, $op_fn);

        num_binary_op_inner_impl!($op_trait, Rational, Rational, $op_fn);

        num_binary_op_inner_impl!($op_trait, Integer, Integer, $op_fn);
    };
}

macro_rules! num_binary_op_impl {
    ($op_trait:ident, $op_fn:ident) => {
        impl $op_trait for Number {
            type Output = Self;

            fn $op_fn(self, rhs: Self) -> Self::Output {
                match rhs {
                    Number::ExactComplex(rhs) => self.$op_fn(rhs).into(),
                    Number::InexactComplex(rhs) => self.$op_fn(rhs).into(),
                    Number::ExactReal(rhs) => self.$op_fn(rhs).into(),
                    Number::InexactReal(rhs) => self.$op_fn(rhs).into(),
                    Number::Rational(rhs) => self.$op_fn(rhs).into(),
                    Number::Integer(rhs) => self.$op_fn(rhs).into(),
                }
            }
        }
    };
}

macro_rules! num_binary_op_inner_impl {
    ($op_trait:ident, $num_type:ty, $num_kind:ident, $op_fn:ident) => {
        impl $op_trait<$num_type> for Number {
            type Output = Self;

            fn $op_fn(self, rhs: $num_type) -> Self::Output {
                match max(number_kind(&self), NumberKind::$num_kind) {
                    NumberKind::InexactComplex => {
                        num_op_pair!(self, rhs, add, pair_to_inexact_complex)
                    }
                    NumberKind::ExactComplex => num_op_pair!(self, rhs, add, pair_to_exact_complex),
                    NumberKind::InexactReal => num_op_pair!(self, rhs, add, pair_to_inexact_real),
                    NumberKind::ExactReal => num_op_pair!(self, rhs, add, pair_to_exact_real),
                    NumberKind::Rational => num_op_pair!(self, rhs, add, pair_to_rational),
                    NumberKind::Integer => num_op_pair!(self, rhs, add, pair_to_integer),
                }
            }
        }
    };
}

macro_rules! num_op_pair {
    ($lhs:expr, $rhs:expr, $op:ident, $pair_fn:ident) => {{
        let (lhs, rhs) = $pair_fn($lhs, Number::from($rhs)).unwrap();
        Number::from(lhs.$op(rhs))
    }};
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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

num_binary_op!(Add, add);

num_binary_op!(Sub, sub);

num_binary_op!(Mul, mul);

num_binary_op!(Div, div);

num_binary_op!(Rem, rem);

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        num_op!(self, neg)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

fn pair_to_inexact_complex(
    l: Number,
    r: Number,
) -> Result<(InexactComplex, InexactComplex), Error> {
    Ok((InexactComplex::try_from(l)?, InexactComplex::try_from(r)?))
}

fn pair_to_exact_complex(l: Number, r: Number) -> Result<(ExactComplex, ExactComplex), Error> {
    Ok((ExactComplex::try_from(l)?, ExactComplex::try_from(r)?))
}

fn pair_to_inexact_real(l: Number, r: Number) -> Result<(InexactReal, InexactReal), Error> {
    Ok((InexactReal::try_from(l)?, InexactReal::try_from(r)?))
}

fn pair_to_exact_real(l: Number, r: Number) -> Result<(ExactReal, ExactReal), Error> {
    Ok((ExactReal::try_from(l)?, ExactReal::try_from(r)?))
}

fn pair_to_rational(l: Number, r: Number) -> Result<(Rational, Rational), Error> {
    Ok((Rational::try_from(l)?, Rational::try_from(r)?))
}

fn pair_to_integer(l: Number, r: Number) -> Result<(Integer, Integer), Error> {
    Ok((Integer::try_from(l)?, Integer::try_from(r)?))
}

fn number_kind(n: &Number) -> NumberKind {
    match n {
        Number::ExactComplex(_) => NumberKind::ExactComplex,
        Number::InexactComplex(_) => NumberKind::InexactComplex,
        Number::ExactReal(_) => NumberKind::ExactReal,
        Number::InexactReal(_) => NumberKind::InexactReal,
        Number::Rational(_) => NumberKind::Rational,
        Number::Integer(_) => NumberKind::Integer,
    }
}
