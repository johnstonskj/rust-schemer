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

use crate::types::SchemeValue;
pub use integer::Integer;
use num::traits::{FromPrimitive, Num, ToPrimitive};

#[cfg(not(feature = "big-numbers"))]
use rust_decimal::Decimal;

#[cfg(feature = "big-numbers")]
use bigdecimal::BigInt;

#[cfg(feature = "big-numbers")]
use bigdecimal::BigDecimal;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[cfg(not(feature = "big-numbers"))]
pub(crate) type BaseInt = i128;

#[cfg(feature = "big-numbers")]
pub(crate) type BaseInt = BigInt;

#[cfg(not(feature = "big-numbers"))]
pub(crate) type BaseFixed = Decimal;

#[cfg(feature = "big-numbers")]
pub(crate) type BaseFixed = BigDecimal;

#[cfg(not(feature = "big-numbers"))]
pub(crate) type BaseFloat = f64;

#[cfg(feature = "big-numbers")]
pub(crate) type BaseFloat = f64;

// ------------------------------------------------------------------------------------------------

pub trait SchemeNum: Num + ToPrimitive + FromPrimitive + SchemeValue {
    fn is_exact(&self) -> bool;

    fn is_finite(&self) -> bool;

    fn is_nan(&self) -> bool;

    fn is_real(&self) -> bool;

    fn is_rational(&self) -> bool;

    fn is_integer(&self) -> bool;
}

pub(crate) const SIGN_NEGATIVE: &str = "-";
pub(crate) const SIGN_POSITIVE: &str = "+";

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! num_ops {
    ($outer:ty, $inner:ty) => {
        impl Zero for $outer {
            fn zero() -> Self {
                Self(<$inner>::zero())
            }

            fn is_zero(&self) -> bool {
                self.deref().is_zero()
            }
        }

        impl One for $outer {
            fn one() -> Self {
                Self(<$inner>::one())
            }
        }

        impl Add for $outer {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.deref().add(rhs.deref()))
            }
        }

        impl Sub for $outer {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.deref().sub(rhs.deref()))
            }
        }

        impl Mul for $outer {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.deref().mul(rhs.deref()))
            }
        }

        impl Div for $outer {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self(self.deref().div(rhs.deref()))
            }
        }

        impl Rem for $outer {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.deref().rem(rhs.deref()))
            }
        }
    };
}

macro_rules! num_primitives {
    ($outer:ty, $inner:ty) => {
        num_primitives! { $outer }

        impl FromPrimitive for $outer {
            fn from_i64(n: i64) -> Option<Self> {
                if let Some(v) = <$inner>::from_i64(n) {
                    Some(Self(v))
                } else {
                    None
                }
            }

            fn from_u64(n: u64) -> Option<Self> {
                if let Some(v) = <$inner>::from_u64(n) {
                    Some(Self(v))
                } else {
                    None
                }
            }
        }
    };
    ($outer:ty) => {
        impl ToPrimitive for $outer {
            fn to_i64(&self) -> Option<i64> {
                self.deref().to_i64()
            }

            fn to_u64(&self) -> Option<u64> {
                self.deref().to_u64()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod number;
pub use number::Number;

pub mod complex;
pub use complex::{ExactComplex, InexactComplex};

pub mod real;
pub use real::{ExactReal, InexactReal, InfNan};

pub mod rational;
pub use rational::Rational;

pub mod integer;
