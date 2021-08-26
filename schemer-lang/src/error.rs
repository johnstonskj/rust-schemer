/*!
One-line description.

More detailed description, with

# Example

*/

use crate::types::exceptions::RuntimeError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn StdError>>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Parser,
    Value {
        kind: String,
        value: String,
    },
    NumericTruncation {
        from: String,
        to: String,
    },
    TypeCast {
        from: String,
        to: String,
    },
    UnexpectedType {
        expected: String,
        actual: Option<String>,
    },
    Runtime(RuntimeError),
    File,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn make_value_error(kind: &'static str, value: &'static str) -> impl Fn() -> Error {
    move || {
        Error::from(ErrorKind::Value {
            kind: kind.to_string(),
            value: value.to_string(),
        })
    }
}

pub fn make_numeric_truncation_error(from: &'static str, to: &'static str) -> impl Fn() -> Error {
    move || {
        Error::from(ErrorKind::NumericTruncation {
            from: from.to_string(),
            to: to.to_string(),
        })
    }
}

pub fn make_type_cast_error(from: &'static str, to: &'static str) -> impl Fn() -> Error {
    move || {
        Error::from(ErrorKind::TypeCast {
            from: from.to_string(),
            to: to.to_string(),
        })
    }
}

pub fn make_unexpected_type_error(
    expected: &'static str,
    actual: &'static str,
) -> impl Fn() -> Error {
    move || {
        Error::from(ErrorKind::UnexpectedType {
            expected: expected.to_string(),
            actual: Some(actual.to_string()),
        })
    }
}

pub fn make_unknown_unexpected_type_error(expected: &'static str) -> impl Fn() -> Error {
    move || {
        Error::from(ErrorKind::UnexpectedType {
            expected: expected.to_string(),
            actual: None,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.to_string())?;
        if let Some(cause) = self.source() {
            write!(f, " Cause: {}", cause)?;
        }
        Ok(())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::File,
            source: Some(Box::new(e)),
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self {
            kind: ErrorKind::File,
            source: Some(Box::new(e)),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind, source: None }
    }
}

impl<T> From<Error> for Result<T, Error> {
    fn from(e: Error) -> Self {
        Err(e)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl Error {
    pub fn chain(source: Box<dyn StdError>, kind: ErrorKind) -> Self {
        Self {
            kind,
            source: Some(source),
        }
    }

    pub fn is_file_error(&self) -> bool {
        matches!(self.kind, ErrorKind::File)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorKind::Parser => "Error parsing source code, see cause.".to_string(),
                ErrorKind::Value { kind, value } =>
                    format!("Error in parsing the value '{}' as a {}.", value, kind),
                ErrorKind::NumericTruncation { from, to } => format!(
                    "Could not convert from {} to {} without truncation, or loss of precision.",
                    from, to
                ),
                ErrorKind::TypeCast { from, to } =>
                    format!("Invalid cast attempt from type {} to type {}.", from, to),
                ErrorKind::UnexpectedType { expected, actual } => format!(
                    "Unexpected value type; expecting {}, received {}",
                    expected,
                    match actual {
                        None => "<unknown>",
                        Some(s) => s.as_str(),
                    }
                ),
                ErrorKind::Runtime(rte) => {
                    rte.message().to_string()
                }
                ErrorKind::File => {
                    format!("File I/O error")
                }
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
