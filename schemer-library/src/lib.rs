/*!
One-line description.

More detailed description, with

# Example

 */

#[macro_use]
extern crate lazy_static;

use crate::scheme::r5rs::scheme_r5rs_exports;
use crate::schemer::environment::schemer_environment_exports;
use crate::schemer::repl::schemer_repl_exports;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::forms::standard_form_exports;
use schemer_lang::eval::Environment;
use schemer_lang::types::{Integer, Ref};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum PresetEnvironmentKind {
    Interaction,
    Null(Integer),
    Report(Integer),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn make_preset_environment(preset: PresetEnvironmentKind) -> Result<Ref<Environment>, Error> {
    match preset {
        PresetEnvironmentKind::Interaction => {
            let base = make_preset_environment(PresetEnvironmentKind::Report(5))?;
            let mut repl = Environment::new_child_named(&base, "*repl*");
            repl.import(schemer_repl_exports())?;
            repl.import(schemer_environment_exports())?;
            Ok(Ref::new(repl).into())
        }
        PresetEnvironmentKind::Null(v) => {
            if v == 5 {
                let mut top = Environment::top_level();
                {
                    let top = Ref::get_mut(&mut top).unwrap();
                    top.import(standard_form_exports())?;
                }
                Ok(top.into())
            } else {
                Err(Error::from(ErrorKind::UnexpectedValue {
                    name: "version".to_string(),
                    expected: "5".to_string(),
                    actual: v.to_string(),
                }))
            }
        }
        PresetEnvironmentKind::Report(v) => {
            let base = make_preset_environment(PresetEnvironmentKind::Null(v))?;
            let mut report = Environment::new_child_named(&base, &format!("r{}rs", v));
            report.import(scheme_r5rs_exports())?;
            Ok(Ref::new(report).into())
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

pub mod scheme;

pub mod schemer;
