/*!
One-line description.

More detailed description, with

# Example

*/

use crate::read::datum::Label;
use crate::types::{Identifier, SchemeRepr};
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
    // Parsing ------------------------------------------------------------------------------------
    Parser,
    ParserState {
        input: String,
        state: Option<i32>,
    },
    ParseValue {
        kind: String,
        value: String,
    },
    UnknownReference {
        label: Label,
    },
    // Types --------------------------------------------------------------------------------------
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
    // Values -------------------------------------------------------------------------------------
    ImproperList,
    UnexpectedValue {
        type_name: String,
        expected: String,
        actual: String,
    },
    // Eval ---------------------------------------------------------------------------------------
    UnboundVariable {
        name: Identifier,
    },
    ProcedureArgumentCardinality {
        name: Identifier,
        min: usize,
        max: Option<usize>,
        given: usize,
    },
    BadFormSyntax {
        name: Identifier,
        value: String,
    },
    ImmutableEnvironment,
    ImmutableValue {
        name: Identifier,
        type_name: String,
    },
    // Library ------------------------------------------------------------------------------------
    BadLibraryName {
        name: String,
    },
    NoLibraryNamed {
        name: String,
    },
    Read,
    File,
    OperatingSystem,
    // Shell --------------------------------------------------------------------------------------
    BadArguments,
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
                ErrorKind::ParserState { input, state } =>
                    if let Some(state) = state {
                        format!(
                            "Unexpected input while parsing; input: {}, state: {}.",
                            input, state
                        )
                    } else {
                        format!("Unexpected input while parsing; input: {}.", input)
                    },
                ErrorKind::ParseValue { kind, value } =>
                    format!("Error in parsing the value '{}' as a {}.", value, kind),
                ErrorKind::NumericTruncation { from, to } => format!(
                    "Could not convert from {} to {} without truncation, or loss of precision.",
                    from, to
                ),
                ErrorKind::TypeCast { from, to } =>
                    format!("Invalid cast attempt from type {} to type {}.", from, to),
                ErrorKind::UnexpectedType { expected, actual } => format!(
                    "Unexpected value type; expecting: {}, given: {}.",
                    expected,
                    match actual {
                        None => "<unknown>",
                        Some(s) => s.as_str(),
                    }
                ),
                ErrorKind::Read => {
                    format!("Read error.")
                }
                ErrorKind::File => {
                    format!("File I/O error.")
                }
                ErrorKind::UnknownReference { label } => {
                    format!("Unknown reference to non-shared object: #{}#.", label)
                }
                ErrorKind::UnboundVariable { name } => {
                    format!("Unbound variable: '{}'.", name.to_repr_string())
                }
                ErrorKind::ImproperList => {
                    String::from("Value was not a proper list.")
                }
                ErrorKind::ProcedureArgumentCardinality {
                    name,
                    min,
                    max,
                    given,
                } => {
                    format!(
                        "The procedure '{}' was called with {}, it expects {}.",
                        name.to_repr_string(),
                        given_to_text(given),
                        min_max_to_text(min, max)
                    )
                }
                ErrorKind::BadFormSyntax { name, value } => {
                    format!("Bad syntax in form '{}': {}.", name.to_repr_string(), value)
                }
                ErrorKind::UnexpectedValue {
                    type_name: name,
                    expected,
                    actual,
                } => {
                    format!(
                        "Unexpected value for type {}, expected {}, not {}.",
                        name, expected, actual
                    )
                }
                ErrorKind::ImmutableEnvironment => {
                    format!("The current environment is immutable.")
                }
                ErrorKind::ImmutableValue { name, type_name } => {
                    format!(
                        "Cannot mutate the the {} value named '{}'.",
                        type_name,
                        name.to_repr_string()
                    )
                }
                ErrorKind::BadLibraryName { name } => {
                    format!("Bad syntax for library name: {}.", name,)
                }
                ErrorKind::BadArguments => {
                    format!("Bad argument syntax.")
                }
                ErrorKind::NoLibraryNamed { name } => {
                    format!("No library could be found with the name {}.", name)
                }
                ErrorKind::OperatingSystem => {
                    format!("An error was returned from an operating system, or other platform, interface.")
                }
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn given_to_text(given: &usize) -> String {
    match *given {
        0 => "zero arguments".to_string(),
        1 => "one argument".to_string(),
        v => format!("{} arguments", v),
    }
}

fn min_max_to_text(min: &usize, max: &Option<usize>) -> String {
    let max = max.map(|v| v.to_string()).unwrap_or(String::from("many"));
    if *min == 0 && max == "0" {
        "zero".to_string()
    } else if *min == 1 && max == "1" {
        "only one".to_string()
    } else if *min == 0 && max == "1" {
        format!("zero or one")
    } else if *min == 1 && max == "1" {
        "only one".to_string()
    } else {
        format!("{} to {}", min, max)
    }
}
// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
