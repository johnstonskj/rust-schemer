/*!
One-line description.

More detailed description, with

# Example

*/

use num::traits::ToPrimitive;
use schemer_lang::error::Error;
use schemer_lang::eval::{Environment, ExportList, Expression, Procedure};
use schemer_lang::types::{Identifier, InexactReal, Integer, MutableRef, Number};
use std::time::{SystemTime, UNIX_EPOCH};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

pub const JIFFIES_PER_SECOND: i64 = 1_000_000;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_time_exports() -> ExportList {
    let mut exports = ExportList::default();

    export_builtin!(exports, "current-second" => current_second);
    export_builtin!(exports, "current-jiffy" => current_jiffy);
    export_builtin!(exports, "jiffies-per-second" => jiffies_per_second);

    exports
}

pub fn current_second(
    _: &[Expression],
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Number(Number::InexactReal(InexactReal::from(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64(),
    ))))
}

pub fn current_jiffy(
    _: &[Expression],
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

    Ok(Expression::Number(Number::Integer(Integer::from(
        Integer::from(secs.checked_add(sub_secs).unwrap()),
    ))))
}

pub fn jiffies_per_second(
    _: &[Expression],
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Number(Number::Integer(Integer::from(
        Integer::from(JIFFIES_PER_SECOND),
    ))))
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
