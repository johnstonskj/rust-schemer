#[macro_export]
macro_rules! to_dstring {
    ($v:expr) => {
        estring!($v.to_string())
    };
}

#[macro_export]
macro_rules! dstring {
    ($v:expr) => {
        Datum::String(SchemeString::from($v))
    };
}

#[macro_export]
macro_rules! dinexact_real {
    ($v:expr) => {
        Datum::Number(Number::InexactReal((InexactReal::from($v))))
    };
}

#[macro_export]
macro_rules! dinteger {
    ($v:expr) => {
        Datum::Number(Number::Integer((Integer::from($v))))
    };
}

#[macro_export]
macro_rules! dtrue {
    () => {
        dboolean!(true)
    };
}

#[macro_export]
macro_rules! dfalse {
    () => {
        dboolean!(false)
    };
}

#[macro_export]
macro_rules! dboolean {
    ($v:expr) => {
        Datum::Boolean(Boolean::from($v))
    };
}
