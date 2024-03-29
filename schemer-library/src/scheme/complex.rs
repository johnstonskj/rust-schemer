/*!
One-line description.

More detailed description, with

# Example

 */

use crate::forms::library::LibraryName;
use crate::scheme::ID_LIB_SCHEME;
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

library_name!(
    ID_LIB_SCHEME_COMPLEX,
    "complex",
    ID_LIB_SCHEME,
    scheme_complex_name
);

pub fn scheme_complex_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "angle" => angle "num");
    export_builtin!(exports, "magnitude" => magnitude "num");
    export_builtin!(exports, "make-rectangle" => make_rectangular_complex "real" "imag");
    export_builtin!(exports, "make-polar" => make_polar_complex "mag" "theta");
    export_builtin!(exports, "real-part" => real_part "num");
    export_builtin!(exports, "imag-part" => imaginary_part "num");

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_rectangular_complex(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn make_polar_complex(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

fn angle(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn magnitude(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn real_part(_: Vec<Expression>, _: &mut MutableRef<Environment>) -> Result<Expression, Error> {
    todo!()
}

fn imaginary_part(
    _: Vec<Expression>,
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
