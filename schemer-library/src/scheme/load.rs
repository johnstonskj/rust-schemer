/*!
One-line description.

More detailed description, with

# Example

 */

use crate::forms::library::LibraryName;
use crate::scheme::ID_LIB_SCHEME;
use schemer_lang::error::Error;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::{Identifier, MutableRef};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

library_name!(ID_LIB_SCHEME_LOAD, "load", ID_LIB_SCHEME, scheme_load_name);

pub fn scheme_load_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "load" => load "file-name");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn load(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
