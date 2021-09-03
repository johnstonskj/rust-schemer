/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::lists::vec_to_list;
use schemer_lang::types::strings::TYPE_NAME_STRING;
use schemer_lang::types::{Boolean, Identifier, MutableRef, Pair, SchemeString, SchemeValue};
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Component, PathBuf, Prefix};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn schemer_file_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "path-a-file?" => is_file "path");
    export_builtin!(exports, "path-a-dir?" => is_dir "path");
    export_builtin!(exports, "path-absolute?" => is_absolute "path");
    export_builtin!(exports, "path-relative?" => is_relative "path");
    export_builtin!(exports, "path-extension" => extension "path");
    export_builtin!(exports, "path-file-name" => file_name "path");
    export_builtin!(exports, "path-file-stem" => file_stem "path");
    export_builtin!(exports, "path-canonicalize" => canonicalize_path "path");
    export_builtin!(exports, "path-components" => components "path");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_absolute(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Boolean(Boolean::from(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            file.is_absolute()
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })))
}

fn is_relative(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Boolean(Boolean::from(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            file.is_relative()
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })))
}

fn is_dir(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Boolean(Boolean::from(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            file.is_dir()
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })))
}

fn is_file(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Boolean(Boolean::from(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            file.is_file()
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })))
}

fn extension(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            match file.extension() {
                None => Expression::Boolean(Boolean::from(false)),
                Some(v) => Expression::String(SchemeString::from(v.to_string_lossy().to_string())),
            }
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })
}

fn file_name(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            match file.file_name() {
                None => Expression::Boolean(Boolean::from(false)),
                Some(v) => Expression::String(SchemeString::from(v.to_string_lossy().to_string())),
            }
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })
}

fn file_stem(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(match &arguments[0] {
        Expression::String(file_name) => {
            let file = PathBuf::from(file_name.deref());
            match file.file_stem() {
                None => Expression::Boolean(Boolean::from(false)),
                Some(v) => Expression::String(SchemeString::from(v.to_string_lossy().to_string())),
            }
        }
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    })
}

fn canonicalize_path(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::String(SchemeString::from(
        match &arguments[0] {
            Expression::String(file_name) => {
                let file = PathBuf::from(file_name.deref());
                let new_file = file.canonicalize()?;
                new_file.to_string_lossy().to_string()
            }
            e => {
                return Err(Error::from(ErrorKind::UnexpectedType {
                    expected: TYPE_NAME_STRING.to_string(),
                    actual: Some(e.type_name().to_string()),
                }))
            }
        },
    )))
}

fn components(
    arguments: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::List(match &arguments[0] {
        Expression::String(file_name) => path_to_components(&PathBuf::from(file_name.deref()))?,
        e => {
            return Err(Error::from(ErrorKind::UnexpectedType {
                expected: TYPE_NAME_STRING.to_string(),
                actual: Some(e.type_name().to_string()),
            }))
        }
    }))
}

fn path_to_components(path: &PathBuf) -> Result<Vec<Expression>, Error> {
    Ok(path
        .components()
        .map(|c| match c {
            //
            // Note that prefix components are not reported on Unix
            //
            Component::Prefix(p) => match p.kind() {
                Prefix::Verbatim(v) => Expression::Quotation(
                    Datum::List(Pair::cons(
                        string_to_symbol("prefix-verbatim").into(),
                        os_string_to_string(v).into(),
                    ))
                    .into(),
                ),
                Prefix::VerbatimUNC(host, share) => {
                    let result = vec_to_list(vec![
                        string_to_symbol("prefix-verbatim-unc"),
                        os_string_to_string(host),
                        os_string_to_string(share),
                    ]);
                    Expression::Quotation(Datum::List(result).into())
                }
                Prefix::VerbatimDisk(v) => {
                    let disk_char = v as char;
                    Expression::Quotation(
                        Datum::List(Pair::cons(
                            string_to_symbol("prefix-verbatim-disk").into(),
                            string_to_string(disk_char.to_string()).into(),
                        ))
                        .into(),
                    )
                }
                Prefix::DeviceNS(v) => Expression::Quotation(
                    Datum::List(Pair::cons(
                        string_to_symbol("prefix-device-ns").into(),
                        os_string_to_string(v).into(),
                    ))
                    .into(),
                ),
                Prefix::UNC(host, share) => {
                    let result = vec_to_list(vec![
                        string_to_symbol("prefix-unc"),
                        os_string_to_string(host),
                        os_string_to_string(share),
                    ]);
                    Expression::Quotation(Datum::List(result).into())
                }
                Prefix::Disk(v) => {
                    let disk_char = v as char;
                    Expression::Quotation(
                        Datum::List(Pair::cons(
                            string_to_symbol("prefix-disk").into(),
                            string_to_string(disk_char.to_string()).into(),
                        ))
                        .into(),
                    )
                }
            },
            Component::RootDir => string_to_quoted_symbol("root"),
            Component::CurDir => string_to_quoted_symbol("current"),
            Component::ParentDir => string_to_quoted_symbol("parent"),
            Component::Normal(v) => Expression::Quotation(
                Datum::List(Pair::cons(
                    string_to_symbol("normal").into(),
                    os_string_to_string(v).into(),
                ))
                .into(),
            ),
        })
        .collect::<Vec<Expression>>())
}

fn string_to_quoted_symbol(s: &str) -> Expression {
    Expression::Quotation(string_to_symbol(s).into())
}

fn string_to_symbol(s: &str) -> Datum {
    Datum::Symbol(Identifier::from_str_unchecked(s))
}

fn os_string_to_string(s: &OsStr) -> Datum {
    string_to_string(s.to_string_lossy().to_string())
}

fn string_to_string(s: String) -> Datum {
    Datum::String(SchemeString::from(s))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
