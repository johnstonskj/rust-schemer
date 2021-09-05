/*!
One-line description.

More detailed description, with

# Example

 */

use super::ID_LIB_SRFI;
use crate::forms::import::LibraryName;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::Procedure;
use schemer_lang::eval::{Environment, Expression};
use schemer_lang::types::{Identifier, MutableRef, SchemeString};
use schemer_lang::{IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

library_name!(ID_LIB_SRFI_112, "112", ID_LIB_SRFI, srfi_112_name);

pub fn srfi_112_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "implementation-name" => implementation_name);
    export_builtin!(exports, "implementation-version" => implementation_version);
    export_builtin!(exports, "cpu-architecture" => cpu_architecture);
    export_builtin!(exports, "machine-name" => machine_name);
    export_builtin!(exports, "os-name" => os_name);
    export_builtin!(exports, "os-version" => os_version);

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn implementation_name(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(to_estring!(IMPLEMENTATION_NAME))
}

fn implementation_version(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(to_estring!(IMPLEMENTATION_VERSION))
}

fn cpu_architecture(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(eid_from_str!(&std::env::consts::ARCH.replace("_", "-")))
}

fn machine_name(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(to_estring!(sys_info::hostname().map_err(|e| {
        Error::chain(Box::new(e), ErrorKind::OperatingSystem)
    })?))
}

fn os_name(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(eid_from_str!(match std::env::consts::OS {
        "macos" => "darwin",
        "linux" => "gnu-linux",
        _ => std::env::consts::OS,
    }))
}

fn os_version(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(to_estring!(sys_info::os_release().map_err(|e| {
        Error::chain(Box::new(e), ErrorKind::OperatingSystem)
    })?))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
