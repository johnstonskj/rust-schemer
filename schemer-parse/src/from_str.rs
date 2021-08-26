/*!
One-line description.

More detailed description, with

# Example

*/

use crate::parser::parse_number_str;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::read::syntax_str::{
    BOOLEAN_FALSE, BOOLEAN_FALSE_SHORT, BOOLEAN_TRUE, BOOLEAN_TRUE_SHORT, SYNTAX_CHAR_PREFIX,
};
use schemer_lang::types::booleans::TYPE_NAME_BOOLEAN;
use schemer_lang::types::chars::{name_to_char, TYPE_NAME_CHAR};
use schemer_lang::types::{Boolean, Char, Number, Symbol};
use std::convert::TryFrom;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn string_to_boolean(s: &str) -> Result<Boolean, Error> {
    if s == BOOLEAN_TRUE || s == BOOLEAN_TRUE_SHORT {
        Ok(Boolean::from(true))
    } else if s == BOOLEAN_FALSE || s == BOOLEAN_FALSE_SHORT {
        Ok(Boolean::from(false))
    } else {
        Err(ErrorKind::Value {
            kind: TYPE_NAME_BOOLEAN.to_string(),
            value: s.to_string(),
        }
        .into())
    }
}

pub fn string_to_char(s: &str) -> Result<Char, Error> {
    if s.starts_with(SYNTAX_CHAR_PREFIX) {
        u32::from_str_radix(&s[3..], 16)
            .map_err(|e| {
                Error::chain(
                    Box::new(e),
                    ErrorKind::Value {
                        kind: TYPE_NAME_CHAR.to_string(),
                        value: s.to_string(),
                    }
                    .into(),
                )
            })
            .and_then(|cv| Char::try_from(cv))
    } else if s.len() == 3 {
        let cs = &s[2..];
        let c = cs.chars().next().unwrap();
        Ok(c.into())
    } else {
        if let Some(c) = name_to_char(s) {
            Ok(c.into())
        } else {
            Err(ErrorKind::Value {
                kind: TYPE_NAME_CHAR.to_string(),
                value: s.to_string(),
            }
            .into())
        }
    }
}

pub fn unicode_name_to_char(_s: &str) -> Result<Char, Error> {
    todo!()
}

pub fn string_to_number(s: &str) -> Result<Number, Error> {
    parse_number_str(s)
}

pub fn string_to_symbol(_s: &str) -> Result<Symbol, Error> {
    todo!()
}

pub fn string_to_list(_s: &str) -> Result<Symbol, Error> {
    todo!()
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
