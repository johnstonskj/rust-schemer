/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::Number;
use std::convert::TryFrom;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum DatumType {
    Null = 0x00,
    Boolean = 0x01,
    Character = 0x02,
    String = 0x03,
    ByteVector = 0x04,
    Integer = 0x11,
    Rational = 0x12,
    ExactReal = 0x13,
    InexactReal = 0x14,
    ExactComplex = 0x15,
    InexactComplex = 0x16,
    List = 0x21,
    Vector = 0x22,
    Identifier = 0xA1,
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

impl From<Datum> for DatumType {
    fn from(v: Datum) -> Self {
        match v {
            Datum::Null => Self::Null,
            Datum::Boolean(_) => Self::Boolean,
            Datum::Number(n) => match n {
                Number::InexactComplex(_) => Self::InexactComplex,
                Number::ExactComplex(_) => Self::ExactComplex,
                Number::InexactReal(_) => Self::InexactReal,
                Number::ExactReal(_) => Self::ExactReal,
                Number::Rational(_) => Self::Rational,
                Number::Integer(_) => Self::Integer,
            },
            Datum::Character(_) => Self::Character,
            Datum::String(_) => Self::String,
            Datum::ByteVector(_) => Self::ByteVector,
            Datum::List(_) => Self::List,
            Datum::Vector(_) => Self::Vector,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for DatumType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Null),
            0x01 => Ok(Self::Boolean),
            0x02 => Ok(Self::Character),
            0x03 => Ok(Self::String),
            0x04 => Ok(Self::ByteVector),
            0x11 => Ok(Self::Integer),
            0x12 => Ok(Self::Rational),
            0x13 => Ok(Self::ExactReal),
            0x14 => Ok(Self::InexactReal),
            0x15 => Ok(Self::ExactComplex),
            0x16 => Ok(Self::InexactComplex),
            0x21 => Ok(Self::List),
            0x22 => Ok(Self::Vector),
            0xA1 => Ok(Self::Identifier),
            _ => Err(ErrorKind::Format.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
