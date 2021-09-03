/*!
One-line description.

More detailed description, with

# Example

*/

use crate::scheme::ID_LIB_SCHEME;
use crate::schemer::ID_LIB_SCHEMER;
use crate::srfi::ID_LIB_SRFI;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::Environment;
use schemer_lang::read::syntax_str::{
    SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR, SYNTAX_SPACE, VALUE_NULL_LIST,
};
use schemer_lang::types::{Identifier, MutableRef, SchemeRepr};
use schemer_lang::IMPLEMENTATION_NAME;
use search_path::SearchPath;
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

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// pub fn load_library(
//     name: LibraryName,
//     into_env: &mut MutableRef<Environment>,
// ) -> Result<(), Error> {
//     if name.is_reserved() {
//         if name.is_scheme() {
//             match name.get(IDX_SECOND_PART).unwrap().as_str() {
//                 "base" => into_env.borrow_mut().import(scheme_),
//             }
//         } else if name.is_srfi() {
//             todo!()
//         } else if name.is_schemer() {
//             todo!()
//         }
//         Ok(())
//     } else {
//         load_from_path(&name.to_path().unwrap(), into_env)
//     }
// }

pub fn load_from_path(
    _file_path: &Path,
    _into_env: &mut MutableRef<Environment>,
) -> Result<(), Error> {
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
    pub(crate) fn new_unchecked(name: &[&str]) -> Self {
        Self(name.iter().map(|id| id_from_str!(id)).collect())
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
            if [ID_LIB_SCHEME, ID_LIB_SRFI, ID_LIB_SCHEMER]
                .contains(&name.get(IDX_RESERVED_PART).unwrap().as_str())
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

pub fn library_path() -> SearchPath {
    let mut search_path = SearchPath::new_or_default("SCHEMER_LIB");
    xdirs::data_local_dir_for(IMPLEMENTATION_NAME).map(|mut p| {
        p.push("lib");
        search_path.append(p)
    });
    search_path.append(PathBuf::from("lib"));
    search_path
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
