/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::Environment;
use crate::read::syntax_str::{
    SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR, SYNTAX_SPACE, VALUE_NULL_LIST,
};
use crate::types::{Identifier, MutableRef, SchemeRepr};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LibraryName(Vec<Identifier>);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const IDX_RESERVED_PART: usize = 0;
const IDX_SECOND_PART: usize = 1;

const ID_LIB_SCHEME: &str = "scheme";
const ID_LIB_SCHEME_PARTS: &[&str] = &[
    "base",
    "case-lambda",
    "char",
    "complex",
    "cxr",
    "eval",
    "inexact",
    "lazy",
    "load",
    "process",
    "repl",
    "time",
    "write",
    "r5rs",
];

const ID_LIB_SRFI: &str = "srfi";
const ID_LIB_SRFI_PARTS: &[&str] = &[];

const ID_LIB_SCHEMER: &str = "schemer";
const ID_LIB_SCHEMER_PARTS: &[&str] = &[];

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn load_library(
    name: LibraryName,
    into_env: &mut MutableRef<Environment>,
) -> Result<(), Error> {
    if name.is_reserved() {
        if name.is_scheme() {
            match name.get(IDX_SECOND_PART).unwrap().as_str() {
                "base" => into_env.borrow_mut().import(scheme_),
            }
        } else if name.is_srfi() {
            todo!()
        } else if name.is_schemer() {
            todo!()
        }
        Ok(())
    } else {
        load_from_path(&name.to_path().unwrap(), into_env)
    }
}

pub fn load_from_path(
    file_path: &Path,
    into_env: &mut MutableRef<Environment>,
) -> Result<(), Error> {
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Deref for LibraryName {
    type Target = Vec<Identifier>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LibraryName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SchemeRepr for LibraryName {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}",
            SYNTAX_LEFT_PARENTHESIS_CHAR,
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(SYNTAX_SPACE),
            SYNTAX_RIGHT_PARENTHESIS_CHAR,
        )
    }
}

impl LibraryName {
    pub fn new_scheme(name: Identifier) -> Result<Self, Error> {
        Self::new(vec![Identifier::from_str_unchecked(ID_LIB_SCHEME), name])
    }

    pub fn new(name: Vec<Identifier>) -> Result<Self, Error> {
        if name.is_empty() {
            Err(Error::from(ErrorKind::BadLibraryName {
                name: String::from(VALUE_NULL_LIST),
            }))
        } else if name.iter().any(|s| {
            s.chars()
                .any(|c| ['|', '\\', '?', '*', '<', '"', ':', '>', '+', '[', ']', '/'].contains(&c))
        }) {
            Err(Error::from(ErrorKind::BadLibraryName {
                name: Self(name).to_repr_string(),
            }))
        } else {
            if (name.get(IDX_RESERVED_PART).unwrap().as_str() == ID_LIB_SCHEME
                && !ID_LIB_SCHEME_PARTS.contains(&name.get(IDX_SECOND_PART).unwrap().as_str()))
                || (name.get(IDX_RESERVED_PART).unwrap().as_str() == ID_LIB_SRFI
                    && !ID_LIB_SRFI_PARTS.contains(&name.get(IDX_SECOND_PART).unwrap().as_str()))
                || (name.get(IDX_RESERVED_PART).unwrap().as_str() == ID_LIB_SCHEMER
                    && !ID_LIB_SCHEMER_PARTS.contains(&name.get(IDX_SECOND_PART).unwrap().as_str()))
            {
                Err(Error::from(ErrorKind::BadLibraryName {
                    name: Self(name).to_repr_string(),
                }))
            } else {
                Ok(Self(name))
            }
        }
    }

    pub fn is_scheme(&self) -> bool {
        !self.is_empty() && self.get(IDX_RESERVED_PART).unwrap().as_str() == ID_LIB_SCHEME
    }

    pub fn is_srfi(&self) -> bool {
        !self.is_empty() && self.get(IDX_RESERVED_PART).unwrap().as_str() == ID_LIB_SRFI
    }

    pub fn is_schemer(&self) -> bool {
        !self.is_empty() && self.get(IDX_RESERVED_PART).unwrap().as_str() == ID_LIB_SCHEMER
    }

    pub fn is_reserved(&self) -> bool {
        self.is_scheme() || self.is_srfi() || self.is_schemer()
    }

    pub fn is_external(&self) -> bool {
        !self.is_reserved()
    }

    pub fn to_path(&self) -> Option<PathBuf> {
        if self.is_reserved() {
            None
        } else {
            let pb: PathBuf = self.iter().fold(PathBuf::new(), |mut p, n| {
                p.push(n.as_str());
                p
            });
            Some(pb)
        }
    }

    pub fn into_inner(self) -> Vec<Identifier> {
        self.0
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
