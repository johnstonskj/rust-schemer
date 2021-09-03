#[macro_export]
macro_rules! unexpected_type {
    ($expected:expr, $actual:expr) => {
        Err(Error::from(ErrorKind::UnexpectedType {
            expected: $expected.to_string(),
            actual: Some($actual.type_name().to_string()),
        }))
    };
    (=> $expected:expr, $actual:expr) => {
        return unexpected_type!($expected, $actual)
    };
    ($expected:expr) => {
        Err(Error::from(ErrorKind::UnexpectedType {
            expected: $expected.to_string(),
            actual: None,
        }))
    };
    (=> $expected:expr) => {
        return unexpected_type!($expected)
    };
}
