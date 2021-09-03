/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::error::Error;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::{Boolean, Identifier, MutableRef};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_base_type_predicates_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "symbol?" => is_symbol "obj");
    export_builtin!(exports, "boolean?" => is_boolean "obj");
    export_builtin!(exports, "number?" => is_number "obj");
    export_builtin!(exports, "vector?" => is_vector "obj");
    export_builtin!(exports, "char?" => is_char "obj");
    export_builtin!(exports, "string?" => is_string "obj");
    export_builtin!(exports, "bytevector?" => is_byte_vector "obj");
    export_builtin!(exports, "procedure?" => is_procedure "obj");
    export_builtin!(exports, "list?" => is_list "obj");
    export_builtin!(exports, "null?" => is_null "obj");

    exports
}

// TODO: is_pair, is_list,

is_a!(is_symbol, Identifier);
is_a!(is_boolean, Boolean);
is_a!(is_number, Number);
is_a!(is_vector, Vector);
is_a!(is_char, Character);
is_a!(is_string, String);
is_a!(is_byte_vector, ByteVector);
is_a!(is_procedure, Procedure);
is_a!(is_null, Null !);

fn is_list(
    mut arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(eboolean!(match arguments.remove(0) {
        Expression::List(_) | Expression::Null => true,
        Expression::Quotation(datum) => datum.is_list_or_null(),
        _ => false,
    }))
}

// ------------------------------------------------------------------------------------------------
// Implementations(
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
