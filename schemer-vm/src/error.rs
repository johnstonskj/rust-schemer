/*!
One-line description.

More detailed description, with

# Example

*/

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn StdError>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    FileNotFound(PathBuf),
    FileNotWritable(PathBuf),
    FileHeader,
    ReadWrite,
    Format,
    VersionMismatch(u8, u8),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::ReadWrite,
            source: Some(Box::new(e)),
        }
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self {
            kind: ErrorKind::Format,
            source: Some(Box::new(e)),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FileNotFound(path) => format!("File not found for path {:?}", path),
                Self::FileNotWritable(path) => format!("File was not writable for path {:?}", path),
                Self::FileHeader => "File does not contain a valid header.".to_string(),
                Self::ReadWrite => "An error occurred reading or writing".to_string(),
                Self::Format => "An error occurred formatting output".to_string(),
                Self::VersionMismatch(found, expecting) => format!(
                    "File version mismatch, found {}, expecting <= {}.",
                    found, expecting
                ),
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
