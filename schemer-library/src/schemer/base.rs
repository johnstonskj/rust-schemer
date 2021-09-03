/*!
One-line description.

More detailed description, with

# Example

 */

use crate::import::LibraryName;
use crate::schemer::ID_LIB_SCHEMER;
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

library_name!(
    ID_LIB_SCHEMER_BASE,
    "base",
    ID_LIB_SCHEMER,
    schemer_base_name
);

pub fn schemer_base_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "exact-complex?" => is_exact_complex "num");
    export_builtin!(exports, "inexact-complex?" => is_inexact_complex "num");
    export_builtin!(exports, "exact-real?" => is_exact_real "num");
    export_builtin!(exports, "inexact-real?" => is_inexact_real "num");

    exports
}

is_number_a!(is_exact_complex);
is_number_a!(is_inexact_complex);
is_number_a!(is_exact_real);
is_number_a!(is_inexact_real);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
