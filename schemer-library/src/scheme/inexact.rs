/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::numbers::TYPE_NAME_NUMBER;
use schemer_lang::types::{Boolean, Identifier, MutableRef, Number, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_inexact_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "finite?" => is_finite "num");
    export_builtin!(exports, "infinite?" => is_infinite "num");
    export_builtin!(exports, "nan?" => is_nan "num");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

is_number_a!(is_finite);
is_number_a!(is_infinite);
is_number_a!(is_nan);

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
