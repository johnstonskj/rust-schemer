/*!
One-line description.

More detailed description, with

# Example

 */

use crate::forms::import::LibraryName;
use crate::scheme::base::numbers::scheme_base_number_exports;
use crate::scheme::base::ports::scheme_base_ports_exports;
use crate::scheme::base::strings::scheme_base_string_exports;
use crate::scheme::base::types::scheme_base_type_predicates_exports;
use crate::scheme::base::write::scheme_base_write_exports;
use crate::scheme::ID_LIB_SCHEME;
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

library_name!(ID_LIB_SCHEME_BASE, "base", ID_LIB_SCHEME, scheme_base_name);

pub fn scheme_base_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "features" => features);

    exports.import(scheme_base_number_exports());
    exports.import(scheme_base_ports_exports());
    exports.import(scheme_base_string_exports());
    exports.import(scheme_base_type_predicates_exports());
    exports.import(scheme_base_write_exports());

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn features(
    _: Vec<Expression>,
    environment: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    forms::quote(
        vec![Datum::List(vector_to_list(
            vec![
                id_from_str!("r7rs"),
                id_from_str!("exact-closed"),
                id_from_str!("exact-complex"),
                id_from_str!("ieee-float"),
                id_from_str!("full-unicode"),
                id_from_str!("ratios"),
                operating_system(),
                architecture(),
                byte_order(),
                id_from_str!(IMPLEMENTATION_NAME),
                id_from_str!(&format!(
                    "{}-{}",
                    IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION
                )),
                #[cfg(feature = "char-names")]
                id_from_str!("unicode-char-names"),
                #[cfg(feature = "big-num-x")]
                id_from_str!("big-numbers"),
            ]
            .into_iter()
            .map(|symbol| Datum::from(Identifier::from(symbol)))
            .collect(),
        ))
        .into()],
        environment,
    )
}

// TODO: merge this and SRFI-112

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

pub mod numbers;

pub mod ports;

pub mod types;

pub mod strings;

pub mod write;
