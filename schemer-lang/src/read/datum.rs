/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::parameters::{get_global_flag, WRITE_QUOTE_LONG_FORM};
use crate::read::syntax_str::{
    FORM_NAME_QUASI_QUOTE, FORM_NAME_QUOTE, FORM_NAME_UNQUOTE, FORM_NAME_UNQUOTE_SPLICING,
    SYNTAX_ABBR_QUASI_QUOTE, SYNTAX_ABBR_QUOTE, SYNTAX_ABBR_UNQUOTE, SYNTAX_ABBR_UNQUOTE_SPLICING,
    SYNTAX_HASH_CHAR, SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_MATH_EQUALITY_CHAR,
    SYNTAX_RIGHT_PARENTHESIS_CHAR, VALUE_NULL_LIST,
};
use crate::types::lists::{list_to_vec, TYPE_NAME_LIST};
use crate::types::strings::ByteVector;
use crate::types::symbols::TYPE_NAME_SYMBOL;
use crate::types::{
    Boolean, Char, Identifier, Integer, Number, Pair, SchemeRepr, SchemeString, Vector,
};
use crate::types::{Ref, SchemeValue};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Label = u128;

#[derive(Clone, Debug, PartialEq)]
pub enum Datum {
    /* Simple */
    Boolean(Boolean),
    Number(Number),
    Character(Char),
    String(SchemeString),
    Symbol(Identifier),
    ByteVector(ByteVector),
    /* Compound */
    List(Pair),
    Vector(Vector<Datum>),
    /* Quotation */
    Abbreviation(Abbreviation, Ref<Datum>),
    /* Other */
    Labeled(Label, Ref<Datum>),
    LabelRef(Label),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Abbreviation {
    Quote,
    QuasiQuote,
    Unquote,
    UnquoteSplicing,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn datum_to_vec(datum: Ref<Datum>) -> Vec<Ref<Datum>> {
    match &*datum {
        Datum::Boolean(_)
        | Datum::Number(_)
        | Datum::Character(_)
        | Datum::String(_)
        | Datum::Symbol(_)
        | Datum::Vector(_)
        | Datum::ByteVector(_)
        | Datum::Abbreviation(_, _)
        | Datum::Null => vec![datum],
        Datum::List(pair) => list_to_vec(pair.clone()),
        Datum::Labeled(_, _) | Datum::LabelRef(_) => unreachable!(),
    }
}

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

impl From<Vector<Datum>> for Datum {
    fn from(v: Vector<Datum>) -> Self {
        Self::Vector(v)
    }
}

impl From<Vec<Datum>> for Datum {
    fn from(v: Vec<Datum>) -> Self {
        Self::Vector(Vector::from(v))
    }
}

impl From<&[Datum]> for Datum {
    fn from(v: &[Datum]) -> Self {
        Self::Vector(Vector::from(v))
    }
}

impl From<ByteVector> for Datum {
    fn from(v: ByteVector) -> Self {
        Self::ByteVector(v)
    }
}

impl From<Vec<u8>> for Datum {
    fn from(v: Vec<u8>) -> Self {
        Self::ByteVector(ByteVector::from(v))
    }
}

impl From<&[u8]> for Datum {
    fn from(v: &[u8]) -> Self {
        Self::ByteVector(ByteVector::from(v.to_vec()))
    }
}

impl From<Label> for Datum {
    fn from(v: Label) -> Self {
        Self::LabelRef(v)
    }
}

impl From<Pair> for Datum {
    fn from(v: Pair) -> Self {
        Self::List(v)
    }
}

impl SchemeValue for Datum {
    fn type_name(&self) -> &'static str {
        match self {
            Datum::Boolean(v) => v.type_name(),
            Datum::Number(v) => v.type_name(),
            Datum::Character(v) => v.type_name(),
            Datum::String(v) => v.type_name(),
            Datum::Symbol(v) => v.type_name(),
            Datum::ByteVector(v) => v.type_name(),
            Datum::List(v) => v.type_name(),
            Datum::Vector(v) => v.type_name(),
            Datum::Abbreviation(_, v) => v.type_name(),
            Datum::Labeled(_, v) => v.type_name(),
            Datum::LabelRef(_) => {
                unreachable!()
            }
            Datum::Null => TYPE_NAME_LIST,
        }
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
            Self::ByteVector(vs) => vs.to_repr_string(),
            Self::Vector(vs) => vs.to_repr_string(),
            Self::Abbreviation(p, v) => {
                if get_global_flag(WRITE_QUOTE_LONG_FORM).unwrap_or_default() {
                    format!(
                        "{}{}{}{}",
                        SYNTAX_LEFT_PARENTHESIS_CHAR,
                        p,
                        v.to_repr_string(),
                        SYNTAX_RIGHT_PARENTHESIS_CHAR
                    )
                } else {
                    format!("{}{}", p, v.to_repr_string())
                }
            }
            Self::Labeled(l, v) => format!(
                "{}{}{}{}",
                SYNTAX_HASH_CHAR,
                l,
                SYNTAX_MATH_EQUALITY_CHAR,
                v.to_repr_string()
            ),
            Self::LabelRef(l) => format!("{}{}{}", SYNTAX_HASH_CHAR, l, SYNTAX_HASH_CHAR),
            Datum::Null => VALUE_NULL_LIST.to_string(),
        }
    }
}

impl Datum {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Datum::Boolean(_))
    }

    pub fn as_boolean(&self) -> Option<&Boolean> {
        match self {
            Self::Boolean(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Datum::Number(_))
    }

    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Self::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_character(&self) -> bool {
        matches!(self, Datum::Character(_))
    }

    pub fn as_character(&self) -> Option<&Char> {
        match self {
            Self::Character(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Datum::String(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Datum::Symbol(_))
    }

    pub fn as_symbol(&self) -> Option<&Identifier> {
        match self {
            Self::Symbol(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Datum::List(_))
    }

    pub fn as_list(&self) -> Option<&Pair> {
        match self {
            Self::List(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_list_or_null(&self) -> bool {
        matches!(self, Datum::List(_) | Datum::Null)
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Datum::Vector(_))
    }

    pub fn as_vector(&self) -> Option<&Vector<Datum>> {
        match self {
            Self::Vector(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_byte_vector(&self) -> bool {
        matches!(self, Datum::ByteVector(_))
    }

    pub fn as_byte_vector(&self) -> Option<&ByteVector> {
        match self {
            Self::ByteVector(v) => Some(v),
            _ => None,
        }
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

    pub fn inner_type_name(&self) -> String {
        match self {
            Self::Boolean(v) => v.type_name().to_string(),
            Self::Number(v) => v.type_name().to_string(),
            Self::Character(v) => v.type_name().to_string(),
            Self::String(v) => v.type_name().to_string(),
            Self::Symbol(_) => TYPE_NAME_SYMBOL.to_string(),
            Self::List(v) => v.type_name().to_string(),
            Self::ByteVector(vs) => vs.type_name().to_string(),
            Self::Vector(vs) => vs.type_name().to_string(),
            Self::Abbreviation(_, _) => TYPE_NAME_SYMBOL.to_string(),
            Self::Labeled(_, v) => v.inner_type_name(),
            Self::LabelRef(_) => unreachable!(),
            Datum::Null => TYPE_NAME_LIST.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Abbreviation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() || get_global_flag(WRITE_QUOTE_LONG_FORM).unwrap_or_default() {
                match self {
                    Abbreviation::Quote => FORM_NAME_QUOTE,
                    Abbreviation::QuasiQuote => FORM_NAME_QUASI_QUOTE,
                    Abbreviation::Unquote => FORM_NAME_UNQUOTE,
                    Abbreviation::UnquoteSplicing => FORM_NAME_UNQUOTE_SPLICING,
                }
            } else {
                match self {
                    Abbreviation::Quote => SYNTAX_ABBR_QUOTE,
                    Abbreviation::QuasiQuote => SYNTAX_ABBR_QUASI_QUOTE,
                    Abbreviation::Unquote => SYNTAX_ABBR_UNQUOTE,
                    Abbreviation::UnquoteSplicing => SYNTAX_ABBR_UNQUOTE_SPLICING,
                }
            }
        )
    }
}

impl FromStr for Abbreviation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SYNTAX_ABBR_QUASI_QUOTE => Ok(Abbreviation::QuasiQuote),
            SYNTAX_ABBR_QUOTE => Ok(Abbreviation::Quote),
            SYNTAX_ABBR_UNQUOTE => Ok(Abbreviation::Unquote),
            SYNTAX_ABBR_UNQUOTE_SPLICING => Ok(Abbreviation::UnquoteSplicing),
            _ => Err(Error::from(ErrorKind::ParseValue {
                kind: "abbreviation".to_string(),
                value: s.to_string(),
            })),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
