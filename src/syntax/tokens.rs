/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::types::booleans::Boolean;
use crate::types::chars::Char;
use crate::types::numbers::Number;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Identifier(String);

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(Identifier),
    Boolean(Boolean),
    Number(Number),
    Character(Char),
    String(String),
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

pub fn string_from_str(s: &str) -> Result<Token, Error> {
    if s.starts_with('"') && s.ends_with('"') {
        let string = s[1..s.len() - 1].to_string();
        Ok(Token::String(string))
    } else {
        Err(ErrorKind::Value {
            kind: "String".to_string(),
            value: s.to_string(),
        }
        .into())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Identifier> for String {
    fn from(v: Identifier) -> Self {
        v.0
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: actually check the string
        Ok(Self(s.to_string()))
    }
}

impl Identifier {
    pub fn is_valid(s: &str) -> bool {
        Self::from_str(s).is_ok()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#| {} |#",
            self.0
                .iter()
                .map(|c| match c {
                    CommentInner::Text(v) => v.to_string(),
                    CommentInner::Nested(v) => v.to_string(),
                })
                .collect::<Vec<String>>()
                .join(" ")
        )
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
/* ------------------------------------------------------------------------------------------------

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.exact { "#e" } else { "" }, self.value)
    }
}

impl Number {
    pub fn exact(value: NumberInner) -> Self {
        Self { exact: true, value }
    }
    pub fn inexact(value: NumberInner) -> Self {
        Self {
            exact: false,
            value,
        }
    }
    pub fn is_exact(&self) -> bool {
        self.exact
    }
    pub fn is_inexact(&self) -> bool {
        !self.exact
    }
    pub fn value(&self) -> &NumberInner {
        &self.value
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Complex64> for NumberInner {
    fn from(v: Complex64) -> Self {
        Self::Complex(v)
    }
}

impl From<Real> for NumberInner {
    fn from(v: Real) -> Self {
        Self::Real(v)
    }
}

impl From<Integer> for NumberInner {
    fn from(v: Integer) -> Self {
        Real::Integer(v).into()
    }
}

impl From<Rational> for NumberInner {
    fn from(v: Rational) -> Self {
        Real::Rational(v).into()
    }
}

impl From<Decimal> for NumberInner {
    fn from(v: Decimal) -> Self {
        Real::Decimal(v).into()
    }
}

impl From<InfNan> for NumberInner {
    fn from(v: InfNan) -> Self {
        Real::InfNan(v).into()
    }
}

impl Display for NumberInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Complex(v) =>
                    format!("{}{}i", float_to_string(&v.re), float_to_string(&v.im)),
                Self::Real(v) => v.to_string(),
            }
        )
    }
}

fn float_to_string(f: &f64) -> String {
    if f.is_nan() && f.is_sign_positive() {
        InfNan::PositiveNan.to_string()
    } else if f.is_nan() && f.is_sign_negative() {
        InfNan::NegativeNan.to_string()
    } else if f.is_infinite() && f.is_sign_positive() {
        InfNan::PositiveInfinity.to_string()
    } else if f.is_infinite() && f.is_sign_negative() {
        InfNan::NegativeInfinity.to_string()
    } else {
        f.to_string()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Integer> for Real {
    fn from(v: Integer) -> Self {
        Self::Integer(v)
    }
}

impl From<Rational> for Real {
    fn from(v: Rational) -> Self {
        Self::Rational(v)
    }
}

impl From<Decimal> for Real {
    fn from(v: Decimal) -> Self {
        Self::Decimal(v)
    }
}

impl From<InfNan> for Real {
    fn from(v: InfNan) -> Self {
        Self::InfNan(v)
    }
}

impl TryFrom<Real> for f64 {
    type Error = Error;

    fn try_from(v: Real) -> Result<Self, Self::Error> {
        match v {
            Real::Integer(v) => v.to_f64().ok_or(
                ErrorKind::NumericTruncation {
                    from: "integer".to_string(),
                    to: "f64".to_string(),
                }
                .into(),
            ),
            Real::Rational(v) => v.to_f64().ok_or(
                ErrorKind::NumericTruncation {
                    from: "rational".to_string(),
                    to: "f64".to_string(),
                }
                .into(),
            ),
            Real::Decimal(v) => v.to_f64().ok_or(
                ErrorKind::NumericTruncation {
                    from: "decimal".to_string(),
                    to: "f64".to_string(),
                }
                .into(),
            ),
            Real::InfNan(v) => Ok(v.into()),
        }
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer(v) => v.to_string(),
                Self::Rational(v) => v.to_string(),
                Self::Decimal(v) => v.to_string(),
                Self::InfNan(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for InfNan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InfNan::PositiveInfinity => "+inf.0",
                InfNan::NegativeInfinity => "-inf.0",
                InfNan::PositiveNan => "+nan.0",
                InfNan::NegativeNan => "-nan.0",
            }
        )
    }
}

impl From<InfNan> for f64 {
    fn from(v: InfNan) -> Self {
        match v {
            InfNan::PositiveInfinity => f64::INFINITY,
            InfNan::NegativeInfinity => f64::NEG_INFINITY,
            InfNan::PositiveNan => f64::NAN,
            InfNan::NegativeNan => -f64::NAN,
        }
    }
}

impl FromStr for InfNan {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "+inf.0" {
            Ok(InfNan::PositiveInfinity)
        } else if s == "-inf.0" {
            Ok(InfNan::NegativeInfinity)
        } else if s == "+nan.0" {
            Ok(InfNan::PositiveNan)
        } else if s == "-nan.0" {
            Ok(InfNan::NegativeNan)
        } else {
            Err(ErrorKind::Value {
                kind: "InfNan".to_string(),
                value: s.to_string(),
            }
            .into())
        }
    }
}

 ------------------------------------------------------------------------------------------------ */

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Identifier(v) => v.to_string(),
                Token::Boolean(v) => v.to_string(),
                Token::Number(v) => v.to_string(),
                Token::Character(v) => v.to_string(),
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

impl From<String> for Token {
    fn from(v: String) -> Self {
        Self::String(v)
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
