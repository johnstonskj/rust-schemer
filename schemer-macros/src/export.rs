#[macro_export]
macro_rules! export_standard_form {
    ($exports:expr, $id:expr => $fn_name:ident) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Form(standard_form!($id => $fn_name)),
        );
    };
   ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Form(standard_form!($id => $fn_name $( $arg )+)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident ; $var:expr) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Form(standard_form!($id => $fn_name ; $var)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+ ; $var:expr) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Form(standard_form!($id => $fn_name $( $arg )+ ; $var)),
        );
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! export_builtin {
    ($exports:expr, $id:expr => $fn_name:ident) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Procedure(builtin!($id => $fn_name)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Procedure(builtin!($id => $fn_name $( $arg )+)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident ; $var:expr) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Procedure(builtin!($id => $fn_name ; $var)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+ ; $var:expr) => {
        $exports.insert(
            id_from_str!($id),
            Expression::Procedure(builtin!($id => $fn_name $( $arg )+ ; $var)),
        );
    };
}

#[macro_export]
macro_rules! builtin {
    ($id:expr => $fn_name:ident) => {
        Procedure::new_builtin($id, vec![], None, &$fn_name)
    };
    ($id:expr => $fn_name:ident $( $arg:expr )+) => {
        Procedure::new_builtin($id, vec![$( $arg, )+], None, &$fn_name)
    };
    ($id:expr => $fn_name:ident ; $var:expr) => {
        Procedure::new_builtin($id, vec![], Some($var), &$fn_name)
    };
    ($id:expr => $fn_name:ident $( $arg:expr )+ ; $var:expr) => {
        Procedure::new_builtin($id, vec![$( $arg, )+], Some($var), &$fn_name)
    };
}

#[macro_export]
macro_rules! is_a {
    ($fn_name:ident, $expr_type:ident) => {
        pub fn $fn_name(
            arguments: Vec<Expression>,
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(eboolean!(matches!(
                &arguments[0],
                Expression::$expr_type(_)
            )))
        }
    };
    ($fn_name:ident, $expr_type:ident => $closure:expr) => {
        pub fn $fn_name(
            arguments: Vec<Expression>,
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(eboolean!(
                if let Expression::$expr_type(v) = &*arguments[0] {
                    ($closure)(v)
                } else {
                    false
                },
            ))
        }
    };
    ($fn_name:ident, $expr_type:ident !) => {
        pub fn $fn_name(
            arguments: Vec<Expression>,
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(eboolean!(matches!(&arguments[0], Expression::$expr_type)))
        }
    };
}

#[macro_export]
macro_rules! is_number_a {
    ($predicate:ident) => {
        is_number_a!($predicate, $predicate);
    };
    ($fn_name:ident, $predicate:ident) => {
        is_number_a!($fn_name => |v: &Number| v.$predicate());
    };
    ($fn_name:ident => $closure:expr) => {
        is_typed_a!($fn_name => $closure, Number, Number, TYPE_NAME_NUMBER);
    };
}

#[macro_export]
macro_rules! is_char_a {
    ($predicate:ident) => {
        is_char_a!($predicate, $predicate);
    };
    ($fn_name:ident, $predicate:ident) => {
        is_char_a!($fn_name => |v: &Char| v.$predicate());
    };
    ($fn_name:ident => $closure:expr) => {
        is_typed_a!($fn_name => $closure, Character, Char, TYPE_NAME_CHAR);
    };
}

#[macro_export]
macro_rules! is_list_a {
    ($predicate:ident) => {
        is_list_a!($predicate, $predicate);
    };
    ($fn_name:ident, $predicate:ident) => {
        is_list_a!($fn_name => |v: &Pair| v.$predicate());
    };
    ($fn_name:ident => $closure:expr) => {
        pub fn $fn_name(
            arguments: Vec<Expression>,
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(eboolean!(match &arguments[0] {
                Expression::Quotation(v) => {
                    if let Some(pair) = v.as_pair() {
                        $closure(pair)
                    } else {
                        unexpected_type!(=> TYPE_NAME_LIST, v)
                    }
                },
                e => {
                    unexpected_type!(=> TYPE_NAME_LIST, e)
                }
            }))
        }
    };
}

#[macro_export]
macro_rules! is_typed_a {
    ($predicate:ident, $expr_type:ident, $value_type:ty, $type_name:expr) => {
        is_typed_a!($predicate, $predicate, $expr_type, $value_type, $type_name);
    };
    ($fn_name:ident, $predicate:ident, $expr_type:ident, $value_type:ty, $type_name:expr) => {
        is_typed_a!($fn_name => |v: &$value_type| v.$predicate(), $expr_type, $type_name);
    };
    ($fn_name:ident => $closure:expr, $expr_type:ident, $value_type:ty, $type_name:expr) => {
        pub fn $fn_name(
            arguments: Vec<Expression>,
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(eboolean!(match &arguments[0] {
                Expression::$expr_type(v) => $closure(v),
                e => {
                    unexpected_type!(=> $type_name, e)
                }
            }))
        }
    };
}
