/*!
One-line description.

More detailed description, with

# Example

 */

use crate::scheme::base::predicates::scheme_base_predicates_exports;
use schemer_lang::error::Error;
use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::{forms, Procedure};
use schemer_lang::eval::{Environment, Expression};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::lists::vector_to_list;
use schemer_lang::types::{Identifier, MutableRef};
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

pub fn scheme_base_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "features" => features);

    exports.import(scheme_base_predicates_exports());

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn features(
    _: &[Expression],
    environment: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    forms::quote(
        &[Datum::List(vector_to_list(
            vec![
                Identifier::from_str_unchecked("r7rs"),
                Identifier::from_str_unchecked("exact-closed"),
                Identifier::from_str_unchecked("exact-complex"),
                Identifier::from_str_unchecked("ieee-float"),
                Identifier::from_str_unchecked("full-unicode"),
                Identifier::from_str_unchecked("ratios"),
                operating_system(),
                architecture(),
                byte_order(),
                Identifier::from_str_unchecked(IMPLEMENTATION_NAME),
                Identifier::from_str_unchecked(&format!(
                    "{}-{}",
                    IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION
                )),
                #[cfg(feature = "char-names")]
                Identifier::from_str_unchecked("unicode-char-names"),
                #[cfg(feature = "big-num-x")]
                Identifier::from_str_unchecked("big-numbers"),
            ]
            .into_iter()
            .map(|symbol| Datum::from(Identifier::from(symbol)))
            .collect(),
        ))
        .into()],
        environment,
    )
}

fn operating_system() -> Identifier {
    Identifier::from_str_unchecked(match std::env::consts::OS {
        "macos" => "darwin",
        "linux" => "gnu-linux",
        _ => std::env::consts::OS,
    })
}

fn architecture() -> Identifier {
    Identifier::from_str_unchecked(&std::env::consts::ARCH.replace("_", "-"))
}

fn byte_order() -> Identifier {
    Identifier::from_str_unchecked(if cfg!(target_endian = "big") {
        "big-endian"
    } else {
        "little-endian"
    })
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod predicates;
