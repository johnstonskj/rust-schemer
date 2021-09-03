/*!
One-line description.

More detailed description, with

# Example

 */

use crate::import::LibraryName;
use crate::scheme::ID_LIB_SCHEME;
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

library_name!(ID_LIB_SCHEME_FILE, "file", ID_LIB_SCHEME, scheme_file_name);

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
            unexpected_type!(=> TYPE_NAME_STRING, e)
        }
    }
    Ok(Expression::Unspecified)
}

fn file_exists(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(eboolean!(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            file.exists()
        }
        e => {
            unexpected_type!(=> TYPE_NAME_STRING, e)
        }
    }))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
