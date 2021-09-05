#[macro_export]
macro_rules! num_op {
    ($number:expr, $op_name:ident, $rhs:ident) => {
        match $rhs {
            Number::ExactComplex(rhs) => $number.$op_name(rhs).into(),
            Number::InexactComplex(rhs) => $number.$op_name(rhs).into(),
            Number::ExactReal(rhs) => $number.$op_name(rhs).into(),
            Number::InexactReal(rhs) => $number.$op_name(rhs).into(),
            Number::Rational(rhs) => $number.$op_name(rhs).into(),
            Number::Integer(rhs) => $number.$op_name(rhs).into(),
        }
    };
    ($number:expr, $op_name:ident) => {
        match $number {
            Number::ExactComplex(v) => v.$op_name().into(),
            Number::InexactComplex(v) => v.$op_name().into(),
            Number::ExactReal(v) => v.$op_name().into(),
            Number::InexactReal(v) => v.$op_name().into(),
            Number::Rational(v) => v.$op_name().into(),
            Number::Integer(v) => v.$op_name().into(),
        }
    };
}

#[macro_export]
macro_rules! num_match_fn {
    ($number:expr, $fn_name:ident) => {
        match $number {
            Number::ExactComplex(v) => v.$fn_name(),
            Number::InexactComplex(v) => v.$fn_name(),
            Number::ExactReal(v) => v.$fn_name(),
            Number::InexactReal(v) => v.$fn_name(),
            Number::Rational(v) => v.$fn_name(),
            Number::Integer(v) => v.$fn_name(),
        }
    };
    ($number:expr, $fn_name:ident, $arg:expr) => {
        match $number {
            Number::ExactComplex(v) => v.$fn_name($arg),
            Number::InexactComplex(v) => v.$fn_name($arg),
            Number::ExactReal(v) => v.$fn_name($arg),
            Number::InexactReal(v) => v.$fn_name($arg),
            Number::Rational(v) => v.$fn_name($arg),
            Number::Integer(v) => v.$fn_name($arg),
        }
    };
}
