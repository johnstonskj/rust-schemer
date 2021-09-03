/*!
One-line description.

More detailed description, with

# Example

 */

use crate::import::library_path;
use crate::import::LibraryName;
use crate::schemer::ID_LIB_SCHEMER;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::Expression;
use schemer_lang::types::{Identifier, SchemeString};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

library_name!(
    ID_LIB_SCHEMER_LOAD,
    "load",
    ID_LIB_SCHEMER,
    schemer_load_name
);

pub fn schemer_load_exports() -> Exports {
    let mut exports = Exports::default();

    let _ = exports.insert(
        id_from_str!("schemer-library-search-path"),
        estring!(library_path().to_string()),
    );

    exports
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
