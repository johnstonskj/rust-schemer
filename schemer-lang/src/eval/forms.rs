/*!
One-line description.

More detailed description, with

# Example

 */

use crate::error::{Error, ErrorKind};
use crate::eval::callable::Callable;
use crate::eval::environment::Exports;
use crate::eval::{eval_datum, Environment, Expression, Procedure};
use crate::read::datum::{datum_to_vec, Datum};
use crate::read::syntax_str::{
    FORM_NAME_IF, FORM_NAME_LAMBDA, FORM_NAME_LAMBDA_ALT, FORM_NAME_QUOTE,
};
use crate::types::lists::list_to_vec;
use crate::types::symbols::TYPE_NAME_SYMBOL;
use crate::types::{Identifier, Ref, SchemeRepr, SchemeValue};
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
    &'static dyn Fn(&[Ref<Datum>], &mut Ref<Environment>) -> Result<Expression, Error>;

pub const TYPE_NAME_FORM: &str = "standard-form";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

macro_rules! export_standard_form {
    ($exports:expr, $id:expr => $fn_name:ident) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Form(standard_form!($id => $fn_name)),
        );
    };
   ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Form(standard_form!($id => $fn_name $( $arg )+)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident ; $var:expr) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Form(standard_form!($id => $fn_name ; $var)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+ ; $var:expr) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Form(standard_form!($id => $fn_name $( $arg )+ ; $var)),
        );
    };
}

macro_rules! standard_form {
    ($id:expr => $fn_name:ident) => {
        Form::new($id, vec![], None, &$fn_name)
    };
    ($id:expr => $fn_name:ident $( $arg:expr )+) => {
        Form::new($id, vec![$( $arg, )+], None, &$fn_name)
    };
    ($id:expr => $fn_name:ident ; $var:expr) => {
        Form::new($id, vec![], Some($var), &$fn_name)
    };
    ($id:expr => $fn_name:ident $( $arg:expr )+ ; $var:expr) => {
        Form::new($id, vec![$( $arg, )+], Some($var), &$fn_name)
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn standard_form_exports() -> Exports {
    let mut exports = Exports::default();

    export_standard_form!(exports, FORM_NAME_LAMBDA => lambda "formals" ; "body");
    export_standard_form!(exports, FORM_NAME_LAMBDA_ALT => lambda "formals" ; "body");
    export_standard_form!(exports, FORM_NAME_IF => conditional "test" "consequence" ; "alternate");
    export_standard_form!(exports, FORM_NAME_QUOTE => quote "datum");

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
                &"fn(&[Ref<Datum>], &mut Ref<Environment>) -> Result<Expression, Error>",
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
                as *const dyn Fn(&[Ref<Datum>], &mut Ref<Environment>) -> Result<Expression, Error>
                == other.body
                    as *const dyn Fn(
                        &[Ref<Datum>],
                        &mut Ref<Environment>,
                    ) -> Result<Expression, Error>
    }
}

impl SchemeValue for Form {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_FORM
    }
}

impl Callable for Form {
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

    pub fn call(
        &self,
        arguments: &[Ref<Datum>],
        environment: &mut Ref<Environment>,
    ) -> Result<Expression, Error> {
        println!("{:?}", self);
        println!("{:?}", arguments);
        let argument_len = arguments.len();
        if argument_len < self.min_arg_count()
            || (!self.has_variadic_argument() && argument_len > self.max_arg_count().unwrap())
        {
            return Err(Error::from(ErrorKind::BadFormSyntax {
                name: self.id.to_repr_string(),
                value: "incorrect argument count".to_string(),
            }));
        }
        (self.body)(arguments, environment)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn quote(arguments: &[Ref<Datum>], _env: &mut Ref<Environment>) -> Result<Expression, Error> {
    if arguments.len() != 1 {
        Err(Error::from(ErrorKind::BadFormSyntax {
            name: FORM_NAME_QUOTE.to_string(),
            value: format!("{:?}", arguments),
        }))
    } else {
        Ok(Expression::Quotation(arguments[0].clone()))
    }
}

fn lambda(arguments: &[Ref<Datum>], _env: &mut Ref<Environment>) -> Result<Expression, Error> {
    let formals = arguments[0].clone();
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
                let mut vector = vector?;
                let variadic = vector.remove(vector.len() - 1);
                (vector, Some(variadic))
            }
        } else {
            return Err(Error::from(ErrorKind::BadFormSyntax {
                name: FORM_NAME_LAMBDA.to_string(),
                value: format!("{:?}", formals),
            }));
        };

    let bodies = datum_to_vec(arguments[1].clone());

    Ok(Expression::Procedure(Procedure::new_lambda(
        Identifier::from_str_unchecked("|<unknown>|"),
        formals,
        variadic,
        bodies,
    )))
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

fn conditional(
    arguments: &[Ref<Datum>],
    environment: &mut Ref<Environment>,
) -> Result<Expression, Error> {
    let test = arguments[0].clone();
    let result = eval_datum(test, environment)?;
    if result.is_true() {
        let consequent = arguments[1].clone();
        eval_datum(consequent, environment)
    } else if arguments.len() == 3 {
        let alternate = arguments[2].clone();
        eval_datum(alternate, environment)
    } else {
        Ok(Expression::Unspecified)
    }
}

/* derived expression */

pub fn unquote(_params: &[Ref<Datum>], _env: &mut Ref<Environment>) -> Result<Expression, Error> {
    todo!()
}

pub fn unquote_splicing(
    _params: &[Ref<Datum>],
    _env: &Ref<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

pub fn quasi_quote(
    _params: &[Ref<Datum>],
    _env: &mut Ref<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
