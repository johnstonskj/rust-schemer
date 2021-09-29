/*!
One-line description.

More detailed description, with

# Example

*/

use crate::forms::library::{LibraryName, LibraryNamePart};
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
use crate::schemer::base::{schemer_base_exports, schemer_base_name};
use crate::schemer::chars::{schemer_chars_exports, schemer_chars_name};
use crate::schemer::environment::{schemer_environment_exports, schemer_environment_name};
use crate::schemer::environment_inquiry::{schemer_env_inquiry_exports, schemer_env_inquiry_name};
use crate::schemer::file::{schemer_file_exports, schemer_file_name};
use crate::schemer::lists::{schemer_lists_exports, schemer_lists_name};
use crate::schemer::load::{schemer_load_exports, schemer_load_name};
use crate::schemer::repl::{schemer_repl_exports, schemer_repl_name};
use crate::srfi::srfi_112::{srfi_112_exports, srfi_112_name};
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression};
use schemer_lang::read::datum::Datum;
use schemer_lang::read::syntax_str::{
    FORM_PART_EXCEPT, FORM_PART_ONLY, FORM_PART_PREFIX, FORM_PART_RENAME,
};
use schemer_lang::types::lists::{list_to_vec, TYPE_NAME_LIST};
use schemer_lang::types::numbers::TYPE_NAME_INTEGER;
use schemer_lang::types::symbols::TYPE_NAME_SYMBOL;
use schemer_lang::types::{MutableRef, Number, Ref, SchemeRepr};
use schemer_lang::types::{Pair, SchemeValue};
use schemer_lang::IMPLEMENTATION_NAME;
use search_path::SearchPath;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::RwLock;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const LIBRARY_PATH_ENV: &str = "SCHEMER_LIB";

pub const LIBRARY_DIR_NAME: &str = "lib";

pub const FILE_PATH_EXTENSION: &str = "sr";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref LOADED_LIBRARIES: RwLock<HashMap<String, fn() -> Exports>> = reserved_libraries();
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn library_path() -> SearchPath {
    let mut search_path = SearchPath::new_or_default(LIBRARY_PATH_ENV);
    xdirs::data_local_dir_for(IMPLEMENTATION_NAME).map(|mut p| {
        p.push(LIBRARY_DIR_NAME);
        search_path.append(p)
    });
    search_path.append(PathBuf::from(LIBRARY_DIR_NAME));
    search_path
}

/*
## 5.2. Import declarations

An import declaration takes the following form:

    (import ⟨import-set⟩ ...)

An import declaration provides a way to import identifiers exported by a library. Each ⟨import-set⟩
names a set of bindings from a library and possibly specifies local names for the imported bindings.
It takes one of the following forms:

* ⟨library name⟩
* (only ⟨import-set⟩ ⟨identifier⟩ ...)
* (except ⟨import-set⟩ ⟨identifier⟩ ...)
* (prefix ⟨import-set⟩ ⟨identifier⟩)
* (rename ⟨import-set⟩
        (⟨identifier1⟩ ⟨identifier2⟩) . . . )

In the first form, all of the identifiers in the named library’s export clauses are imported with
the same names (or the exported names if exported with rename). The additional ⟨import set⟩ forms
modify this set as follows:

* only produces a subset of the given ⟨import set⟩ including only the listed identifiers (after
  any renaming). It is an error if any of the listed identifiers are not found in the original set.
* except produces a subset of the given ⟨import set⟩, excluding the listed identifiers (after any
  renaming). It is an error if any of the listed identifiers are not found in the original set.
* rename modifies the given ⟨import set⟩, replacing each instance of ⟨identifier1⟩ with
  ⟨identifier2⟩. It is an error if any of the listed ⟨identifier1⟩s are not found in the original
  set.
* prefix automatically renames all identifiers in the given ⟨import set⟩, prefixing each with the
  specified ⟨identifier⟩.

In a program or library declaration, it is an error to import the same identifier more than once
with different bindings, or to redefine or mutate an imported binding with a definition or with
set!, or to refer to an identifier before it is imported. However, a REPL should permit these
actions.
*/
pub(super) fn import(
    arguments: Vec<Ref<Datum>>,
    _env: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    for argument in &arguments {
        if let Datum::List(pair) = argument.deref() {
            import_set(pair, _env)?;
        } else {
            unexpected_type!(=> TYPE_NAME_LIST, argument)
        }
    }
    Ok(Expression::Unspecified)
}

fn import_set(argument: &Pair, env: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    if let Datum::Symbol(id) = argument.car().deref() {
        let imports = match id.deref() {
            FORM_PART_ONLY => import_only(argument.cdr())?,
            FORM_PART_EXCEPT => import_except(argument.cdr())?,
            FORM_PART_PREFIX => import_prefix(argument.cdr())?,
            FORM_PART_RENAME => import_rename(argument.cdr())?,
            _ => import_all(argument)?,
        };
        env.borrow_mut().import(imports)?;
    } else {
        unexpected_type!(=> TYPE_NAME_SYMBOL, argument)
    }
    Ok(Expression::Unspecified)
}

fn import_all(name: &Pair) -> Result<Exports, Error> {
    load_library_exports(list_to_library_name(name)?)
}

fn list_to_library_name(name: &Pair) -> Result<LibraryName, Error> {
    let lib_name: Result<Vec<LibraryNamePart>, Error> = list_to_vec(name.clone())
        .iter()
        .map(|d| match d.deref() {
            Datum::Symbol(id) => Ok(LibraryNamePart::Identifier(id.clone())),
            Datum::Number(Number::Integer(n)) => Ok(LibraryNamePart::Number(*n)),
            _ => unexpected_type!(
                format!("(or {} {})", TYPE_NAME_SYMBOL, TYPE_NAME_INTEGER),
                d
            ),
        })
        .collect();
    Ok(LibraryName::from(lib_name?))
}

fn import_only(_argument: &Ref<Datum>) -> Result<Exports, Error> {
    Ok(Exports::default())
}

fn import_except(_argument: &Ref<Datum>) -> Result<Exports, Error> {
    Ok(Exports::default())
}

fn import_prefix(_argument: &Ref<Datum>) -> Result<Exports, Error> {
    Ok(Exports::default())
}

fn import_rename(_argument: &Ref<Datum>) -> Result<Exports, Error> {
    Ok(Exports::default())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn load_library_exports(name: LibraryName) -> Result<Exports, Error> {
    let loaded = LOADED_LIBRARIES.write().expect("Oops");
    if let Some(exports) = loaded.get(&name.to_repr_string()) {
        Ok((exports)())
    } else {
        println!("loading {:?}", name.to_path());
        println!("path {:?}", library_path());
        if let Some(_load_path) = library_path().find(&name.to_path().unwrap()) {
            todo!()
        } else {
            Err(Error::from(ErrorKind::NoLibraryNamed {
                name: name.to_repr_string(),
            }))
        }
    }
}

fn reserved_libraries() -> RwLock<HashMap<String, fn() -> Exports>> {
    RwLock::new(
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
            // ----------------------------------------------------------------------------------------
            (
                srfi_112_name().to_repr_string(),
                srfi_112_exports as fn() -> Exports,
            ),
            // ----------------------------------------------------------------------------------------
            (
                schemer_base_name().to_repr_string(),
                schemer_base_exports as fn() -> Exports,
            ),
            (
                schemer_chars_name().to_repr_string(),
                schemer_chars_exports as fn() -> Exports,
            ),
            (
                schemer_environment_name().to_repr_string(),
                schemer_environment_exports as fn() -> Exports,
            ),
            (
                schemer_env_inquiry_name().to_repr_string(),
                schemer_env_inquiry_exports as fn() -> Exports,
            ),
            (
                schemer_file_name().to_repr_string(),
                schemer_file_exports as fn() -> Exports,
            ),
            (
                schemer_lists_name().to_repr_string(),
                schemer_lists_exports as fn() -> Exports,
            ),
            (
                schemer_load_name().to_repr_string(),
                schemer_load_exports as fn() -> Exports,
            ),
            (
                schemer_repl_name().to_repr_string(),
                schemer_repl_exports as fn() -> Exports,
            ),
        ]
        .iter()
        .cloned()
        .collect(),
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
