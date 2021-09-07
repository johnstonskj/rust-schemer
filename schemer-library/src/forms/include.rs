/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::error::Error;
use schemer_lang::eval::{Environment, Expression};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::{MutableRef, Ref};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn include(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn include_ci(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
