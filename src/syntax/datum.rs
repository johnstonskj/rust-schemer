/*!
One-line description.

More detailed description, with

# Example

*/

use crate::syntax::tokens::Identifier;
use crate::types::booleans::Boolean;
use crate::types::chars;
use crate::types::chars::Char;
use crate::types::numbers::Number;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Label = u128;

pub enum Datum {
    Simple(SimpleDatum),
    Compound(CompoundDatum),
    Labeled(Label, Box<Datum>),
    LabelRef(Label),
}

pub enum SimpleDatum {
    Boolean(Boolean),
    Number(Number),
    Character(Char),
    String(String),
    Symbol(Identifier),
    ByteVector(Vec<u8>),
}

pub enum CompoundDatum {
    List(Vec<Datum>),
    Vector(Vec<Datum>),
    Abbreviation(AbbreviationPrefix, Box<Datum>),
}

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

impl From<SimpleDatum> for Datum {
    fn from(v: SimpleDatum) -> Self {
        Self::Simple(v)
    }
}

impl From<CompoundDatum> for Datum {
    fn from(v: CompoundDatum) -> Self {
        Self::Compound(v)
    }
}

impl From<Label> for Datum {
    fn from(v: Label) -> Self {
        Self::LabelRef(v)
    }
}

impl Display for Datum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Datum::Simple(v) => v.to_string(),
                Datum::Compound(v) => v.to_string(),
                Datum::Labeled(l, v) => format!("#{}={}", l, v),
                Datum::LabelRef(l) => format!("#{}#", l),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Boolean> for SimpleDatum {
    fn from(v: Boolean) -> Self {
        Self::Boolean(v)
    }
}

impl From<bool> for SimpleDatum {
    fn from(v: bool) -> Self {
        Self::Boolean(Boolean::from(v))
    }
}

impl From<Number> for SimpleDatum {
    fn from(v: Number) -> Self {
        Self::Number(v)
    }
}

impl From<Char> for SimpleDatum {
    fn from(v: Char) -> Self {
        Self::Character(v)
    }
}

impl From<char> for SimpleDatum {
    fn from(v: char) -> Self {
        Self::Character(Char::from(v))
    }
}

impl From<String> for SimpleDatum {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for SimpleDatum {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<Identifier> for SimpleDatum {
    fn from(v: Identifier) -> Self {
        Self::Symbol(v)
    }
}

impl From<Vec<u8>> for SimpleDatum {
    fn from(v: Vec<u8>) -> Self {
        Self::ByteVector(v)
    }
}

impl From<&[u8]> for SimpleDatum {
    fn from(v: &[u8]) -> Self {
        Self::ByteVector(v.to_vec())
    }
}

impl Display for SimpleDatum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SimpleDatum::Boolean(v) => v.to_string(),
                SimpleDatum::Number(v) => v.to_string(),
                SimpleDatum::Character(v) => v.to_string(),
                SimpleDatum::String(v) => v.to_string(),
                SimpleDatum::Symbol(v) => v.to_string(),
                SimpleDatum::ByteVector(vs) => format!(
                    "#u8({})",
                    vs.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ),
            }
        )
    }
}

fn char_to_string(c: &char) -> String {
    if c.is_ascii() && !c.is_ascii_control() {
        format!("#\\{}", c)
    } else {
        chars::char_to_name(*c)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for CompoundDatum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CompoundDatum::List(vs) => format!(
                    "({})",
                    vs.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ),
                CompoundDatum::Vector(vs) => format!(
                    "#({})",
                    vs.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ),
                CompoundDatum::Abbreviation(p, v) => format!("{}{}", p, v),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for AbbreviationPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AbbreviationPrefix::Apostrophe => "'",
                AbbreviationPrefix::BackTick => "`",
                AbbreviationPrefix::Comma => ",",
                AbbreviationPrefix::CommaAt => ",@",
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
