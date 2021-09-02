/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::chars::{char_to_name, TYPE_NAME_CHAR};
use schemer_lang::types::{Boolean, Char, Identifier, Ref, SchemeString, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

pub const JIFFIES_PER_SECOND: i64 = 1_000_000;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_char_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "char-alphanumeric?" => is_alphanumeric "char");
    export_builtin!(exports, "char-control?" => is_control "char");
    export_builtin!(exports, "char->name" => char_name "char");

    exports
}

is_char_a!(is_alphanumeric);
is_char_a!(is_control);

pub fn char_name(arguments: &[Expression], _: &mut Ref<Environment>) -> Result<Expression, Error> {
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
