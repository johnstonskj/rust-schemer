/*!
One-line description.

More detailed description, with

# Example

*/

use crate::parameters::{get_global_flag, WRITE_QUOTE_LONG_FORM};
use crate::types::lists::NULL_LIST_REPR_STRING;
use crate::types::{
    Boolean, Char, Identifier, Integer, Number, Pair, SchemeRepr, SchemeString, Symbol,
};
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Label = u128;

#[derive(Clone, Debug, PartialEq)]
pub enum Datum {
    Boolean(Boolean),
    Number(Number),
    Character(Char),
    String(SchemeString),
    Symbol(Symbol),
    ByteVector(Vec<u8>),
    List(Box<Pair>),
    Vector(Vec<Box<Datum>>),
    Abbreviation(AbbreviationPrefix, Box<Datum>),
    Labeled(Label, Box<Datum>),
    LabelRef(Label),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AbbreviationPrefix {
    Apostrophe,
    BackTick,
    Comma,
    CommaAt,
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

impl From<Boolean> for Datum {
    fn from(v: Boolean) -> Self {
        Self::Boolean(v)
    }
}

impl From<bool> for Datum {
    fn from(v: bool) -> Self {
        Self::Boolean(v.into())
    }
}

impl From<Number> for Datum {
    fn from(v: Number) -> Self {
        Self::Number(v)
    }
}

impl From<Integer> for Datum {
    fn from(v: Integer) -> Self {
        Self::Number(v.into())
    }
}

impl From<Char> for Datum {
    fn from(v: Char) -> Self {
        Self::Character(v)
    }
}

impl From<char> for Datum {
    fn from(v: char) -> Self {
        Self::Character(v.into())
    }
}

impl From<SchemeString> for Datum {
    fn from(v: SchemeString) -> Self {
        Self::String(v)
    }
}

impl From<String> for Datum {
    fn from(v: String) -> Self {
        Self::String(SchemeString::from(v))
    }
}

impl From<&str> for Datum {
    fn from(v: &str) -> Self {
        Self::String(SchemeString::new_unchecked(v))
    }
}

impl From<Identifier> for Datum {
    fn from(v: Identifier) -> Self {
        Self::Symbol(v)
    }
}

impl From<Vec<u8>> for Datum {
    fn from(v: Vec<u8>) -> Self {
        Self::ByteVector(v)
    }
}

impl From<&[u8]> for Datum {
    fn from(v: &[u8]) -> Self {
        Self::ByteVector(v.to_vec())
    }
}

impl From<Label> for Datum {
    fn from(v: Label) -> Self {
        Self::LabelRef(v)
    }
}
impl From<Box<Pair>> for Datum {
    fn from(v: Box<Pair>) -> Self {
        Self::List(v)
    }
}

impl From<Pair> for Datum {
    fn from(v: Pair) -> Self {
        Self::List(Box::new(v))
    }
}

impl SchemeRepr for Datum {
    fn to_repr_string(&self) -> String {
        match self {
            Self::Boolean(v) => v.to_repr_string(),
            Self::Number(v) => v.to_repr_string(),
            Self::Character(v) => v.to_repr_string(),
            Self::String(v) => v.to_repr_string(),
            Self::Symbol(v) => v.to_repr_string(),
            Self::List(v) => v.to_repr_string(),
            Self::ByteVector(vs) => format!(
                "#u8({})",
                vs.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Self::Vector(vs) => format!(
                "#({})",
                vs.iter()
                    .map(|v| v.to_repr_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Self::Abbreviation(p, v) => {
                if get_global_flag(WRITE_QUOTE_LONG_FORM).unwrap_or_default() {
                    format!("({}{})", p, v.to_repr_string())
                } else {
                    format!("{}{}", p, v.to_repr_string())
                }
            }
            Self::Labeled(l, v) => format!("#{}={}", l, v.to_repr_string()),
            Self::LabelRef(l) => format!("#{}#", l),
            Datum::Null => NULL_LIST_REPR_STRING.to_string(),
        }
    }
}

impl Datum {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Datum::Boolean(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Datum::Number(_))
    }

    pub fn is_character(&self) -> bool {
        matches!(self, Datum::Character(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Datum::String(_))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Datum::Symbol(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Datum::List(_))
    }

    pub fn is_list_or_null(&self) -> bool {
        matches!(self, Datum::List(_) | Datum::Null)
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Datum::Vector(_))
    }

    pub fn is_byte_vector(&self) -> bool {
        matches!(self, Datum::ByteVector(_))
    }

    pub fn is_abbreviation(&self) -> bool {
        matches!(self, Datum::Abbreviation(_, _))
    }

    pub fn is_labeled(&self) -> bool {
        matches!(self, Datum::Labeled(_, _))
    }

    pub fn is_label_ref(&self) -> bool {
        matches!(self, Datum::LabelRef(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Datum::Null)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for AbbreviationPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if get_global_flag(WRITE_QUOTE_LONG_FORM).unwrap_or_default() {
                match self {
                    AbbreviationPrefix::Apostrophe => "quote",
                    AbbreviationPrefix::BackTick => "quasiquote",
                    AbbreviationPrefix::Comma => "unquote",
                    AbbreviationPrefix::CommaAt => "unquote-splicing",
                }
            } else {
                match self {
                    AbbreviationPrefix::Apostrophe => "'",
                    AbbreviationPrefix::BackTick => "`",
                    AbbreviationPrefix::Comma => ",",
                    AbbreviationPrefix::CommaAt => ",@",
                }
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
