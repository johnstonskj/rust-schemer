/*!
One-line description.

More detailed description, with

# Example

*/

use crate::parser::parse_number_str;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::read::syntax_str::{
    SYNTAX_CHAR_PREFIX, SYNTAX_HEX_CHAR_PREFIX, VALUE_BOOLEAN_FALSE, VALUE_BOOLEAN_FALSE_SHORT,
    VALUE_BOOLEAN_TRUE, VALUE_BOOLEAN_TRUE_SHORT,
};
use schemer_lang::types::booleans::TYPE_NAME_BOOLEAN;
use schemer_lang::types::chars::{name_to_char, TYPE_NAME_CHAR};
use schemer_lang::types::{Boolean, Char, Identifier, Number};
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
    if s == VALUE_BOOLEAN_TRUE || s == VALUE_BOOLEAN_TRUE_SHORT {
        Ok(Boolean::from(true))
    } else if s == VALUE_BOOLEAN_FALSE || s == VALUE_BOOLEAN_FALSE_SHORT {
        Ok(Boolean::from(false))
    } else {
        Err(ErrorKind::ParseValue {
            kind: TYPE_NAME_BOOLEAN.to_string(),
            value: s.to_string(),
        }
        .into())
    }
}

pub fn string_to_char(s: &str) -> Result<Char, Error> {
    let char_length = s.chars().count();
    if s.starts_with(SYNTAX_HEX_CHAR_PREFIX) {
        u32::from_str_radix(&s[3..], 16)
            .map_err(|e| {
                Error::chain(
                    Box::new(e),
                    ErrorKind::ParseValue {
                        kind: TYPE_NAME_CHAR.to_string(),
                        value: s.to_string(),
                    }
                    .into(),
                )
            })
            .and_then(|cv| Char::try_from(cv))
    } else if s.starts_with(SYNTAX_CHAR_PREFIX) && char_length == 3 {
        let cs = &s[2..];
        let c = cs.chars().next().unwrap();
        Ok(c.into())
    } else {
        if let Some(c) = name_to_char(s) {
            Ok(c.into())
        } else {
            Err(ErrorKind::ParseValue {
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

pub fn string_to_symbol(_s: &str) -> Result<Identifier, Error> {
    todo!()
}

pub fn string_to_list(_s: &str) -> Result<Identifier, Error> {
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
