/*!
One-line description.

More detailed description, with

# Example

*/

use crate::read::syntax_str::{
    PSEUDO_SYNTAX_COLON_CHAR, PSEUDO_SYNTAX_LEFT_PROCEDURE, PSEUDO_SYNTAX_RANGE,
    PSEUDO_SYNTAX_RIGHT_PROCEDURE, SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR,
};
use crate::types::{Identifier, SchemeRepr, SchemeValue};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Callable: Clone + Debug + PartialEq + SchemeValue {
    fn id(&self) -> &Identifier;

    fn rename(&mut self, id: Identifier);

    fn formal_arguments(&self) -> &Vec<Identifier>;

    fn variadic_formal_argument(&self) -> &Option<Identifier>;

    fn has_variadic_argument(&self) -> bool {
        self.variadic_formal_argument().is_some()
    }

    fn min_arg_count(&self) -> usize {
        self.formal_arguments().len()
    }

    fn max_arg_count(&self) -> Option<usize> {
        if self.has_variadic_argument() {
            None
        } else {
            Some(self.min_arg_count())
        }
    }

    fn signature(&self) -> String {
        let formal_arguments = if self.formal_arguments().is_empty() {
            String::new()
        } else {
            format!(
                " {}",
                self.formal_arguments()
                    .iter()
                    .map(|id| id.to_repr_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        };
        let variadic_formal_argument =
            if let Some(variadic_formal_argument) = self.variadic_formal_argument() {
                if self.formal_arguments().is_empty() {
                    variadic_formal_argument.to_repr_string()
                } else {
                    format!(" . {}", variadic_formal_argument.to_repr_string())
                }
            } else {
                String::new()
            };
        format!(
            "{}{}{}{}{}",
            SYNTAX_LEFT_PARENTHESIS_CHAR,
            self.id().to_repr_string(),
            formal_arguments,
            variadic_formal_argument,
            SYNTAX_RIGHT_PARENTHESIS_CHAR
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T: Callable> SchemeRepr for T {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            PSEUDO_SYNTAX_LEFT_PROCEDURE,
            self.type_name(),
            PSEUDO_SYNTAX_COLON_CHAR,
            self.id().to_repr_string(),
            PSEUDO_SYNTAX_COLON_CHAR,
            self.min_arg_count(),
            PSEUDO_SYNTAX_RANGE,
            self.max_arg_count().unwrap_or_default(),
            PSEUDO_SYNTAX_RIGHT_PROCEDURE
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
