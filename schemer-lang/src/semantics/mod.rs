/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::read::datum::Datum;
use crate::types::booleans::Boolean;
use crate::types::chars::Char;
use crate::types::lists::NULL_LIST_REPR_STRING;
use crate::types::numbers::Number;
use crate::types::strings::SchemeString;
use crate::types::symbols::Identifier;
use crate::types::SchemeRepr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Quotation(Box<Datum>),
    Boolean(Boolean),
    Number(Number),
    Vector(Vec<Box<Datum>>),
    Character(Char),
    String(SchemeString),
    ByteVector(Vec<u8>),
    ProcedureCall {
        operator: Box<Expression>,
        operands: Vec<Box<Expression>>,
    },
    Null,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn datum_to_expression(datum: Datum) -> Result<Expression, Error> {
    Ok(match datum {
        Datum::Boolean(v) => Expression::Boolean(v),
        Datum::Number(v) => Expression::Number(v),
        Datum::Character(v) => Expression::Character(v),
        Datum::String(v) => Expression::String(v),
        Datum::Symbol(v) => Expression::Identifier(v),
        Datum::ByteVector(v) => Expression::ByteVector(v),
        Datum::List(_) => {
            todo!()
        }
        Datum::Vector(v) => Expression::Vector(v),
        Datum::Abbreviation(_, _) => {
            todo!()
        }
        Datum::Labeled(_, _) => {
            todo!()
        }
        Datum::LabelRef(_) => {
            todo!()
        }
        Datum::Null => Expression::Null,
    })
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Expression {
    fn to_repr_string(&self) -> String {
        match self {
            Self::Identifier(v) => v.to_repr_string(),
            Self::Quotation(v) => {
                format!("'{}", v.to_repr_string())
            }
            Self::Boolean(v) => v.to_repr_string(),
            Self::Number(v) => v.to_repr_string(),
            Self::Character(v) => v.to_repr_string(),
            Self::String(v) => v.to_string(),
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
            Self::ProcedureCall { operator, operands } => {
                format!(
                    "({}{}{})",
                    operator.to_repr_string(),
                    if operands.is_empty() { "" } else { " " },
                    operands
                        .iter()
                        .map(|v| v.to_repr_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Self::Null => NULL_LIST_REPR_STRING.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod library;

pub mod program;
