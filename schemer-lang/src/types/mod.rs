/*!
One-line description.

More detailed description, with

# Example

*/

use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait SchemeRepr {
    fn to_repr_string(&self) -> String;
}

pub trait SchemeValue: Debug + SchemeRepr {
    fn type_name(&self) -> &'static str;
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! scheme_value {
    ($type_name:ty, $const_name:ident, $value:expr) => {
        pub const $const_name: &str = $value;

        impl SchemeValue for $type_name {
            fn type_name(&self) -> &'static str
            where
                Self: Sized,
            {
                $const_name
            }
        }
    };
}

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

pub mod booleans;
pub use booleans::Boolean;

pub mod chars;
pub use chars::Char;

pub mod exceptions;

pub mod lists;
pub use lists::Pair;

pub mod numbers;
pub use numbers::{
    ExactComplex, ExactReal, InexactComplex, InexactReal, InfNan, Integer, Number, Rational,
};

pub mod strings;
pub use strings::SchemeString;

pub mod new_type;

pub mod symbols;
pub use symbols::{Identifier, Symbol};
