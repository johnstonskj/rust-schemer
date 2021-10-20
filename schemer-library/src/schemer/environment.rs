/*!
One-line description.

More detailed description, with

# Example

 */

use crate::forms::library::LibraryName;
use crate::schemer::ID_LIB_SCHEMER;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::{Exports, TYPE_NAME_ENVIRONMENT};
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::symbols::TYPE_NAME_SYMBOL;
use schemer_lang::types::{Boolean, Identifier, MutableRef, SchemeString, SchemeValue};

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
    ID_LIB_SCHEMER_ENVIRONMENT,
    "environment",
    ID_LIB_SCHEMER,
    schemer_environment_name
);

pub fn schemer_environment_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "current-environment" => current_environment);
    export_builtin!(exports, "environment?" => is_environment "obj");
    export_builtin!(exports, "environment-name" => name "env");
    export_builtin!(exports, "environment-is-immutable?" => is_immutable "env");
    export_builtin!(exports, "environment-has-parent?" => has_parent "env");
    export_builtin!(exports, "environment-has-binding?" => is_bound "env" "id");
    // export_builtin!(exports, "environment-bound-names" => bound_names "env");
    // export_builtin!(exports, "environment-bindings" => bindings "env");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn current_environment(
    _: Vec<Expression>,
    env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Environment(env.clone()))
}

is_a!(is_environment, Environment);

fn is_immutable(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    match &arguments[0] {
        Expression::Environment(env) => Ok(eboolean!(env.borrow().is_immutable())),
        e => {
            unexpected_type!(TYPE_NAME_ENVIRONMENT, e)
        }
    }
}

fn name(arguments: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    match &arguments[0] {
        Expression::Environment(env) => Ok(match env.borrow().name() {
            None => efalse!(),
            Some(name) => estring!(name.to_string()),
        }),
        e => {
            unexpected_type!(TYPE_NAME_ENVIRONMENT, e)
        }
    }
}

fn has_parent(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    match &arguments[0] {
        Expression::Environment(env) => Ok(eboolean!(env.borrow().has_parent())),
        e => {
            unexpected_type!(TYPE_NAME_ENVIRONMENT, e)
        }
    }
}

fn is_bound(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    let identifier = match &arguments[1] {
        Expression::Quotation(datum) => {
            if datum.is_symbol() {
                Ok(datum.as_symbol().unwrap())
            } else {
                unexpected_type!(TYPE_NAME_SYMBOL, datum)
            }
        }
        e => {
            println!("{:#?}", e);
            unexpected_type!(TYPE_NAME_SYMBOL, e)
        }
    }?;
    match &arguments[0] {
        Expression::Environment(env) => Ok(eboolean!(env.borrow().is_bound(identifier))),
        e => {
            unexpected_type!(TYPE_NAME_ENVIRONMENT, e)
        }
    }
}

// fn bound_names(
//     _: Vec<Expression>,
//     _env: &mut MutableRef<Environment>,
// ) -> Result<Expression, Error> {
//     todo!()
// }
//
// fn bindings(_: Vec<Expression>, _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
//     Ok(Expression::Quotation(Ref::new(Datum::from(vec_to_list(
//         std::env::vars()
//             .map(|(k, v)| {
//                 Datum::List(Pair::cons(
//                     Datum::String(SchemeString::from(k)).into(),
//                     Datum::String(SchemeString::from(v)).into(),
//                 ))
//             })
//             .collect(),
//     )))))
// }

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
