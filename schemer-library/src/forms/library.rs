/*!
One-line description.

More detailed description, with

# Example

*/

use crate::forms::import::FILE_PATH_EXTENSION;
use crate::scheme::ID_LIB_SCHEME;
use crate::schemer::ID_LIB_SCHEMER;
use crate::srfi::ID_LIB_SRFI;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::read::syntax_str::{
    SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR, SYNTAX_SPACE, VALUE_NULL_LIST,
};
use schemer_lang::types::{Identifier, Integer, SchemeRepr};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LibraryName(Vec<LibraryNamePart>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LibraryNamePart {
    Identifier(Identifier),
    Number(Integer),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const IDX_RESERVED_PART: usize = 0;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Deref for LibraryName {
    type Target = Vec<LibraryNamePart>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LibraryName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<LibraryNamePart>> for LibraryName {
    fn from(v: Vec<LibraryNamePart>) -> Self {
        Self(v)
    }
}

impl From<LibraryName> for Vec<LibraryNamePart> {
    fn from(v: LibraryName) -> Self {
        v.0
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
    pub(crate) fn new_unchecked(name: &[&str]) -> Self {
        Self(
            name.iter()
                .map(|id| LibraryNamePart::from(id_from_str!(id)))
                .collect(),
        )
    }

    pub fn scheme(id: &str) -> Result<Self, Error> {
        Ok(Self::from(vec![
            LibraryNamePart::scheme(),
            LibraryNamePart::from(Identifier::from_str(id)?),
        ]))
    }

    pub fn srfi(id: Integer) -> Result<Self, Error> {
        Ok(Self::from(vec![
            LibraryNamePart::srfi(),
            LibraryNamePart::from(id),
        ]))
    }

    pub fn schemer(id: &str) -> Result<Self, Error> {
        Ok(Self::from(vec![
            LibraryNamePart::schemer(),
            LibraryNamePart::from(Identifier::from_str(id)?),
        ]))
    }

    pub fn new(name: Vec<LibraryNamePart>) -> Result<Self, Error> {
        if name.is_empty() {
            Err(Error::from(ErrorKind::BadLibraryName {
                name: String::from(VALUE_NULL_LIST),
            }))
        } else if name.iter().any(|part| match part {
            LibraryNamePart::Identifier(v) => !LibraryNamePart::is_valid(v),
            LibraryNamePart::Number(_) => false,
        }) {
            Err(Error::from(ErrorKind::BadLibraryName {
                name: Self(name).to_repr_string(),
            }))
        } else {
            Ok(Self(name))
        }
    }

    pub fn is_scheme(&self) -> bool {
        !self.is_empty() && self.get(IDX_RESERVED_PART).unwrap().is_scheme()
    }

    pub fn is_srfi(&self) -> bool {
        !self.is_empty() && self.get(IDX_RESERVED_PART).unwrap().is_srfi()
    }

    pub fn is_schemer(&self) -> bool {
        !self.is_empty() && self.get(IDX_RESERVED_PART).unwrap().is_schemer()
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
                p.push(&n.to_string());
                p
            });
            let pb = pb.with_extension(FILE_PATH_EXTENSION);
            Some(pb)
        }
    }

    pub fn into_inner(self) -> Vec<LibraryNamePart> {
        self.0
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LibraryNamePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LibraryNamePart::Identifier(v) => v.to_string(),
                LibraryNamePart::Number(v) => v.to_string(),
            }
        )
    }
}

impl From<Identifier> for LibraryNamePart {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl From<Integer> for LibraryNamePart {
    fn from(v: Integer) -> Self {
        Self::Number(v)
    }
}

impl LibraryNamePart {
    pub fn scheme() -> Self {
        Self::Identifier(Identifier::from_str_unchecked(ID_LIB_SCHEME))
    }

    pub fn srfi() -> Self {
        Self::Identifier(Identifier::from_str_unchecked(ID_LIB_SRFI))
    }

    pub fn schemer() -> Self {
        Self::Identifier(Identifier::from_str_unchecked(ID_LIB_SCHEMER))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_scheme(&self) -> bool {
        if let Self::Identifier(id) = self {
            id.as_str() == ID_LIB_SCHEME
        } else {
            false
        }
    }

    pub fn is_srfi(&self) -> bool {
        if let Self::Identifier(id) = self {
            id.as_str() == ID_LIB_SRFI
        } else {
            false
        }
    }

    pub fn is_schemer(&self) -> bool {
        if let Self::Identifier(id) = self {
            id.as_str() == ID_LIB_SCHEMER
        } else {
            false
        }
    }

    pub fn is_reserved(&self) -> bool {
        self.is_scheme() || self.is_srfi() || self.is_schemer()
    }

    pub fn is_external(&self) -> bool {
        !self.is_reserved()
    }

    pub fn is_valid(id: &str) -> bool {
        !id.chars()
            .any(|c| ['|', '\\', '?', '*', '<', '"', ':', '>', '+', '[', ']', '/'].contains(&c))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
