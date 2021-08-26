/*!
One-line description.

More detailed description, with

# Example

*/

use crate::types::{Boolean, Char, Identifier, Number, SchemeRepr, SchemeString};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Comment(Vec<CommentInner>);

#[derive(Clone, Debug)]
pub enum CommentInner {
    Text(String),
    Nested(Comment),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(Identifier),
    Boolean(Boolean),
    Number(Number),
    Character(Char),
    String(SchemeString),
    LeftParen,
    RightParen,
    LeftVector,
    LeftByteVector,
    Apostrophe,
    BackTick,
    Comma,
    CommaAt,
    Dot,
}

pub type Tokens = Vec<Token>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Comment {
    fn to_repr_string(&self) -> String {
        format!(
            "#| {} |#",
            self.0
                .iter()
                .map(|c| match c {
                    CommentInner::Text(v) => v.to_string(),
                    CommentInner::Nested(v) => v.to_repr_string(),
                })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl FromStr for Comment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<String> for Comment {
    fn from(v: String) -> Self {
        Self(vec![CommentInner::Text(v)])
    }
}

impl From<&str> for Comment {
    fn from(v: &str) -> Self {
        Self::from(v.to_string())
    }
}

impl Comment {
    pub fn iter(&self) -> impl Iterator<Item = &CommentInner> {
        self.0.iter()
    }

    pub fn push(&mut self, inner: CommentInner) {
        self.0.push(inner)
    }

    pub fn push_str(&mut self, inner: &str) {
        self.push(CommentInner::Text(inner.to_string()))
    }

    pub fn push_nested(&mut self, inner: Self) {
        self.push(CommentInner::Nested(inner))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Identifier(v) => v.to_repr_string(),
                Token::Boolean(v) => v.to_repr_string(),
                Token::Number(v) => v.to_repr_string(),
                Token::Character(v) => v.to_repr_string(),
                Token::String(v) => v.to_string(),
                Token::LeftParen => "(".to_string(),
                Token::RightParen => "(".to_string(),
                Token::LeftVector => "#(".to_string(),
                Token::LeftByteVector => "#u8(".to_string(),
                Token::Apostrophe => "'".to_string(),
                Token::BackTick => "`".to_string(),
                Token::Comma => ",".to_string(),
                Token::CommaAt => ",@".to_string(),
                Token::Dot => ".".to_string(),
            }
        )
    }
}

impl From<Identifier> for Token {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl From<Boolean> for Token {
    fn from(v: Boolean) -> Self {
        Self::Boolean(v)
    }
}

impl From<bool> for Token {
    fn from(v: bool) -> Self {
        Self::Boolean(v.into())
    }
}

impl From<Number> for Token {
    fn from(v: Number) -> Self {
        Self::Number(v)
    }
}

impl From<char> for Token {
    fn from(v: char) -> Self {
        Self::Character(v.into())
    }
}

impl From<Char> for Token {
    fn from(v: Char) -> Self {
        Self::Character(v)
    }
}

impl From<SchemeString> for Token {
    fn from(v: SchemeString) -> Self {
        Self::String(v)
    }
}

impl From<String> for Token {
    fn from(v: String) -> Self {
        Self::String(SchemeString::from(v))
    }
}

impl From<&str> for Token {
    fn from(v: &str) -> Self {
        Self::from(v.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
