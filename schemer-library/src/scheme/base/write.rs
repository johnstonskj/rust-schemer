/*!
One-line description.

More detailed description, with

# Example

 */

use schemer_lang::error::Error;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{Environment, Expression, Procedure};
use schemer_lang::types::{Identifier, MutableRef};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn scheme_base_write_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "newline" => newline "output-port");
    export_builtin!(exports, "write-char" => write_char "char" ; "output-port");
    export_builtin!(exports, "write-string" => write_string "str" ; "output-port");
    export_builtin!(exports, "write-u8" => write_u8 "byte" ; "output-port");
    export_builtin!(exports, "write-bytevector" => write_byte_vector "bytevector" ; "output-port");
    export_builtin!(exports, "flush-output-port" => flush "obj" ; "output-port");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn newline(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn write_char(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn write_string(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn write_u8(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn write_byte_vector(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn flush(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
