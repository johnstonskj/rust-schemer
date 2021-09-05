/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::eval::expression::Evaluate;
use crate::eval::{Environment, Expression};
use crate::parameters::{get_global_flag, WRITE_BOOLEAN_LONG_FORM};
use crate::read::syntax_str::{
    VALUE_BOOLEAN_FALSE, VALUE_BOOLEAN_FALSE_SHORT, VALUE_BOOLEAN_TRUE, VALUE_BOOLEAN_TRUE_SHORT,
};
use crate::types::new_type::NewType;
use crate::types::{MutableRef, SchemeRepr, SchemeValue};
use std::ops::Deref;

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
                VALUE_BOOLEAN_TRUE
            } else {
                VALUE_BOOLEAN_TRUE_SHORT
            }
        } else {
            if get_global_flag(WRITE_BOOLEAN_LONG_FORM).unwrap_or_default() {
                VALUE_BOOLEAN_FALSE
            } else {
                VALUE_BOOLEAN_FALSE_SHORT
            }
        }
        .to_string()
    }
}

scheme_value!(Boolean, TYPE_NAME_BOOLEAN, "boolean");

impl Evaluate for Boolean {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Boolean(self.clone()))
    }
}

impl Boolean {
    pub fn is_true(&self) -> bool {
        *self.deref() == true
    }
    pub fn is_false(&self) -> bool {
        *self.deref() == false
    }
}
// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
