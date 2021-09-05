/*!
One-line description.

More detailed description, with

# Example

*/

use crate::scheme::base::{scheme_base_exports, scheme_base_name};
use crate::scheme::case_lambda::{scheme_case_lambda_exports, scheme_case_lambda_name};
use crate::scheme::chars::{scheme_chars_exports, scheme_chars_name};
use crate::scheme::complex::{scheme_complex_exports, scheme_complex_name};
use crate::scheme::cxr::{scheme_cxr_exports, scheme_cxr_name};
use crate::scheme::eval::{scheme_eval_exports, scheme_eval_name};
use crate::scheme::file::{scheme_file_exports, scheme_file_name};
use crate::scheme::inexact::{scheme_inexact_exports, scheme_inexact_name};
use crate::scheme::lazy::{scheme_lazy_exports, scheme_lazy_name};
use crate::scheme::load::{scheme_load_exports, scheme_load_name};
use crate::scheme::process_context::{scheme_process_context_exports, scheme_process_context_name};
use crate::scheme::r5rs::{scheme_r5rs_exports, scheme_r5rs_name};
use crate::scheme::read::{scheme_read_exports, scheme_read_name};
use crate::scheme::repl::{scheme_repl_exports, scheme_repl_name};
use crate::scheme::time::{scheme_time_exports, scheme_time_name};
use crate::scheme::write::{scheme_write_exports, scheme_write_name};
use crate::scheme::ID_LIB_SCHEME;
use crate::schemer::ID_LIB_SCHEMER;
use crate::srfi::ID_LIB_SRFI;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression};
use schemer_lang::read::datum::Datum;
use schemer_lang::read::syntax_str::{
    SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR, SYNTAX_SPACE, VALUE_NULL_LIST,
};
use schemer_lang::types::{Identifier, Integer, MutableRef, Ref, SchemeRepr};
use schemer_lang::IMPLEMENTATION_NAME;
use search_path::SearchPath;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LibraryName(Vec<Identifier>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LibraryNamePart {
    Identifier(Identifier),
    Number(Integer),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const IDX_RESERVED_PART: usize = 0;

lazy_static! {
    static ref RESERVED_LIBRARIES: HashMap<String, fn() -> Exports> = reserved_libraries();
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn library_path() -> SearchPath {
    let mut search_path = SearchPath::new_or_default("SCHEMER_LIB");
    xdirs::data_local_dir_for(IMPLEMENTATION_NAME).map(|mut p| {
        p.push("lib");
        search_path.append(p)
    });
    search_path.append(PathBuf::from("lib"));
    search_path
}

pub fn load_library_exports(name: LibraryName) -> Result<Exports, Error> {
    if let Some(exports) = RESERVED_LIBRARIES.get(&name.to_repr_string()) {
        Ok((exports)())
    } else {
        load_external_library(name)
    }
}

pub(super) fn import(
    _arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn load_external_library(name: LibraryName) -> Result<Exports, Error> {
    Err(Error::from(ErrorKind::NoLibraryNamed {
        name: name.to_repr_string(),
    }))
}

fn reserved_libraries() -> HashMap<String, fn() -> Exports> {
    vec![
        (
            scheme_base_name().to_repr_string(),
            scheme_base_exports as fn() -> Exports,
        ),
        (
            scheme_case_lambda_name().to_repr_string(),
            scheme_case_lambda_exports as fn() -> Exports,
        ),
        (
            scheme_chars_name().to_repr_string(),
            scheme_chars_exports as fn() -> Exports,
        ),
        (
            scheme_complex_name().to_repr_string(),
            scheme_complex_exports as fn() -> Exports,
        ),
        (
            scheme_cxr_name().to_repr_string(),
            scheme_cxr_exports as fn() -> Exports,
        ),
        (
            scheme_eval_name().to_repr_string(),
            scheme_eval_exports as fn() -> Exports,
        ),
        (
            scheme_file_name().to_repr_string(),
            scheme_file_exports as fn() -> Exports,
        ),
        (
            scheme_inexact_name().to_repr_string(),
            scheme_inexact_exports as fn() -> Exports,
        ),
        (
            scheme_lazy_name().to_repr_string(),
            scheme_lazy_exports as fn() -> Exports,
        ),
        (
            scheme_load_name().to_repr_string(),
            scheme_load_exports as fn() -> Exports,
        ),
        (
            scheme_process_context_name().to_repr_string(),
            scheme_process_context_exports as fn() -> Exports,
        ),
        (
            scheme_r5rs_name().to_repr_string(),
            scheme_r5rs_exports as fn() -> Exports,
        ),
        (
            scheme_read_name().to_repr_string(),
            scheme_read_exports as fn() -> Exports,
        ),
        (
            scheme_repl_name().to_repr_string(),
            scheme_repl_exports as fn() -> Exports,
        ),
        (
            scheme_time_name().to_repr_string(),
            scheme_time_exports as fn() -> Exports,
        ),
        (
            scheme_write_name().to_repr_string(),
            scheme_write_exports as fn() -> Exports,
        ),
    ]
    .iter()
    .cloned()
    .collect()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
