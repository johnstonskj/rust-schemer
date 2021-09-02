/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

macro_rules! export_builtin {
    ($exports:expr, $id:expr => $fn_name:ident) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Procedure(builtin!($id => $fn_name)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Procedure(builtin!($id => $fn_name $( $arg )+)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident ; $var:expr) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Procedure(builtin!($id => $fn_name ; $var)),
        );
    };
    ($exports:expr, $id:expr => $fn_name:ident $( $arg:expr )+ ; $var:expr) => {
        $exports.insert(
            Identifier::from_str_unchecked($id),
            Expression::Procedure(builtin!($id => $fn_name $( $arg )+ ; $var)),
        );
    };
}

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

macro_rules! is_a {
    ($fn_name:ident, $expr_type:ident) => {
        pub fn $fn_name(
            arguments: &[Expression],
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(Expression::Boolean(Boolean::from(matches!(
                &arguments[0],
                Expression::$expr_type(_)
            ))))
        }
    };
    ($fn_name:ident, $expr_type:ident !) => {
        pub fn $fn_name(
            arguments: &[Expression],
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(Expression::Boolean(Boolean::from(matches!(
                &arguments[0],
                Expression::$expr_type
            ))))
        }
    };
}

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

macro_rules! is_list_a {
    ($predicate:ident) => {
        is_list_a!($predicate, $predicate);
    };
    ($fn_name:ident, $predicate:ident) => {
        is_list_a!($fn_name => |v: &Pair| v.$predicate());
    };
    ($fn_name:ident => $closure:expr) => {
        pub fn $fn_name(
            arguments: &[Expression],
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(Expression::Boolean(Boolean::from(match &arguments[0] {
                Expression::Quotation(v) => {
                    if let Some(pair) = v.as_list() {
                        $closure(pair)
                    } else {
                        return Err(Error::from(ErrorKind::UnexpectedType {
                            expected: TYPE_NAME_LIST.to_string(),
                            actual: Some(v.type_name().to_string()),
                        }))
                    }
                },
                e => {
                    return Err(Error::from(ErrorKind::UnexpectedType {
                        expected: TYPE_NAME_LIST.to_string(),
                        actual: Some(e.type_name().to_string()),
                    }))
                }
            })))
        }
    };
}

macro_rules! is_typed_a {
    ($predicate:ident, $expr_type:ident, $value_type:ty, $type_name:expr) => {
        is_typed_a!($predicate, $predicate, $expr_type, $value_type, $type_name);
    };
    ($fn_name:ident, $predicate:ident, $expr_type:ident, $value_type:ty, $type_name:expr) => {
        is_typed_a!($fn_name => |v: &$value_type| v.$predicate(), $expr_type, $type_name);
    };
    ($fn_name:ident => $closure:expr, $expr_type:ident, $value_type:ty, $type_name:expr) => {
        pub fn $fn_name(
            arguments: &[Expression],
            _: &mut MutableRef<Environment>,
        ) -> Result<Expression, Error> {
            Ok(Expression::Boolean(Boolean::from(match &arguments[0] {
                Expression::$expr_type(v) => $closure(v),
                e => {
                    return Err(Error::from(ErrorKind::UnexpectedType {
                        expected: $type_name.to_string(),
                        actual: Some(e.type_name().to_string()),
                    }))
                }
            })))
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
