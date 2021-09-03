/*!
One-line description.

More detailed description, with

# Example

 */

use crate::import::LibraryName;
use crate::scheme::ID_LIB_SCHEME;
use schemer_lang::error::Error;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Form};
use schemer_lang::read::datum::Datum;
use schemer_lang::read::syntax_str::{
    FORM_NAME_DELAY, FORM_NAME_DELAY_FORCE, FORM_NAME_FORCE, FORM_NAME_IS_PROMISE,
    FORM_NAME_MAKE_PROMISE,
};
use schemer_lang::types::{Identifier, MutableRef, Ref};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

library_name!(ID_LIB_SCHEME_LAZY, "lazy", ID_LIB_SCHEME, scheme_lazy_name);

pub fn scheme_lazy_exports() -> Exports {
    let mut exports = Exports::default();

    export_standard_form!(exports, FORM_NAME_DELAY => delay "expression");
    export_standard_form!(exports, FORM_NAME_DELAY_FORCE => delay_force "expression");
    export_standard_form!(exports, FORM_NAME_FORCE => force "promise");
    export_standard_form!(exports, FORM_NAME_IS_PROMISE => is_promise "obj");
    export_standard_form!(exports, FORM_NAME_MAKE_PROMISE => make_promise "obj");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn delay(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn delay_force(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn force(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn is_promise(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn make_promise(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
