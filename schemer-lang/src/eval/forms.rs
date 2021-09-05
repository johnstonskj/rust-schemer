/*!
One-line description.

More detailed description, with

# Example

 */

// TODO: implement everything!
#![allow(dead_code)]

use crate::error::{Error, ErrorKind};
use crate::eval::callable::Callable;
use crate::eval::environment::Exports;
use crate::eval::{Environment, Evaluate, Expression, Procedure};
use crate::read::datum::{datum_to_vec, Datum};
use crate::read::syntax_str::{
    FORM_NAME_BEGIN, FORM_NAME_DEFINE, FORM_NAME_IF, FORM_NAME_LAMBDA, FORM_NAME_QUOTE,
    FORM_NAME_SET, PSEUDO_SYNTAX_COLON_CHAR, PSEUDO_SYNTAX_LEFT_PROCEDURE, PSEUDO_SYNTAX_RANGE,
    PSEUDO_SYNTAX_RIGHT_PROCEDURE,
};
use crate::types::lists::{list_to_vec, TYPE_NAME_LIST};
use crate::types::symbols::TYPE_NAME_SYMBOL;
use crate::types::{Identifier, MutableRef, Ref, SchemeRepr, SchemeValue};
use std::fmt::{Debug, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct Form {
    id: Identifier,
    formals: Vec<Identifier>,
    variadic_formal: Option<Identifier>,
    body: FormFn,
}

pub type FormFn =
    &'static dyn Fn(Vec<Ref<Datum>>, &mut MutableRef<Environment>) -> Result<Expression, Error>;

pub const TYPE_NAME_FORM: &str = "standard-form";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn standard_form_exports() -> Exports {
    let mut exports = Exports::default();

    export_standard_form!(exports, FORM_NAME_QUOTE => quote "datum");
    export_standard_form!(exports, FORM_NAME_LAMBDA => lambda "formals" ; "body");
    export_standard_form!(exports, FORM_NAME_IF => conditional "test" "consequence" ; "alternate");

    export_standard_form!(exports, FORM_NAME_SET => set_bang "variable" "expression");

    export_standard_form!(exports, FORM_NAME_BEGIN => begin ; "expression-or-definition");

    export_standard_form!(exports, FORM_NAME_DEFINE => define "variable-or-formals" "expression-or-body" ; "expression-or-body");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Debug for Form {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("id", &self.id)
            .field("formals", &self.formals)
            .field("variadic_formal", &self.variadic_formal)
            .field(
                "body",
                &"fn(&[Ref<Datum>], &mut MutableRef<Environment>) -> Result<Expression, Error>",
            )
            .finish()
    }
}

impl PartialEq for Form {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.formals == other.formals
            && self.variadic_formal == other.variadic_formal
            && self.body
                as *const dyn Fn(
                    Vec<Ref<Datum>>,
                    &mut MutableRef<Environment>,
                ) -> Result<Expression, Error>
                == other.body
                    as *const dyn Fn(
                        Vec<Ref<Datum>>,
                        &mut MutableRef<Environment>,
                    ) -> Result<Expression, Error>
    }
}

impl SchemeRepr for Form {
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

impl SchemeValue for Form {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_FORM
    }
}

impl Callable<Ref<Datum>> for Form {
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
        arguments: Vec<Ref<Datum>>,
        environment: &mut MutableRef<Environment>,
    ) -> Result<Expression, Error> {
        let argument_len = arguments.len();
        if argument_len < self.min_arg_count()
            || (!self.has_variadic_argument() && argument_len > self.max_arg_count().unwrap())
        {
            return Err(Error::from(ErrorKind::BadFormSyntax {
                name: self.id.clone(),
                value: "incorrect argument count".to_string(),
            }));
        }
        (self.body)(arguments, environment)
    }
}

impl Form {
    pub fn new(id: &str, formals: Vec<&str>, variadic_formal: Option<&str>, body: FormFn) -> Self {
        Self {
            id: Identifier::from_str_unchecked(id),
            formals: formals
                .iter()
                .map(|i| Identifier::from_str_unchecked(i))
                .collect(),
            variadic_formal: variadic_formal.map(|i| Identifier::from_str_unchecked(i)),
            body,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn head(arguments: &mut Vec<Ref<Datum>>) -> Ref<Datum> {
    arguments.remove(0)
}

fn datum_to_id(datum: Ref<Datum>) -> Result<Identifier, Error> {
    if let Datum::Symbol(symbol) = &*datum {
        Ok(symbol.clone())
    } else {
        Err(Error::from(ErrorKind::UnexpectedType {
            expected: TYPE_NAME_SYMBOL.to_string(),
            actual: Some(datum.type_name().to_string()),
        }))
    }
}

// §4.1.2. Literal expressions --------------------------------------------------------------------

pub fn quote(
    mut arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    if arguments.len() != 1 {
        Err(Error::from(ErrorKind::BadFormSyntax {
            name: Identifier::from_str_unchecked(FORM_NAME_QUOTE),
            value: format!("{:?}", arguments),
        }))
    } else {
        Ok(Expression::Quotation(head(&mut arguments)))
    }
}

// §4.1.4. Procedures -----------------------------------------------------------------------------

fn lambda(
    mut arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    let formals = head(&mut arguments);
    let (formals, variadic): (Vec<Identifier>, Option<Identifier>) =
        if let Datum::Symbol(symbol) = &*formals {
            (vec![], Some(symbol.clone()))
        } else if let Datum::List(list) = &*formals {
            let vector: Result<Vec<Identifier>, _> = list_to_vec(list.clone())
                .into_iter()
                .map(|d| datum_to_id(d))
                .collect();
            if list.is_proper_list() {
                (vector?, None)
            } else {
                let last_pair = list.last();
                (vector?, Some(datum_to_id(last_pair.cdr().clone())?))
            }
        } else {
            return Err(Error::from(ErrorKind::BadFormSyntax {
                name: Identifier::from_str_unchecked(FORM_NAME_LAMBDA),
                value: format!("{:?}", formals),
            }));
        };

    let bodies = datum_to_vec(head(&mut arguments));

    Ok(Expression::Procedure(Procedure::new_lambda(
        Identifier::from_str_unchecked("|<unknown>|"),
        formals,
        variadic,
        bodies,
    )))
}

// §4.1.5. Conditionals ---------------------------------------------------------------------------

fn conditional(
    mut arguments: Vec<Ref<Datum>>,
    environment: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    let test = head(&mut arguments);
    let result = test.eval(environment)?;
    if result.is_true() {
        let consequent = head(&mut arguments);
        consequent.eval(environment)
    } else if arguments.len() == 2 {
        let alternate = arguments.remove(1);
        alternate.eval(environment)
    } else {
        Ok(Expression::Unspecified)
    }
}

// §4.1.6. Assignments ----------------------------------------------------------------------------

fn set_bang(
    mut arguments: Vec<Ref<Datum>>,
    env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    let variable = head(&mut arguments);
    if let Datum::Symbol(id) = &*variable {
        let expr = head(&mut arguments).eval(env)?;
        env.borrow_mut().update(id.clone(), expr)?;
        Ok(Expression::Unspecified)
    } else {
        Err(Error::from(ErrorKind::UnexpectedType {
            expected: TYPE_NAME_SYMBOL.to_string(),
            actual: Some(variable.type_name().to_string()),
        }))
    }
}

// §4.1.7. Inclusion ------------------------------------------------------------------------------

fn include(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn include_ci(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2. Derived expression types -----------------------------------------------------------------

fn cond(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn case(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn and(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn or(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn when(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn unless(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn cond_expand(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// 4.2.2. Binding constructs ----------------------------------------------------------------------

fn bind_let(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_star(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_rec(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_rec_star(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_values_let(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_values_star(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// 4.2.3. Sequencing ------------------------------------------------------------------------------

fn begin(
    arguments: Vec<Ref<Datum>>,
    environment: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    arguments
        .into_iter()
        .fold(Ok(Expression::Unspecified), |_, datum| {
            datum.eval(environment)
        })
}

// 4.2.4. Iteration -------------------------------------------------------------------------------

fn iter_do(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn iter_let(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.5. Delayed evaluation ---------------------------------------------------------------------

// See library::scheme::lazy

// §4.2.6. Dynamic bindings -----------------------------------------------------------------------

fn make_parameter(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn parameterize(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.7. Exception handling ---------------------------------------------------------------------

fn guard(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.8. Quasiquotation -------------------------------------------------------------------------

pub fn quasi_quote(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

pub fn unquote(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

pub fn unquote_splicing(
    _arguments: Vec<Ref<Datum>>,
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.9. Case-lambda ----------------------------------------------------------------------------

fn case_lambda(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.3.1. Binding constructs for syntactic keywords ----------------------------------------------

fn let_syntax(
    _arguments: Vec<Ref<Datum>>,
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn let_rec_syntax(
    _arguments: Vec<Ref<Datum>>,
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.3.2. Pattern language -----------------------------------------------------------------------

fn syntax_rules(
    _arguments: Vec<Ref<Datum>>,
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.3.3. Signaling errors in macro transformers -------------------------------------------------

fn syntax_error(
    _arguments: Vec<Ref<Datum>>,
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §5.2. Import declarations ----------------------------------------------------------------------

// see schemer_library::forms::import

// §5.3. Variable definitions ---------------------------------------------------------------------

fn define(
    mut arguments: Vec<Ref<Datum>>,
    env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    let variable_or_formals = head(&mut arguments);

    if let Datum::List(list) = &*variable_or_formals {
        // defining a function
        let formals: Result<Vec<Identifier>, Error> = list_to_vec(list.clone())
            .into_iter()
            .map(|d| datum_to_id(d))
            .collect();
        let mut formals = formals?;

        if formals.is_empty() {
            Err(Error::from(ErrorKind::BadFormSyntax {
                name: Identifier::from_str_unchecked(FORM_NAME_DEFINE),
                value: variable_or_formals.to_repr_string(),
            }))
        } else {
            let id = formals.remove(0);

            let variadic = if list.is_proper_list() {
                None
            } else {
                let last_pair = list.last();
                Some(datum_to_id(last_pair.cdr().clone())?)
            };

            let bodies = arguments;

            let value =
                Expression::Procedure(Procedure::new_lambda(id.clone(), formals, variadic, bodies));
            let _ = env.borrow_mut().insert(id, value);
            Ok(Expression::Unspecified)
        }
    } else if let Datum::Symbol(id) = &*variable_or_formals {
        // defining a value
        let value = head(&mut arguments);
        let value = value.eval(env)?;
        let _ = env.borrow_mut().insert(id.clone(), value);
        Ok(Expression::Unspecified)
    } else {
        Err(Error::from(ErrorKind::UnexpectedType {
            expected: format!("(or {} {})", TYPE_NAME_SYMBOL, TYPE_NAME_LIST),
            actual: Some(variable_or_formals.type_name().to_string()),
        }))
    }
}

// §5.3.3. Multiple-value definitions -------------------------------------------------------------

fn define_values(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §5.4. Syntax definitions -----------------------------------------------------------------------

fn define_syntax(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §5.6.1. Library Syntax -------------------------------------------------------------------------

fn define_library(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
