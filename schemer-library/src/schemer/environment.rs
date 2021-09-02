/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::Error;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::{Boolean, Identifier, MutableRef};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn schemer_environment_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "current-environment" => current_environment);
    export_builtin!(exports, "environment?" => is_environment "obj");
    export_builtin!(exports, "environment-is-immutable?" => is_immutable "env");
    export_builtin!(exports, "environment-has-parent?" => has_parent "env");
    export_builtin!(exports, "environment-bound-names" => bound_names "env");
    export_builtin!(exports, "environment-bindings" => bindings "env");
    export_builtin!(exports, "environment-has-binding?" => is_bound "env" "id");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn current_environment(
    _: &[Expression],
    env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Environment(env.clone()))
}

is_a!(is_environment, Environment);

fn is_immutable(
    arguments: &[Expression],
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Boolean(Boolean::from(
        if let Expression::Environment(env) = &arguments[0] {
            env.borrow().is_immutable()
        } else {
            false
        },
    )))
}

fn has_parent(
    arguments: &[Expression],
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Boolean(Boolean::from(
        if let Expression::Environment(env) = &arguments[0] {
            env.borrow().has_parent()
        } else {
            false
        },
    )))
}

fn is_bound(_: &[Expression], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn bound_names(_: &[Expression], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn bindings(_: &[Expression], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
