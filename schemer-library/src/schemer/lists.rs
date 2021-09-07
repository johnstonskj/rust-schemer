/*!
One-line description.

More detailed description, with

# Example

 */

use crate::forms::library::LibraryName;
use crate::schemer::ID_LIB_SCHEMER;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::lists::TYPE_NAME_LIST;
use schemer_lang::types::{Boolean, Identifier, MutableRef, Pair, SchemeValue};

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
    ID_LIB_SCHEMER_LISTS,
    "lists",
    ID_LIB_SCHEMER,
    schemer_lists_name
);

pub fn schemer_lists_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "proper-list?" => is_proper_list "obj");

    exports
}

is_list_a!(is_proper_list);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
