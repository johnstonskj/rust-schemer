/*!
One-line description.

More detailed description, with

# Example

 */

use crate::forms::library::LibraryName;
use crate::schemer::ID_LIB_SCHEMER;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::Procedure;
use schemer_lang::eval::{Environment, Expression};
use schemer_lang::types::{Identifier, MutableRef, SchemeString};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

library_name!(
    ID_LIB_SCHEMER_ENV_INQUIRY,
    "environment-inquiry",
    ID_LIB_SCHEMER,
    schemer_env_inquiry_name
);

pub fn schemer_env_inquiry_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "byte-order" => byte_order);
    export_builtin!(exports, "cpu-count" => cpu_count);
    export_builtin!(exports, "cpu-speed" => cpu_speed);
    export_builtin!(exports, "os-family" => os_family);

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn byte_order(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(eid_from_str!(if cfg!(target_endian = "big") {
        "big-endian"
    } else {
        "little-endian"
    }))
}

fn cpu_count(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(to_estring!(sys_info::cpu_num().map_err(|e| {
        Error::chain(Box::new(e), ErrorKind::OperatingSystem)
    })?))
}

fn cpu_speed(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(to_estring!(sys_info::cpu_speed().map_err(|e| {
        Error::chain(Box::new(e), ErrorKind::OperatingSystem)
    })?))
}

fn os_family(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    Ok(eid_from_str!(std::env::consts::FAMILY))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
