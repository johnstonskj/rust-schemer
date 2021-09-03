#[macro_export]
macro_rules! to_estring {
    ($v:expr) => {
        estring!($v.to_string())
    };
}

#[macro_export]
macro_rules! estring {
    ($v:expr) => {
        Expression::String(SchemeString::from($v))
    };
}

#[macro_export]
macro_rules! einexact_real {
    ($v:expr) => {
        Expression::Number(Number::InexactReal((InexactReal::from($v))))
    };
}

#[macro_export]
macro_rules! einteger {
    ($v:expr) => {
        Expression::Number(Number::Integer((Integer::from($v))))
    };
}

#[macro_export]
macro_rules! etrue {
    () => {
        eboolean!(true)
    };
}

#[macro_export]
macro_rules! efalse {
    () => {
        eboolean!(false)
    };
}

#[macro_export]
macro_rules! eboolean {
    ($v:expr) => {
        Expression::Boolean(Boolean::from($v))
    };
}

// macro_rules! expr_is_a {
//     ($v:expr, $var:ident) => {
//         matches!($var, Expression::$var(_))
//     };
//     ($v:expr, $tp:ident !) => {
//         matches!($v, Expression::$var)
//     };
// }
