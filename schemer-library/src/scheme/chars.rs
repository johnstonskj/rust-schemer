/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::chars::TYPE_NAME_CHAR;
use schemer_lang::types::{Boolean, Char, Identifier, Ref, SchemeValue};

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

    export_builtin!(exports, "char-alphabetic?" => is_alphabetic "char");
    export_builtin!(exports, "char-lower-case?" => is_lower_case "char");
    export_builtin!(exports, "char-numeric?" => is_numeric "char");
    export_builtin!(exports, "char-upper-case?" => is_upper_case "char");
    export_builtin!(exports, "char-whitespace?" => is_whitespace "char");

    exports
}

is_char_a!(is_alphabetic);
is_char_a!(is_lower_case, is_lowercase);
is_char_a!(is_numeric);
is_char_a!(is_upper_case, is_uppercase);
is_char_a!(is_whitespace);

/*
pub fn down_case(arguments: &[Expression], _: &Ref<Environment>) -> Result<Expression, Error> {
    Ok(Expression::Character(Char::from(match &arguments[0] {
        Expression::Character(v) => v.to_lowercase(),
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_CHAR.to_string(),
                actual: e.type_name(),
            }))
        }
    })))
}

pub fn up_case(arguments: &[Expression], _: &Ref<Environment>) -> Result<Expression, Error> {
    Ok(Expression::Character(Char::from(match &arguments[0] {
        Expression::Character(v) => v.to_uppercase(),
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_CHAR.to_string(),
                actual: e.type_name(),
            }))
        }
    })))
}
*/

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
