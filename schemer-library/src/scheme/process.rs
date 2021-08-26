/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::read::datum::Datum;
use schemer_lang::types::lists::list;
use schemer_lang::types::{Pair, SchemeString};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn command_line() -> Box<Pair> {
    list(
        std::env::args()
            .map(|s| Datum::String(SchemeString::from(s)))
            .collect(),
    )
}

pub fn exit(_obj: Option<Datum>) {
    std::process::exit(0)
}

pub fn emergency_exit(_obj: Option<Datum>) {
    std::process::exit(0)
}

pub fn get_environment_variable(name: &str) -> Option<SchemeString> {
    std::env::var(name).ok().map(|s| SchemeString::from(s))
}

pub fn get_environment_variables() -> Box<Pair> {
    list(
        std::env::vars()
            .map(|(k, v)| {
                Datum::List(Box::new(Pair::cons(
                    Datum::String(SchemeString::from(k)),
                    Datum::String(SchemeString::from(v)),
                )))
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
