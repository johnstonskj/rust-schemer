/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::callable::Callable;
use crate::eval::{eval_datum, Environment, Expression};
use crate::read::datum::Datum;

use crate::types::{Identifier, Ref, SchemeRepr, SchemeValue};
use std::fmt::{Debug, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Procedure {
    id: Identifier,
    formals: Vec<Identifier>,
    variadic_formal: Option<Identifier>,
    body: ProcedureBody,
}

#[derive(Clone)]
pub enum ProcedureBody {
    Builtin(BuiltinFn),
    Lambda(Vec<Ref<Datum>>),
}

pub type BuiltinFn =
    &'static dyn Fn(&[Expression], &mut Ref<Environment>) -> Result<Expression, Error>;

pub const TYPE_NAME_BUILTIN_PROCEDURE: &str = "builtin-procedure";

pub const TYPE_NAME_PROCEDURE: &str = "procedure";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Debug for ProcedureBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcedureBody")
            .field(
                "body",
                &match self {
                    ProcedureBody::Builtin(_) => String::from(
                        "fn(&[Expression], &Environment) -> Result<Expression, Error>>",
                    ),
                    ProcedureBody::Lambda(v) => format!("{:?}", v),
                },
            )
            .finish()
    }
}

impl PartialEq for ProcedureBody {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // TODO: can we do better?
            (Self::Builtin(lhs), Self::Builtin(rhs)) => {
                lhs as *const dyn Fn(
                    &[Expression],
                    &mut Ref<Environment>,
                ) -> Result<Expression, Error>
                    == rhs as *const dyn Fn(
                        &[Expression],
                        &mut Ref<Environment>,
                    ) -> Result<Expression, Error>
            }
            (Self::Lambda(lhs), Self::Lambda(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl SchemeValue for Procedure {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_PROCEDURE
    }
}

impl Callable for Procedure {
    fn id(&self) -> &Identifier {
        &self.id
    }

    fn rename(&mut self, id: Identifier) {
        self.id = id;
    }

    fn formal_arguments(&self) -> &Vec<Identifier> {
        &self.formals
    }

    fn variadic_formal_argument(&self) -> &Option<Identifier> {
        &self.variadic_formal
    }
}

impl Procedure {
    pub fn new_lambda(
        id: Identifier,
        formals: Vec<Identifier>,
        variadic_formal: Option<Identifier>,
        body: Vec<Ref<Datum>>,
    ) -> Self {
        Self {
            id,
            formals,
            variadic_formal,
            body: ProcedureBody::Lambda(body),
        }
    }

    pub fn new_builtin(
        id: &str,
        formals: Vec<&str>,
        variadic_formal: Option<&str>,
        body: BuiltinFn,
    ) -> Self {
        Self {
            id: Identifier::from_str_unchecked(id),
            formals: formals
                .iter()
                .map(|i| Identifier::from_str_unchecked(i))
                .collect(),
            variadic_formal: variadic_formal.map(|i| Identifier::from_str_unchecked(i)),
            body: ProcedureBody::Builtin(body),
        }
    }

    pub fn call(
        &self,
        arguments: &[Expression],
        environment: &mut Ref<Environment>,
    ) -> Result<Expression, Error> {
        let argument_len = arguments.len();
        if argument_len < self.min_arg_count()
            || (!self.has_variadic_argument() && argument_len > self.max_arg_count().unwrap())
        {
            return Err(Error::from(ErrorKind::ProcedureArgumentCardinality {
                name: self.id.to_repr_string(),
                min: self.min_arg_count(),
                max: self.max_arg_count(),
                given: argument_len,
            }));
        }
        match &self.body {
            ProcedureBody::Builtin(body) => (body)(arguments, environment),
            ProcedureBody::Lambda(body) => {
                let mut environment = Environment::new_child_named(environment, self.id().as_str());

                for (i, argument) in arguments.iter().enumerate() {
                    environment.insert(self.formals.get(i).unwrap().clone(), argument.clone())?;
                }

                let mut environment = Ref::new(environment);
                body.iter().fold(Ok(Expression::Unspecified), |_, datum| {
                    eval_datum(datum.clone(), &mut environment)
                })
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
