/*!
One-line description.

More detailed description, with

# Example

*/

use crate::forms::library::LibraryName;
use crate::scheme::ID_LIB_SCHEME;
use num::traits::ToPrimitive;
use schemer_lang::error::Error;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::{Identifier, InexactReal, Integer, MutableRef, Number};
use std::time::{SystemTime, UNIX_EPOCH};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

pub const JIFFIES_PER_SECOND: Integer = 1_000_000;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

library_name!(ID_LIB_SCHEME_TIME, "time", ID_LIB_SCHEME, scheme_time_name);

pub fn scheme_time_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "current-second" => current_second);
    export_builtin!(exports, "current-jiffy" => current_jiffy);
    export_builtin!(exports, "jiffies-per-second" => jiffies_per_second);

    exports
}

pub fn current_second(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(einexact_real!(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs_f64()))
}

pub fn current_jiffy(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    // TODO: remove unwraps
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let secs = duration
        .as_secs()
        .to_i64()
        .and_then(|s| s.checked_mul(JIFFIES_PER_SECOND))
        .unwrap();
    let sub_secs = duration.as_micros().to_i64().unwrap();

    Ok(einteger!(secs.checked_add(sub_secs).unwrap()))
}

pub fn jiffies_per_second(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(einteger!(Integer::from(JIFFIES_PER_SECOND)))
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
