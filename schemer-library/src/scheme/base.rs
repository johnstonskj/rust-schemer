/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::read::datum::Datum;
use schemer_lang::types::lists::list;
use schemer_lang::types::{Pair, Symbol};
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

///
/// TBD
///
/// # Scheme API
///
/// ```scheme
/// (features) -> (list-of symbol?)
/// ```
///
pub fn features() -> Box<Pair> {
    list(
        vec![
            Symbol::from_str_unchecked("r7rs"),
            Symbol::from_str_unchecked("exact-closed"),
            Symbol::from_str_unchecked("exact-complex"),
            Symbol::from_str_unchecked("ieee-float"),
            Symbol::from_str_unchecked("full-unicode"),
            Symbol::from_str_unchecked("ratios"),
            operating_system(),
            architecture(),
            byte_order(),
            Symbol::from_str_unchecked(IMPLEMENTATION_NAME),
            Symbol::from_str_unchecked(&format!(
                "{}-{}",
                IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION
            )),
            #[cfg(feature = "char-names")]
            Symbol::from_str_unchecked("unicode-char-names"),
            #[cfg(feature = "big-num-x")]
            Symbol::from_str_unchecked("big-numbers"),
        ]
        .into_iter()
        .map(|symbol| Datum::from(Symbol::from(symbol)))
        .collect(),
    )
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn operating_system() -> Symbol {
    Symbol::from_str_unchecked(match std::env::consts::OS {
        "macos" => "darwin",
        "linux" => "gnu-linux",
        _ => std::env::consts::OS,
    })
}

fn architecture() -> Symbol {
    Symbol::from_str_unchecked(&std::env::consts::ARCH.replace("_", "-"))
}

fn byte_order() -> Symbol {
    Symbol::from_str_unchecked(if cfg!(target_endian = "big") {
        "big-endian"
    } else {
        "little-endian"
    })
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
