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

library_name!(
    ID_LIB_SCHEME_CASE_LAMBDA,
    "case-lambda",
    ID_LIB_SCHEME,
    scheme_case_lambda_name
);

pub fn scheme_case_lambda_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "case-lambda" => case_lambda "?");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn case_lambda(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
