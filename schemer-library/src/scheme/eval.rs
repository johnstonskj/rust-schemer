/*!
One-line description.

More detailed description, with

# Example

*/

use crate::{ImportSet, Library};
use schemer_lang::error::Error;
use schemer_lang::read::datum::Datum;
use schemer_lang::semantics::{datum_to_expression, Expression};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct Environment {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn environment(_import_sets: &[ImportSet]) -> Result<Environment, Error> {
    Ok(Environment {})
}

pub fn eval(expr: Expression, _environment: &Environment) -> Result<Expression, Error> {
    Ok(expr)
}

pub fn eval_datum(datum: Datum, _environment: &Environment) -> Result<Expression, Error> {
    eval(datum_to_expression(datum)?, _environment)
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
