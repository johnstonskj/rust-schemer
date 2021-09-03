/*!
One-line description.

More detailed description, with

# Example

*/

#[macro_export]
macro_rules! id_from_str {
    ($v:expr) => {
        Identifier::from_str_unchecked($v)
    };
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod datum;

pub mod errors;

pub mod export;

pub mod expression;
