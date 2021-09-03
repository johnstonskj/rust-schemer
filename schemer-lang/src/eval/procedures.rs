/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::callable::Callable;
use crate::eval::{eval_datum, Environment, Expression};
use crate::read::datum::Datum;

use crate::read::syntax_str::{
    PSEUDO_SYNTAX_COLON_CHAR, PSEUDO_SYNTAX_LEFT_PROCEDURE, PSEUDO_SYNTAX_RANGE,
    PSEUDO_SYNTAX_RIGHT_PROCEDURE,
};
use crate::types::{Identifier, MutableRef, Ref, SchemeRepr, SchemeValue};
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
    &'static dyn Fn(Vec<Expression>, &mut MutableRef<Environment>) -> Result<Expression, Error>;

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
                        "fn(Vec<Expression>, &MutableRef<Environment>) -> Result<Expression, Error>>",
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
            (Self::Builtin(lhs), Self::Builtin(rhs)) => {
                lhs as *const dyn Fn(
                    Vec<Expression>,
                    &mut MutableRef<Environment>,
                ) -> Result<Expression, Error>
                    == rhs as *const dyn Fn(
                        Vec<Expression>,
                        &mut MutableRef<Environment>,
                    ) -> Result<Expression, Error>
            }
            (Self::Lambda(lhs), Self::Lambda(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Procedure {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            PSEUDO_SYNTAX_LEFT_PROCEDURE,
            self.type_name(),
            PSEUDO_SYNTAX_COLON_CHAR,
            self.id().to_repr_string(),
            PSEUDO_SYNTAX_COLON_CHAR,
            self.min_arg_count(),
            PSEUDO_SYNTAX_RANGE,
            self.max_arg_count()
                .map(|c| c.to_string())
                .unwrap_or("*".to_string()),
            PSEUDO_SYNTAX_RIGHT_PROCEDURE
        )
    }
}

impl SchemeValue for Procedure {
    fn type_name(&self) -> &'static str {
        match &self.body {
            ProcedureBody::Builtin(_) => TYPE_NAME_BUILTIN_PROCEDURE,
            ProcedureBody::Lambda(_) => TYPE_NAME_PROCEDURE,
        }
    }
}

impl Callable<Expression> for Procedure {
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

    fn call(
        &self,
        mut arguments: Vec<Expression>,
        environment: &mut MutableRef<Environment>,
    ) -> Result<Expression, Error> {
        let argument_len = arguments.len();
        if argument_len < self.min_arg_count()
            || (!self.has_variadic_argument() && argument_len > self.max_arg_count().unwrap())
        {
            return Err(Error::from(ErrorKind::ProcedureArgumentCardinality {
                name: self.id.clone(),
                min: self.min_arg_count(),
                max: self.max_arg_count(),
                given: argument_len,
            }));
        }
        match &self.body {
            ProcedureBody::Builtin(body) => (body)(arguments, environment),
            ProcedureBody::Lambda(body) => {
                let mut environment =
                    Environment::new_child_named(environment.clone(), self.id().as_str());

                for i in 0..self.min_arg_count() {
                    let argument = arguments.remove(0);
                    environment
                        .borrow_mut()
                        .insert(self.formals.get(i).unwrap().clone(), argument)?;
                }

                if self.has_variadic_argument() {
                    environment.borrow_mut().insert(
                        self.variadic_formal.as_ref().unwrap().clone(),
                        if !arguments.is_empty() {
                            Expression::List(arguments)
                        } else {
                            Expression::Null
                        },
                    )?;
                }

                body.iter().fold(Ok(Expression::Unspecified), |_, datum| {
                    eval_datum(datum.clone(), &mut environment)
                })
            }
        }
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

    pub fn is_builtin(&self) -> bool {
        matches!(self.body, ProcedureBody::Builtin(_))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
