/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::chars::{char_to_name, TYPE_NAME_CHAR};
use schemer_lang::types::{Boolean, Char, Identifier, MutableRef, SchemeString, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn schemer_chars_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "char-alphanumeric?" => is_alphanumeric "char");
    export_builtin!(exports, "char-control?" => is_control "char");
    export_builtin!(exports, "char->name" => char_name "char");

    exports
}

is_char_a!(is_alphanumeric);
is_char_a!(is_control);

pub fn char_name(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::String(SchemeString::from(
        match &arguments[0] {
            Expression::Character(v) => char_to_name(**v),
            e => {
                return Err(Error::from(ErrorKind::UnexpectedType {
                    expected: TYPE_NAME_CHAR.to_string(),
                    actual: Some(e.type_name().to_string()),
                }))
            }
        },
    )))
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
