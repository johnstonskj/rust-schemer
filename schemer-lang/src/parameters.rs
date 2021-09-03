/*!
One-line description.

More detailed description, with

# Example

*/

use crate::read::datum::Datum;
use crate::types::lists::vec_to_list;
use crate::types::{Boolean, Identifier, Pair};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::RwLock;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const WRITE_BOOLEAN_LONG_FORM: &str = "write-boolean-long-form";
pub const WRITE_CONS_LONG_FORM: &str = "write-cons-long-form";
pub const WRITE_QUOTE_LONG_FORM: &str = "write-quote-long-form";
pub const DEBUG_SHOW_TOKEN_TREE: &str = "debug-show-token-tree";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref GLOBAL_FLAGS: HashMap<&'static str, RwLock<bool>> = init_global_flags();
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_global_flag(s: &str) -> Option<bool> {
    let maybe_value = GLOBAL_FLAGS.get(s).and_then(|f| f.read().ok());
    if let Some(value) = maybe_value {
        Some(*value.deref())
    } else {
        None
    }
}

pub fn global_flags() -> Pair {
    vec_to_list(
        GLOBAL_FLAGS
            .keys()
            .map(|k| {
                Datum::List(Pair::cons(
                    Datum::Symbol(Identifier::from_str_unchecked(k)).into(),
                    Datum::Boolean(Boolean::from(get_global_flag(k).unwrap_or_default())).into(),
                ))
            })
            .collect(),
    )
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_global_flags() -> HashMap<&'static str, RwLock<bool>> {
    vec![
        (WRITE_BOOLEAN_LONG_FORM, false),
        (WRITE_CONS_LONG_FORM, false),
        (WRITE_QUOTE_LONG_FORM, false),
        (DEBUG_SHOW_TOKEN_TREE, false),
    ]
    .into_iter()
    .map(|(k, v)| (k, RwLock::new(v)))
    .collect()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
