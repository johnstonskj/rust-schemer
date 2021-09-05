/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::eval::environment::Exports;
use schemer_lang::eval::expression::Expression;
use schemer_lang::eval::forms::Form;
use schemer_lang::read::syntax_str::FORM_NAME_IMPORT;
use schemer_lang::types::Identifier;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub fn standard_form_exports() -> Exports {
    let mut exports = Exports::default();

    export_standard_form!(exports, FORM_NAME_IMPORT => import_form "import");

    exports
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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
use import::import as import_form;
