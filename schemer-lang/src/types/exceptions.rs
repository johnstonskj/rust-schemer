/*!
One-line description.

More detailed description, with

# Example

*/

use crate::read::datum::Datum;
use crate::types::{Identifier, SchemeRepr, Symbol};
use std::error::Error;
use std::fmt::{Display, Formatter};
use unique_id::string::StringGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeError {
    source: RuntimeSource,
    message: String,
    irritants: Vec<Datum>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeSource {
    ReadError,
    FileError,
    Other(Symbol),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn error_identifier() -> Identifier {
    Identifier::from_str_unchecked(&format!("error-{}", StringGenerator::default().next_id()))
}

pub fn is_file_error(err: &RuntimeError) -> bool {
    err.source == RuntimeSource::FileError
}

pub fn is_read_error(err: &RuntimeError) -> bool {
    err.source == RuntimeSource::ReadError
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RuntimeError {}

impl RuntimeError {
    pub fn new(source: RuntimeSource, message: &str) -> Self {
        Self {
            source,
            message: message.to_string(),
            irritants: Vec::default(),
        }
    }

    pub fn new_with_irritants(source: RuntimeSource, message: &str, irritants: &[Datum]) -> Self {
        Self {
            source,
            message: message.to_string(),
            irritants: irritants.to_vec(),
        }
    }

    pub fn source(&self) -> &RuntimeSource {
        &self.source
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn irritants(&self) -> impl Iterator<Item = &Datum> {
        self.irritants.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RuntimeSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RuntimeSource::ReadError => "read-error".to_string(),
                RuntimeSource::FileError => "file-error".to_string(),
                RuntimeSource::Other(s) => s.to_repr_string(),
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
