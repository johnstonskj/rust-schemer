/*!
One-line description.

More detailed description, with

# Example

*/

use crate::scheme::base::ports::scheme_base_ports_exports;
use crate::scheme::base::strings::scheme_base_string_exports;
use crate::scheme::base::types::scheme_base_type_predicates_exports;
use crate::scheme::base::write::scheme_base_write_exports;
use crate::scheme::chars::scheme_char_exports;
use crate::scheme::eval::scheme_eval_exports;
use crate::scheme::repl::scheme_repl_exports;
use crate::scheme::write::scheme_write_exports;
use crate::{make_preset_environment, PresetEnvironmentKind};
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::numbers::TYPE_NAME_INTEGER;
use schemer_lang::types::{Identifier, MutableRef, Number, SchemeValue};

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

    // (scheme base) ------------------------------------------------------------------------------

    exports.import(
        scheme_base_ports_exports()
            .except(&[&Identifier::from_str_unchecked("current-error-port")])
            .clone(),
    );
    exports.import(scheme_base_string_exports());
    exports.import(
        scheme_base_type_predicates_exports()
            .except(&[
                &Identifier::from_str_unchecked("number?"),
                &Identifier::from_str_unchecked("bytevector?"),
                &Identifier::from_str_unchecked("exact-integer?"),
            ])
            .clone(),
    );
    exports.import(
        scheme_base_write_exports()
            .only(&[
                &Identifier::from_str_unchecked("newline"),
                &Identifier::from_str_unchecked("write-char"),
            ])
            .clone(),
    );

    // (scheme char) ------------------------------------------------------------------------------

    exports.import(scheme_char_exports());

    // (scheme eval) ------------------------------------------------------------------------------

    exports.import(
        scheme_eval_exports()
            .only(&[&Identifier::from_str_unchecked("eval")])
            .clone(),
    );
    // (scheme repl) ------------------------------------------------------------------------------

    exports.import(scheme_repl_exports());

    // (scheme write) -----------------------------------------------------------------------------

    exports.import(
        scheme_write_exports()
            .only(&[
                &Identifier::from_str_unchecked("display"),
                &Identifier::from_str_unchecked("write"),
            ])
            .clone(),
    );

    // specials while testing >>>>

    // <<<< specials while testing

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
        e => unexpected_type!(TYPE_NAME_INTEGER, e),
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
        e => unexpected_type!(TYPE_NAME_INTEGER, e),
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
