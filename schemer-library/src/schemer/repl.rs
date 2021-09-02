/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::callable::Callable;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::forms::TYPE_NAME_FORM;
use schemer_lang::eval::procedures::{TYPE_NAME_BUILTIN_PROCEDURE, TYPE_NAME_PROCEDURE};
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::{Identifier, Ref, SchemeRepr, SchemeString, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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
    _: &[Expression],
    env: &mut Ref<Environment>,
) -> Result<Expression, Error> {
    env.print();
    Ok(Expression::Boolean(true.into()))
}

fn help(argument: &[Expression], _: &mut Ref<Environment>) -> Result<Expression, Error> {
    let expr = &argument[0];
    Ok(Expression::String(SchemeString::from(
        if let Expression::Procedure(procedure) = expr {
            procedure.signature()
        } else if let Expression::Form(form) = expr {
            form.signature()
        } else {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: format!(
                    "(or {} {} {})",
                    TYPE_NAME_PROCEDURE, TYPE_NAME_BUILTIN_PROCEDURE, TYPE_NAME_FORM
                ),
                actual: Some(expr.type_name().to_string()),
            }));
        },
    )))
}

fn inspect(argument: &[Expression], _: &mut Ref<Environment>) -> Result<Expression, Error> {
    println!(
        "{} => {}",
        &argument[0].to_repr_string(),
        &argument[0].type_name()
    );
    Ok(Expression::Boolean(true.into()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
