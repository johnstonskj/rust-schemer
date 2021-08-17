/*!
One-line description.

More detailed description, with

# Example

*/

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

#[derive(Clone, Debug)]
pub enum ErrorKind {
    Parser,
    Value { kind: String, value: String },
    NumericTruncation { from: String, to: String },
    TypeCast { from: String, to: String },
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
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
            }
        )
    }
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
