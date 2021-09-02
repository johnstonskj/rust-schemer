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
    FORM_NAME_IF, FORM_NAME_LAMBDA, FORM_NAME_LAMBDA_ALT, FORM_NAME_QUOTE, FORM_NAME_SET,
};
use crate::types::lists::list_to_vec;
use crate::types::symbols::TYPE_NAME_SYMBOL;
use crate::types::{Identifier, MutableRef, Ref, SchemeValue};
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
    &'static dyn Fn(&[Ref<Datum>], &mut MutableRef<Environment>) -> Result<Expression, Error>;

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

    export_standard_form!(exports, FORM_NAME_QUOTE => quote "datum");
    export_standard_form!(exports, FORM_NAME_LAMBDA => lambda "formals" ; "body");
    export_standard_form!(exports, FORM_NAME_LAMBDA_ALT => lambda "formals" ; "body");
    export_standard_form!(exports, FORM_NAME_IF => conditional "test" "consequence" ; "alternate");

    export_standard_form!(exports, FORM_NAME_SET => set_bang "variable" "expression");

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
                    &[Ref<Datum>],
                    &mut MutableRef<Environment>,
                ) -> Result<Expression, Error>
                == other.body
                    as *const dyn Fn(
                        &[Ref<Datum>],
                        &mut MutableRef<Environment>,
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// §4.1.2. Literal expressions --------------------------------------------------------------------

pub fn quote(
    arguments: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    if arguments.len() != 1 {
        Err(Error::from(ErrorKind::BadFormSyntax {
            name: Identifier::from_str_unchecked(FORM_NAME_QUOTE),
            value: format!("{:?}", arguments),
        }))
    } else {
        Ok(Expression::Quotation(arguments[0].clone()))
    }
}

// §4.1.4. Procedures -----------------------------------------------------------------------------

fn lambda(
    arguments: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    let formals = arguments[0].clone();
    println!(">>formals>> {:?}", formals);
    let (formals, variadic): (Vec<Identifier>, Option<Identifier>) =
        if let Datum::Symbol(symbol) = &*formals {
            (vec![], Some(symbol.clone()))
        } else if let Datum::List(list) = &*formals {
            let vector: Result<Vec<Identifier>, _> = list_to_vec(list.clone())
                .into_iter()
                .map(|d| datum_to_id(d))
                .collect();
            println!(">>vec>> {:?}", vector);
            if list.is_proper_list() {
                (vector?, None)
            } else {
                println!(">>list?>> {:?}", list);
                let last_pair = list.last();

                (vector?, Some(datum_to_id(last_pair.cdr().clone())?))
            }
        } else {
            return Err(Error::from(ErrorKind::BadFormSyntax {
                name: Identifier::from_str_unchecked(FORM_NAME_LAMBDA),
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

// §4.1.5. Conditionals ---------------------------------------------------------------------------

fn conditional(
    arguments: &[Ref<Datum>],
    environment: &mut MutableRef<Environment>,
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

// §4.1.6. Assignments ----------------------------------------------------------------------------

fn set_bang(params: &[Ref<Datum>], env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    let variable = params[0].clone();
    if let Datum::Symbol(id) = &*variable {
        let expr = eval_datum(params[1].clone(), env)?;
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
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn include_ci(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2. Derived expression types -----------------------------------------------------------------

fn cond(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn case(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn and(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn or(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn when(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn unless(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn cond_expand(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// 4.2.2. Binding constructs ----------------------------------------------------------------------

fn bind_let(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_star(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_rec(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_recstar(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_values_let(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn bind_let_values_star(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// 4.2.3. Sequencing ------------------------------------------------------------------------------

fn begin(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// 4.2.4. Iteration -------------------------------------------------------------------------------

fn iter_do(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn iter_let(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.5. Delayed evaluation ---------------------------------------------------------------------

fn delay(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn delay_force(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn force(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn is_promise(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn make_promise(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.6. Dynamic bindings -----------------------------------------------------------------------

fn make_parameter(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn parameterize(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.7. Exception handling ---------------------------------------------------------------------

fn guard(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// §4.2.8. Quasiquotation -------------------------------------------------------------------------

pub fn quasi_quote(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

pub fn unquote(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

pub fn unquote_splicing(
    _params: &[Ref<Datum>],
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.2.9. Case-lambda ----------------------------------------------------------------------------

fn case_lambda(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.3.1. Binding constructs for syntactic keywords ----------------------------------------------

fn let_syntax(_params: &[Ref<Datum>], _env: &MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn let_rec_syntax(
    _params: &[Ref<Datum>],
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.3.2. Pattern language -----------------------------------------------------------------------

fn syntax_rules(
    _params: &[Ref<Datum>],
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §4.3.3. Signaling errors in macro transformers -------------------------------------------------

fn syntax_error(
    _params: &[Ref<Datum>],
    _env: &MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §5.2. Import declarations ----------------------------------------------------------------------

fn import(_params: &[Ref<Datum>], _env: &MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// §5.3. Variable definitions ---------------------------------------------------------------------

fn define(_params: &[Ref<Datum>], _env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// §5.3.3. Multiple-value definitions -------------------------------------------------------------

fn define_values(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §5.4. Syntax definitions -----------------------------------------------------------------------

fn define_syntax(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// §5.6.1. Library Syntax -------------------------------------------------------------------------

fn define_library(
    _params: &[Ref<Datum>],
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
