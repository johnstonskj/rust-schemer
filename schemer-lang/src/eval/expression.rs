/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::callable::Callable;
use crate::eval::environment::Environment;
use crate::eval::procedures::Procedure;
use crate::eval::{forms, Form};
use crate::read::datum::{datum_to_vec, Abbreviation, Datum};
use crate::read::syntax_str::{
    SYNTAX_ABBR_QUOTE, SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR, VALUE_NULL_LIST,
};
use crate::types::lists::TYPE_NAME_LIST;
use crate::types::symbols::TYPE_NAME_SYMBOL;
use crate::types::{
    Boolean, ByteVector, Char, Identifier, MutableRef, Number, Pair, Ref, SchemeRepr, SchemeString,
    SchemeValue, Vector,
};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Evaluate: SchemeValue {
    fn eval(&self, environment: &mut MutableRef<Environment>) -> Result<Expression, Error>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    /* literal */
    Boolean(Boolean),
    Number(Number),
    Vector(Vector<Datum>),
    Character(Char),
    String(SchemeString),
    ByteVector(ByteVector),
    Quotation(Ref<Datum>),
    Form(Form),
    Procedure(Procedure),
    List(Vec<Expression>),
    Environment(MutableRef<Environment>),
    Null,
    Unspecified,
}

pub const VALUE_NAME_UNSPECIFIED: &str = "#!unspecified";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Evaluate for Datum {
    fn eval(&self, environment: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        Ok(match self {
            Datum::Symbol(v) => v.eval(environment)?,
            Datum::Boolean(v) => v.eval(environment)?,
            Datum::Number(v) => v.eval(environment)?,
            Datum::Character(v) => v.eval(environment)?,
            Datum::String(v) => v.eval(environment)?,
            Datum::ByteVector(v) => v.eval(environment)?,
            Datum::Vector(v) => v.eval(environment)?,
            Datum::List(v) => call_or_form_from_list(&v, environment)?,
            Datum::Abbreviation(a, d) => match a {
                Abbreviation::Quote => forms::quote(vec![d.clone()], environment)?,
                Abbreviation::QuasiQuote => forms::quasi_quote(vec![d.clone()], environment)?,
                Abbreviation::Unquote => forms::unquote(vec![d.clone()], environment)?,
                Abbreviation::UnquoteSplicing => {
                    forms::unquote_splicing(vec![d.clone()], environment)?
                }
            },
            Datum::Labeled(_, _) => {
                unreachable!()
            }
            Datum::LabelRef(_) => {
                unreachable!()
            }
            Datum::Null => Expression::Null,
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Expression {
    fn to_repr_string(&self) -> String {
        match self {
            Self::Identifier(v) => v.to_repr_string(),
            Self::Boolean(v) => v.to_repr_string(),
            Self::Number(v) => v.to_repr_string(),
            Self::Vector(v) => v.to_repr_string(),
            Self::Character(v) => v.to_repr_string(),
            Self::String(v) => v.to_repr_string(),
            Self::ByteVector(v) => v.to_repr_string(),
            Self::Quotation(v) => format!("{}{}", SYNTAX_ABBR_QUOTE, v.to_repr_string()),
            Self::Procedure(v) => v.to_repr_string(),
            Self::List(v) => format!(
                "{}{}{}",
                SYNTAX_LEFT_PARENTHESIS_CHAR,
                v.iter()
                    .map(|e| e.to_repr_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                SYNTAX_RIGHT_PARENTHESIS_CHAR
            ),
            Self::Null => VALUE_NULL_LIST.to_string(),
            Self::Unspecified => VALUE_NAME_UNSPECIFIED.to_string(),
            Self::Environment(v) => v.borrow().to_repr_string(),
            Self::Form(v) => v.to_repr_string(),
        }
    }
}

impl SchemeValue for Expression {
    fn type_name(&self) -> &'static str {
        match self {
            Self::Identifier(v) => v.type_name(),
            Self::Boolean(v) => v.type_name(),
            Self::Number(v) => v.type_name(),
            Self::Vector(v) => v.type_name(),
            Self::Character(v) => v.type_name(),
            Self::String(v) => v.type_name(),
            Self::ByteVector(v) => v.type_name(),
            Self::Quotation(v) => v.type_name(),
            Self::Form(v) => v.type_name(),
            Self::Procedure(v) => v.type_name(),
            Self::List(_) => TYPE_NAME_LIST,
            Self::Null => TYPE_NAME_LIST,
            Self::Unspecified => VALUE_NAME_UNSPECIFIED,
            Self::Environment(v) => v.borrow().type_name(),
        }
    }
}

impl Evaluate for Expression {
    fn eval(&self, environment: &mut MutableRef<Environment>) -> Result<Expression, Error> {
        match self {
            Expression::Quotation(v) => v.as_ref().eval(environment),
            _ => Ok(self.clone()),
        }
    }
}

impl Expression {
    pub fn is_false(&self) -> bool {
        if let Self::Boolean(v) = self {
            v.is_false()
        } else {
            false
        }
    }

    pub fn is_true(&self) -> bool {
        !self.is_false()
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(_))
    }

    pub fn is_character(&self) -> bool {
        matches!(self, Self::Character(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_byte_vector(&self) -> bool {
        matches!(self, Self::ByteVector(_))
    }

    pub fn is_quotation(&self) -> bool {
        matches!(self, Self::Quotation(_))
    }

    pub fn is_procedure(&self) -> bool {
        matches!(self, Self::Procedure(_))
    }

    pub fn is_builtin_procedure(&self) -> bool {
        match self {
            Self::Procedure(p) => p.is_builtin(),
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, Self::Unspecified)
    }

    pub fn is_environment(&self) -> bool {
        matches!(self, Self::Environment(_))
    }

    pub fn is_form(&self) -> bool {
        matches!(self, Self::Form(_))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn call_or_form_from_list(
    from: &Pair,
    environment: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    if from.is_proper_list() {
        if let Datum::Symbol(id) = &**from.car() {
            let variable = environment.borrow().get(&id);
            if let Some(Expression::Form(form)) = variable {
                let arguments = if from.cdr().is_null() {
                    Vec::default()
                } else {
                    datum_to_vec(from.cdr().clone())
                };
                form.call(arguments, environment)
            } else if let Some(Expression::Procedure(procedure)) = variable {
                let procedure = procedure.clone();
                procedure.call(make_parameters(from.cdr(), environment)?, environment)
            } else if let Some(expr) = variable {
                Error::from(ErrorKind::UnexpectedType {
                    expected: TYPE_NAME_SYMBOL.to_string(),
                    actual: Some(expr.type_name().to_string()),
                })
                .into()
            } else {
                Error::from(ErrorKind::UnboundVariable { name: id.clone() }).into()
            }
        } else {
            Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_SYMBOL.to_string(),
                actual: Some(from.car().inner_type_name()),
            })
            .into()
        }
    } else {
        Error::from(ErrorKind::ImproperList).into()
    }
}

fn make_parameters(
    from: &Datum,
    environment: &mut MutableRef<Environment>,
) -> Result<Vec<Expression>, Error> {
    if from.is_null() {
        Ok(Default::default())
    } else {
        // TODO: unwrap list, no clones
        let list = from.as_pair().unwrap();
        list.iter().map(|d| d.eval(environment)).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
