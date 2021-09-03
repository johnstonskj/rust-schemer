/*!
One-line description.

More detailed description, with

# Example

 */

#[macro_use]
extern crate lazy_static;

use crate::scheme::base::scheme_base_exports;
use crate::scheme::r5rs::scheme_r5rs_exports;
use crate::schemer::environment::schemer_environment_exports;
use crate::schemer::repl::schemer_repl_exports;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::forms::standard_form_exports;
use schemer_lang::eval::Environment;
use schemer_lang::types::{Integer, MutableRef};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum PresetEnvironmentKind {
    Interaction,
    Null(Integer),
    Report(Integer),
    SchemeBase,
}

pub const DEFAULT_SCHEME_ENVIRONMENT_VERSION: Integer = 5;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn make_preset_environment(
    preset: PresetEnvironmentKind,
) -> Result<MutableRef<Environment>, Error> {
    match preset {
        PresetEnvironmentKind::Interaction => {
            let base = make_preset_environment(PresetEnvironmentKind::Report(
                DEFAULT_SCHEME_ENVIRONMENT_VERSION,
            ))?;
            let repl = Environment::new_child_named(base, "*repl*");
            repl.borrow_mut().import(schemer_repl_exports())?;
            repl.borrow_mut().import(schemer_environment_exports())?;
            Ok(repl)
        }
        PresetEnvironmentKind::Null(v) => {
            if v == 5 {
                let top = Environment::top_level();
                top.borrow_mut().import(standard_form_exports())?;
                Ok(top)
            } else {
                Err(Error::from(ErrorKind::UnexpectedValue {
                    type_name: "version".to_string(),
                    expected: "5".to_string(),
                    actual: v.to_string(),
                }))
            }
        }
        PresetEnvironmentKind::Report(v) => {
            let base = make_preset_environment(PresetEnvironmentKind::Null(v))?;
            let report = Environment::new_child_named(base, &format!("r{}rs", v));
            report.borrow_mut().import(scheme_r5rs_exports())?;
            Ok(report)
        }
        PresetEnvironmentKind::SchemeBase => {
            let base = make_preset_environment(PresetEnvironmentKind::Null(
                DEFAULT_SCHEME_ENVIRONMENT_VERSION,
            ))?;
            let scheme_base = Environment::new_child_named(base, "base");
            scheme_base.borrow_mut().import(scheme_base_exports())?;
            Ok(scheme_base)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
pub mod macros;

pub mod import;

pub mod scheme;

pub mod schemer;
