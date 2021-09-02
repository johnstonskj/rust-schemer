/*!
One-line description.

More detailed description, with

# Example

*/

use crate::{make_preset_environment, PresetEnvironmentKind};
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

pub fn scheme_repl_exports() -> Exports {
    let mut exports = Exports::default();

    export_builtin!(exports, "interaction-environment" => interaction_environment);

    exports
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn interaction_environment(
    _: &[Expression],
    _: &mut MutableRef<Environment>,
) -> Result<Expression, Error> {
    Ok(Expression::Environment(make_preset_environment(
        PresetEnvironmentKind::Interaction,
    )?))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
