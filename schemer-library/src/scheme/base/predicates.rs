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
use schemer_lang::types::{Boolean, Identifier, Number, Ref, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_base_predicates_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "symbol?" => is_symbol "obj");
    export_builtin!(exports, "boolean?" => is_boolean "obj");
    export_builtin!(exports, "number?" => is_rational "obj");
    export_builtin!(exports, "vector?" => is_vector "obj");
    export_builtin!(exports, "char?" => is_char "obj");
    export_builtin!(exports, "string?" => is_string "obj");
    export_builtin!(exports, "bytevector?" => is_byte_vector "obj");
    export_builtin!(exports, "procedure?" => is_procedure "obj");
    export_builtin!(exports, "null?" => is_null "obj");

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

// TODO: is_even, is_odd, is_pair, is_list,

is_a!(is_symbol, Identifier);
is_a!(is_boolean, Boolean);
is_a!(is_number, Number);
is_a!(is_vector, Vector);
is_a!(is_char, Character);
is_a!(is_string, String);
is_a!(is_byte_vector, ByteVector);
is_a!(is_procedure, Procedure);
is_a!(is_null, Null !);

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
