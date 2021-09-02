/*!
One-line description.

More detailed description, with

# Example

*/

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

pub fn scheme_eval_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "eval" => eval "expr-or-def" "environment-specifier");
    export_builtin!(exports, "environment" => environment "import-set-1" ; "import-set-n");

    exports
}

pub fn environment(
    _args: &[Expression],
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

pub fn eval(args: &[Expression], _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(args[0].clone())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
