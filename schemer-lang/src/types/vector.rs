/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::eval::expression::Evaluate;
use crate::eval::{Environment, Expression};
use crate::read::datum::Datum;
use crate::read::syntax_str::{
    SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR, SYNTAX_VECTOR_PREFIX,
};
use crate::types::{MutableRef, Ref};
use crate::types::{SchemeRepr, SchemeValue};
use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<T>(Vec<Ref<T>>)
where
    T: Clone + Debug + PartialEq + SchemeRepr;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> Default for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T> Deref for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    type Target = Vec<Ref<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<Ref<T>>> for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn from(value: Vec<Ref<T>>) -> Self {
        Self(value)
    }
}

impl<T> From<Vec<T>> for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn from(value: Vec<T>) -> Self {
        Self(value.into_iter().map(|t| Ref::new(t)).collect())
    }
}

impl<T> From<&[T]> for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn from(value: &[T]) -> Self {
        Self::from(value.to_vec())
    }
}

impl<T> From<Vector<T>> for Vec<Ref<T>>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn from(v: Vector<T>) -> Self {
        v.0
    }
}

impl<T> FromIterator<T> for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<T>>())
    }
}

pub const TYPE_NAME_VECTOR: &str = "vector";

impl<T> SchemeRepr for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}{}",
            SYNTAX_VECTOR_PREFIX,
            SYNTAX_LEFT_PARENTHESIS_CHAR,
            self.iter()
                .map(|v| v.to_repr_string())
                .collect::<Vec<String>>()
                .join(" "),
            SYNTAX_RIGHT_PARENTHESIS_CHAR
        )
    }
}

impl<T> SchemeValue for Vector<T>
where
    T: Clone + Debug + PartialEq + SchemeRepr,
{
    fn type_name(&self) -> &'static str
    where
        Self: Sized,
    {
        TYPE_NAME_VECTOR
    }
}

impl Evaluate for Vector<Datum> {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::Vector(self.clone()))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
