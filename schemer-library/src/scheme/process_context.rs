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
use schemer_lang::read::datum::Datum;
use schemer_lang::types::lists::vec_to_list;
use schemer_lang::types::{Boolean, Identifier, MutableRef, Pair, Ref, SchemeString};

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
    ID_LIB_SCHEME_PROCESS_CONTEXT,
    "process-context",
    ID_LIB_SCHEME,
    scheme_process_context_name
);

pub fn scheme_process_context_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "command-line" => command_line);
    export_builtin!(exports, "exit" => exit ; "obj");
    export_builtin!(exports, "emergency-exit" => emergency_exit ; "obj");
    export_builtin!(exports, "get-environment-variable" => get_environment_variable "name");
    export_builtin!(exports, "get-environment-variables" => get_environment_variables);

    exports
}

pub fn command_line(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Quotation(Ref::new(Datum::from(vec_to_list(
        std::env::args()
            .map(|s| Datum::String(SchemeString::from(s)))
            .collect(),
    )))))
}

pub fn exit(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    std::process::exit(0)
}

pub fn emergency_exit(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    std::process::exit(0)
}

pub fn get_environment_variable(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(match &arguments[0] {
        Expression::String(name) => match std::env::var(name.as_str()) {
            Err(_) => efalse!(),
            Ok(value) => estring!(value),
        },
        _ => efalse!(),
    })
}

pub fn get_environment_variables(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Quotation(Ref::new(Datum::from(vec_to_list(
        std::env::vars()
            .map(|(k, v)| {
                Datum::List(Pair::cons(
                    Datum::String(SchemeString::from(k)).into(),
                    Datum::String(SchemeString::from(v)).into(),
                ))
            })
            .collect(),
    )))))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
