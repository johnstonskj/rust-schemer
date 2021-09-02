/*!
One-line description.

More detailed description, with

# Example

*/

use crate::scheme::base::predicates::scheme_base_predicates_exports;
use crate::scheme::chars::scheme_char_exports;
use crate::scheme::eval::scheme_eval_exports;
use crate::scheme::repl::scheme_repl_exports;
use crate::{make_preset_environment, PresetEnvironmentKind};
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::numbers::TYPE_NAME_INTEGER;
use schemer_lang::types::{Identifier, MutableRef, Number};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_r5rs_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "null-environment" => null_environment "version");
    export_builtin!(exports, "scheme-report-environment" => scheme_report_environment "version");

    exports.import(
        scheme_base_predicates_exports()
            .except(&[
                &Identifier::from_str_unchecked("number?"),
                &Identifier::from_str_unchecked("bytevector?"),
                &Identifier::from_str_unchecked("exact-integer?"),
            ])
            .clone(),
    );
    exports.import(scheme_char_exports());
    exports.import(
        scheme_eval_exports()
            .only(&[&Identifier::from_str_unchecked("eval")])
            .clone(),
    );
    exports.import(scheme_repl_exports());

    exports
}

pub fn null_environment(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    match &arguments[0] {
        Expression::Number(Number::Integer(v)) => Ok(Expression::Environment(
            make_preset_environment(PresetEnvironmentKind::Null(*v))?,
        )),
        _ => Err(Error::from(ErrorKind::UnexpectedType {
            expected: TYPE_NAME_INTEGER.to_string(),
            actual: None,
        })),
    }
}

fn scheme_report_environment(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    match &arguments[0] {
        Expression::Number(Number::Integer(v)) => Ok(Expression::Environment(
            make_preset_environment(PresetEnvironmentKind::Report(*v))?,
        )),
        _ => Err(Error::from(ErrorKind::UnexpectedType {
            expected: TYPE_NAME_INTEGER.to_string(),
            actual: None,
        })),
    }
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
