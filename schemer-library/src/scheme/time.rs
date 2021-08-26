/*!
One-line description.

More detailed description, with

# Example

*/

use num::traits::ToPrimitive;
use schemer_lang::error::Error;
use schemer_lang::types::{InexactReal, Integer};
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

pub fn current_second() -> Result<InexactReal, Error> {
    Ok(InexactReal::from(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64(),
    ))
}

pub fn current_jiffy() -> Result<Integer, Error> {
    // TODO: remove unwraps
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let secs = duration
        .as_secs()
        .to_i64()
        .and_then(|s| s.checked_mul(JIFFIES_PER_SECOND))
        .unwrap();
    let sub_secs = duration.as_micros().to_i64().unwrap();
    Ok(Integer::from(secs.checked_add(sub_secs).unwrap()))
}

pub fn jiffies_per_second() -> Integer {
    Integer::from(JIFFIES_PER_SECOND)
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
