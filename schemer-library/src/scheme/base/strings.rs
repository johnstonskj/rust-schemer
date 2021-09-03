/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::strings::TYPE_NAME_STRING;
use schemer_lang::types::{Identifier, Integer, MutableRef, Number, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_base_string_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "string-length" => string_length "str");

    exports
}

pub fn string_length(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Number(Number::from(Integer::from(
        match &arguments[0] {
            Expression::String(v) => v.len() as Integer,
            e => {
                unexpected_type!(=> TYPE_NAME_STRING, e)
            }
        },
    ))))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
