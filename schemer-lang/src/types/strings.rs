/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::expression::Evaluate;
use crate::eval::{Environment, Expression};
use crate::read::syntax_str::{
    SYNTAX_BYTE_VECTOR_PREFIX, SYNTAX_DOUBLE_QUOTE_CHAR, SYNTAX_LEFT_PARENTHESIS_CHAR,
    SYNTAX_RIGHT_PARENTHESIS_CHAR,
};
use crate::types::new_type::NewType;
use crate::types::{MutableRef, SchemeRepr, SchemeValue};
use std::borrow::Cow;
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type SchemeString = NewType<String>;

pub type ByteVector = NewType<Vec<u8>>;

pub const TYPE_NAME_BYTE: &str = "u8";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn escape_string(s: &str) -> Cow<str> {
    if s.chars().any(char::is_control) {
        s.into()
    } else {
        s.into()
    }
}

pub fn unescape_string(s: &str) -> Cow<str> {
    s.into()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl FromStr for SchemeString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(SYNTAX_DOUBLE_QUOTE_CHAR) && s.ends_with(SYNTAX_DOUBLE_QUOTE_CHAR) {
            let string = s[1..s.len() - 1].to_string();
            Ok(Self::from(string))
        } else {
            Err(ErrorKind::ParseValue {
                kind: TYPE_NAME_STRING.to_string(),
                value: s.to_string(),
            }
            .into())
        }
    }
}

impl SchemeRepr for SchemeString {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}",
            SYNTAX_DOUBLE_QUOTE_CHAR,
            self.deref(),
            SYNTAX_DOUBLE_QUOTE_CHAR
        )
    }
}

scheme_value!(SchemeString, TYPE_NAME_STRING, "string");

impl Evaluate for SchemeString {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::String(self.clone()))
    }
}

impl SchemeString {
    pub fn new_unchecked(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeRepr for ByteVector {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}{}",
            SYNTAX_BYTE_VECTOR_PREFIX,
            SYNTAX_LEFT_PARENTHESIS_CHAR,
            self.iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            SYNTAX_RIGHT_PARENTHESIS_CHAR
        )
    }
}

scheme_value!(ByteVector, TYPE_NAME_BYTE_VECTOR, "byte-vector");

impl Evaluate for ByteVector {
    fn eval(&self, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(Expression::ByteVector(self.clone()))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
