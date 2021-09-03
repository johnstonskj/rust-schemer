/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::strings::TYPE_NAME_STRING;
use schemer_lang::types::{Boolean, Identifier, MutableRef, SchemeValue};
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_file_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "delete-file" => delete_file "file-name");
    export_builtin!(exports, "file-exists?" => file_exists "file-name");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn delete_file(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            fs::remove_file(file)?;
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    }
    Ok(Expression::Unspecified)
}

fn file_exists(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Boolean(Boolean::from(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            file.exists()
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
