/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::eval::{Environment, Expression};
use crate::read::syntax_str::{SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR};
use crate::types::{Identifier, MutableRef, SchemeRepr, SchemeValue};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Callable<T>: Clone + Debug + PartialEq + SchemeValue
where
    T: Clone + Debug + PartialEq,
{
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
                " {}{}",
                SYNTAX_LEFT_PARENTHESIS_CHAR,
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
                    format!(" {}", variadic_formal_argument.to_repr_string())
                } else {
                    format!(" . {}", variadic_formal_argument.to_repr_string())
                }
            } else {
                String::new()
            };
        format!(
            "{}{}{}{}{}{}",
            SYNTAX_LEFT_PARENTHESIS_CHAR,
            self.id().to_repr_string(),
            formal_arguments,
            variadic_formal_argument,
            if formal_arguments.is_empty() {
                String::new()
            } else {
                SYNTAX_RIGHT_PARENTHESIS_CHAR.to_string()
            },
            SYNTAX_RIGHT_PARENTHESIS_CHAR
        )
    }

    fn call(
        &self,
        arguments: Vec<T>,
        environment: &mut MutableRef<Environment>,
    ) -> Result<Expression, Error>;
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
