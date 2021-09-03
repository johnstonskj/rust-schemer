/*!
One-line description.

More detailed description, with

# Example

*/

use crate::import::LibraryName;
use crate::schemer::ID_LIB_SCHEMER;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::callable::Callable;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::forms::TYPE_NAME_FORM;
use schemer_lang::eval::procedures::{TYPE_NAME_BUILTIN_PROCEDURE, TYPE_NAME_PROCEDURE};
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::{Boolean, Identifier, MutableRef, SchemeRepr, SchemeString, SchemeValue};

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
    ID_LIB_SCHEMER_REPL,
    "repl",
    ID_LIB_SCHEMER,
    schemer_repl_name
);

pub fn schemer_repl_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "help" => help "procedure_or_form");
    export_builtin!(exports, "inspect" => inspect "obj");
    export_builtin!(exports, "print-current-environment" => print_current_environment);

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn print_current_environment(
    _: Vec<Expression>,
    env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    env.borrow().print();
    Ok(etrue!())
}

fn help(argument: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    let expr = &argument[0];
    Ok(estring!(if let Expression::Procedure(procedure) = expr {
        procedure.signature()
    } else if let Expression::Form(form) = expr {
        form.signature()
    } else {
        unexpected_type!(=> &format!(
            "(or {} {} {})",
            TYPE_NAME_PROCEDURE, TYPE_NAME_BUILTIN_PROCEDURE, TYPE_NAME_FORM
        ), expr)
    }))
}

fn inspect(
    argument: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::String(SchemeString::from(format!(
        "{} => {}",
        &argument[0].to_repr_string(),
        &argument[0].type_name()
    ))))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
