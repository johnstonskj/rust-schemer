/*!
One-line description.

More detailed description, with

# Example

 */

use num::Zero;
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

pub fn scheme_base_number_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "complex?" => is_complex "num");
    export_builtin!(exports, "real?" => is_real "num");
    export_builtin!(exports, "rational?" => is_rational "num");
    export_builtin!(exports, "integer?" => is_integer "num");
    export_builtin!(exports, "exact-integer?" => is_exact_integer "num");
    export_builtin!(exports, "exact?" => is_exact "num");
    export_builtin!(exports, "inexact?" => is_inexact "num");

    export_builtin!(exports, "even?" => is_even "num");
    export_builtin!(exports, "odd?" => is_odd "num");
    export_builtin!(exports, "negative?" => is_negative "num");
    export_builtin!(exports, "positive?" => is_positive "num");
    export_builtin!(exports, "zero?" => is_zero "num");

    exports
}

is_number_a!(is_complex);
is_number_a!(is_real);
is_number_a!(is_rational);
is_number_a!(is_integer);
is_number_a!(is_exact_integer, is_integer);
is_number_a!(is_exact);
is_number_a!(is_inexact);

is_number_a!(is_even);
is_number_a!(is_odd);
is_number_a!(is_positive => |v: &Number| v.is_positive().unwrap_or_default());
is_number_a!(is_negative => |v: &Number| v.is_negative().unwrap_or_default());
is_number_a!(is_zero);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
