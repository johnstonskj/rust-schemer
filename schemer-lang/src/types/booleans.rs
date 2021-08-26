/*!
One-line description.

More detailed description, with

# Example

*/

use crate::parameters::{get_global_flag, WRITE_BOOLEAN_LONG_FORM};
use crate::read::syntax_str::{
    BOOLEAN_FALSE, BOOLEAN_FALSE_SHORT, BOOLEAN_TRUE, BOOLEAN_TRUE_SHORT,
};
use crate::types::new_type::NewType;
use crate::types::{SchemeRepr, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Boolean = NewType<bool>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Boolean {
    fn to_repr_string(&self) -> String {
        if **self {
            if get_global_flag(WRITE_BOOLEAN_LONG_FORM).unwrap_or_default() {
                BOOLEAN_TRUE
            } else {
                BOOLEAN_TRUE_SHORT
            }
        } else {
            if get_global_flag(WRITE_BOOLEAN_LONG_FORM).unwrap_or_default() {
                BOOLEAN_FALSE
            } else {
                BOOLEAN_FALSE_SHORT
            }
        }
        .to_string()
    }
}

scheme_value!(Boolean, TYPE_NAME_BOOLEAN, "boolean");

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
