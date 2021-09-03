/*!
One-line description.

More detailed description, with

# Example

 */

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate schemer_macros;

use crate::scheme::base::scheme_base_exports;
use crate::scheme::r5rs::scheme_r5rs_exports;
use crate::schemer::environment::schemer_environment_exports;
use crate::schemer::load::schemer_load_exports;
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
            base.borrow_mut().make_immutable();
            let interaction = Environment::new_child_named(base, "*interaction*");
            interaction.borrow_mut().import(schemer_repl_exports())?;
            interaction
                .borrow_mut()
                .import(schemer_environment_exports())?;
            interaction.borrow_mut().import(schemer_load_exports())?;
            interaction.borrow_mut().make_immutable();
            Ok(interaction)
        }
        PresetEnvironmentKind::Null(v) => {
            if v == 5 {
                let top = Environment::top_level();
                top.borrow_mut().import(standard_form_exports())?;
                top.borrow_mut().make_immutable();
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
            base.borrow_mut().make_immutable();
            let report = Environment::new_child_named(base, &format!("*r{}rs*", v));
            report.borrow_mut().import(scheme_r5rs_exports())?;
            report.borrow_mut().make_immutable();
            Ok(report)
        }
        PresetEnvironmentKind::SchemeBase => {
            let base = make_preset_environment(PresetEnvironmentKind::Null(
                DEFAULT_SCHEME_ENVIRONMENT_VERSION,
            ))?;
            base.borrow_mut().make_immutable();
            let scheme_base = Environment::new_child_named(base, "*scheme-base*");
            scheme_base.borrow_mut().import(scheme_base_exports())?;
            scheme_base.borrow_mut().make_immutable();
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

pub mod import;

pub mod scheme;

pub mod schemer;

pub mod srfi;
