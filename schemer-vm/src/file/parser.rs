/*!
One-line description.

More detailed description, with

# Example

*/

use crate::machine::Instruction;
use num::complex::Complex;
use num::traits::Zero;
use pest::iterators::Pair;
use pest::Parser;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::parameters::{get_global_flag, DEBUG_SHOW_TOKEN_TREE};
use schemer_lang::read::datum::{Abbreviation, Datum};
use schemer_lang::types::numbers::conv::{
    exact_to_inexact, inexact_complex_to_exact_complex, integer_to_inexact_real,
    rational_to_inexact_real,
};
use schemer_lang::types::numbers::{TYPE_NAME_EXACT_REAL, TYPE_NAME_INTEGER};
use schemer_lang::types::{
    lists::vector_to_list, ExactReal, Identifier, InexactComplex, InexactReal, InfNan, Integer,
    Number, Pair as DatumPair, Rational, Ref, SchemeString, Vector,
};
use schemer_parse::from_str::{string_to_boolean, string_to_char};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "file/secd.pest"]
struct SimpleSyntax;

#[derive(Clone, Debug, PartialEq)]
pub struct Parsed<'a, T>
where
    T: Clone + Debug + PartialEq,
{
    parsed: T,
    rest: Option<&'a str>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const SIGN_NEGATIVE: &str = "-";

macro_rules! debug_token_tree {
    ($parsed:expr) => {
        if get_global_flag(DEBUG_SHOW_TOKEN_TREE).unwrap_or_default() {
            println!(
                "{}",
                pest_ascii_tree::into_ascii_tree($parsed.clone())
                    .expect("Could not print debug token tree")
            );
        }
    };
}

// TODO: report errors with span to make better messages
macro_rules! unexpected_input {
    ($input_pair:expr) => {
        return Err(Error::from(ErrorKind::ParserState {
            input: $input_pair.as_str().to_string(),
            state: None,
        }))
    };
    ($input_pair:expr, $state:expr) => {
        return Err(Error::from(ErrorKind::ParserState {
            input: $input_pair.as_str().to_string(),
            state: Some($state),
        }))
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse_instructions_str(source: &str) -> Result<Vec<Instruction>, Error> {
    println!("parse_instructions_str");
    let mut parsed = SimpleSyntax::parse(Rule::instruction_data, source).map_err(|e| {
        println!("{}", e);
        Error::chain(Box::new(e), ErrorKind::Parser)
    })?;
    debug_token_tree!(parsed);
    let pair = parsed.next().unwrap();
    parse_instruction_data(pair)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// fn make_parsed<'a, T>(parsed: T, original_str: &'a str, matched_str: &str) -> Parsed<'a, T>
// where
//     T: Clone + Debug + PartialEq,
// {
//     let original_len = original_str.len();
//     let matched_len = matched_str.len();
//     if matched_len < original_len {
//         Parsed::new_with_more(parsed, &original_str[matched_len..original_len])
//     } else {
//         Parsed::new(parsed)
//     }
// }

impl<'a, T> Parsed<'a, T>
where
    T: Clone + Debug + PartialEq,
{
    fn new(parsed: T) -> Self {
        Self { parsed, rest: None }
    }

    fn new_with_more(parsed: T, rest: &'a str) -> Self {
        if !rest.is_empty() {
            Self {
                parsed,
                rest: Some(rest),
            }
        } else {
            Self::new(parsed)
        }
    }

    pub fn parsed(&self) -> &T {
        &self.parsed
    }

    pub fn into_parsed(self) -> T {
        self.parsed
    }

    pub fn rest_of_input(&self) -> &Option<&'a str> {
        &self.rest
    }

    pub fn has_more_input(&self) -> bool {
        self.rest.map(|s| !s.is_empty()).unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_number(input_pair: Pair<'_, Rule>) -> Result<Number, Error> {
    let mut radix = 10;
    let mut exact: Option<bool> = None;
    let mut negative = false;

    for inner_pair in input_pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::exactness => {
                exact = if inner_pair.as_str() == "#e" {
                    Some(true)
                } else {
                    Some(false)
                }
            }
            Rule::radix_2 => radix = 2,
            Rule::radix_8 => radix = 8,
            Rule::radix_10 => radix = 10,
            Rule::radix_16 => radix = 16,
            Rule::sign => negative = inner_pair.as_str() == SIGN_NEGATIVE,
            Rule::polar_2 | Rule::polar_8 | Rule::polar_10 | Rule::polar_16 => {
                let number = parse_polar_complex_number(inner_pair, radix)?;
                return Ok(if let Some(true) = exact {
                    Number::from(inexact_complex_to_exact_complex(number)?)
                } else {
                    Number::from(number)
                });
            }
            Rule::cartesian_2 | Rule::cartesian_8 | Rule::cartesian_10 | Rule::cartesian_16 => {
                let number = parse_cartesian_complex_number(inner_pair, radix)?;
                return Ok(if let Some(true) = exact {
                    Number::from(inexact_complex_to_exact_complex(number)?)
                } else {
                    Number::from(number)
                });
            }
            Rule::decimal_10 => return Ok(parse_decimal_number(inner_pair, negative)?.into()),
            Rule::infnan => {
                return if let Some(true) = exact {
                    Err(ErrorKind::ParseValue {
                        kind: "exact".to_string(),
                        value: inner_pair.as_str().to_string(),
                    }
                    .into())
                } else {
                    Ok(InexactReal::from(InfNan::from_str(inner_pair.as_str())?).into())
                }
            }
            Rule::uratio_2 | Rule::uratio_8 | Rule::uratio_10 | Rule::uratio_16 => {
                let number = parse_rational_number(inner_pair, radix, negative)?;
                return Ok(if let Some(false) = exact {
                    Number::from(rational_to_inexact_real(number)?)
                } else {
                    Number::from(number)
                });
            }
            Rule::uinteger_2 | Rule::uinteger_8 | Rule::uinteger_10 | Rule::uinteger_16 => {
                let number = parse_integer_number(inner_pair, radix, negative)?;
                return Ok(if let Some(false) = exact {
                    Number::from(integer_to_inexact_real(number)?)
                } else {
                    Number::from(number)
                });
            }
            _ => {
                unexpected_input!(inner_pair)
            }
        }
    }
    unreachable!()
}

fn parse_polar_complex_number(
    input_pair: Pair<'_, Rule>,
    radix: u32,
) -> Result<InexactComplex, Error> {
    let mut inner_pairs = input_pair.into_inner();
    let next_pair = inner_pairs.next().unwrap();
    let (negative, next_pair) = if next_pair.as_rule() == Rule::sign {
        (
            next_pair.as_str() == SIGN_NEGATIVE,
            inner_pairs.next().unwrap(),
        )
    } else {
        (false, next_pair)
    };
    let r = parse_real_number(next_pair, radix, negative)?;

    let p = inner_pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::polar_sym);

    let next_pair = inner_pairs.next().unwrap();
    let (negative, next_pair) = if next_pair.as_rule() == Rule::sign {
        (
            next_pair.as_str() == SIGN_NEGATIVE,
            inner_pairs.next().unwrap(),
        )
    } else {
        (false, next_pair)
    };
    let t = parse_real_number(next_pair, radix, negative)?;

    Ok(Complex::from_polar(
        InexactReal::try_from(exact_to_inexact(r)?)?,
        InexactReal::try_from(exact_to_inexact(t)?)?,
    )
    .into())
}

fn parse_cartesian_complex_number(
    input_pair: Pair<'_, Rule>,
    radix: u32,
) -> Result<InexactComplex, Error> {
    let mut real: Number = Integer::zero().into();
    let mut negative = false;
    let mut imag: Number = Integer::zero().into();

    let mut state = 0;
    for inner_pair in input_pair.into_inner() {
        //
        //  [([+-] {real} | infnan)] ([+-] {real} | infnan) "i"
        //   ↑     ↑        ↑        ↑     ↑        ↑        ↑
        //   0→1   0|1→2    0→2      2→3   2|3→4    2→4      2|4→5
        //
        match (state, inner_pair.as_rule()) {
            (0, Rule::sign) => {
                negative = inner_pair.as_str() == SIGN_NEGATIVE;
                state = 1;
            }
            (
                0 | 1,
                Rule::uratio_2
                | Rule::uratio_8
                | Rule::uratio_10
                | Rule::uratio_16
                | Rule::uinteger_2
                | Rule::uinteger_8
                | Rule::uinteger_10
                | Rule::uinteger_16
                | Rule::decimal_10,
            ) => {
                real = parse_real_number(inner_pair, radix, negative)?;
                state = 2;
            }
            (0, Rule::infnan) => {
                real = InfNan::from_str(inner_pair.as_str())
                    .map(|v| InexactReal::from(v))
                    .map(Number::from)?;
                state = 2;
            }
            (2, Rule::sign) => {
                negative = inner_pair.as_str() == SIGN_NEGATIVE;
                state = 3;
            }
            (
                2 | 3,
                Rule::uratio_2
                | Rule::uratio_8
                | Rule::uratio_10
                | Rule::uratio_16
                | Rule::uinteger_2
                | Rule::uinteger_8
                | Rule::uinteger_10
                | Rule::uinteger_16
                | Rule::decimal_10,
            ) => {
                imag = parse_real_number(inner_pair, radix, negative)?;
                state = 4;
            }
            (2, Rule::infnan) => {
                imag = InfNan::from_str(inner_pair.as_str())
                    .map(|v| InexactReal::from(v))
                    .map(Number::from)?;
                state = 4;
            }
            (2 | 4, Rule::i) => {
                negative = false;
            }
            _ => unexpected_input!(inner_pair, state),
        }
    }

    // TODO: Allow rational and integer values.

    if state == 2 {
        Ok(InexactComplex::new(
            InexactReal::zero(),
            InexactReal::try_from(real)?,
        ))
    } else {
        Ok(InexactComplex::new(
            InexactReal::try_from(real)?,
            InexactReal::try_from(imag)?,
        ))
    }
}

fn parse_real_number(
    input_pair: Pair<'_, Rule>,
    radix: u32,
    negative: bool,
) -> Result<Number, Error> {
    match input_pair.as_rule() {
        Rule::uratio_2 | Rule::uratio_8 | Rule::uratio_10 | Rule::uratio_16 => {
            parse_rational_number(input_pair, radix, negative).map(Number::from)
        }
        Rule::uinteger_2 | Rule::uinteger_8 | Rule::uinteger_10 | Rule::uinteger_16 => {
            parse_integer_number(input_pair, radix, negative).map(Number::from)
        }
        Rule::decimal_10 => parse_decimal_number(input_pair, negative).map(Number::from),
        Rule::infnan => InfNan::from_str(input_pair.as_str())
            .map(|v| InexactReal::from(v))
            .map(Number::from),
        _ => unexpected_input!(input_pair),
    }
}

fn parse_rational_number(
    input_pair: Pair<'_, Rule>,
    radix: u32,
    negative: bool,
) -> Result<Rational, Error> {
    let mut inner_pairs = input_pair.into_inner();
    let next_pair = inner_pairs.next().unwrap();
    let n = parse_integer_number(next_pair, radix, negative)?;

    let p = inner_pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::fraction_slash);

    let next_pair = inner_pairs.next().unwrap();
    let d = parse_integer_number(next_pair, radix, false)?;

    Ok(Rational::new(n, d))
}

fn parse_integer_number(
    input_pair: Pair<'_, Rule>,
    radix: u32,
    negative: bool,
) -> Result<Integer, Error> {
    Integer::from_str_radix(
        &format!(
            "{}{}",
            if negative { "-" } else { "+" },
            input_pair.as_str()
        ),
        radix,
    )
    .map_err(|e| {
        Error::chain(
            Box::new(e),
            ErrorKind::ParseValue {
                kind: TYPE_NAME_INTEGER.to_string(),
                value: input_pair.as_str().to_string(),
            },
        )
    })
}

fn parse_decimal_number(input_pair: Pair<'_, Rule>, negative: bool) -> Result<ExactReal, Error> {
    let mut pre = String::from("0");
    let mut post = String::from("0");
    let mut exp = String::from("");

    let mut state = 0;
    for inner_pair in input_pair.into_inner() {
        match (state, inner_pair.as_rule()) {
            (0, Rule::uinteger_10) => {
                pre = inner_pair.as_str().to_string();
                state = 1;
            }
            (0 | 1, Rule::period) => {
                state = 2;
            }
            (2, Rule::uinteger_10) => {
                post = inner_pair.as_str().to_string();
                state = 3;
            }
            (_, Rule::suffix) => {
                exp = inner_pair.as_str().to_string();
            }
            _ => unexpected_input!(inner_pair, state),
        }
    }

    let decimal_str = format!("{}{}.{}{}", if negative { "-" } else { "" }, pre, post, exp);
    if exp.is_empty() {
        ExactReal::from_str(&decimal_str)
    } else {
        ExactReal::from_scientific(&decimal_str)
    }
    .map_err(|e| {
        Error::chain(
            Box::new(e),
            ErrorKind::ParseValue {
                kind: TYPE_NAME_EXACT_REAL.to_string(),
                value: decimal_str,
            },
        )
    })
}

fn parse_instruction_data(input_pair: Pair<'_, Rule>) -> Result<Vec<Instruction>, Error> {
    println!(
        "parse_instruction_data(<{:?},{:?}>)",
        input_pair.as_rule(),
        input_pair.as_str()
    );
    let mut instructions: Vec<Instruction> = Default::default();
    match input_pair.as_rule() {
        Rule::instruction_data => {
            for inner_pair in input_pair.into_inner() {
                println!(
                    "parse_instruction_data => (<{:?},{:?}>)",
                    inner_pair.as_rule(),
                    inner_pair.as_str()
                );
                match inner_pair.as_rule() {
                    Rule::simple_instruction => {
                        instructions.push(match inner_pair.as_str() {
                            "NIL" => Instruction::Nil,
                            "AP" => Instruction::Apply,
                            "RTN" => Instruction::Return,
                            "DUM" => Instruction::Dummy,
                            "RAP" => Instruction::RecursiveApply,
                            "ADD" => Instruction::Add,
                            "SUB" => Instruction::Sub,
                            "MUL" => Instruction::Mul,
                            "DIV" => Instruction::Div,
                            "MOD" => Instruction::Mod,
                            "EQ" => Instruction::Equal,
                            "NEQ" => Instruction::NotEqual,
                            "LT" => Instruction::LessThan,
                            "LEQ" => Instruction::LessOrEqual,
                            "GT" => Instruction::GreaterThan,
                            "GEQ" => Instruction::GreaterOrEqual,
                            "CONS" => Instruction::Cons,
                            "CAR" => Instruction::Car,
                            "CDR" => Instruction::Cdr,
                            "ATOM" => Instruction::IsAtom,
                            "NULL" => Instruction::IsNull,
                            "SEL" => Instruction::Select,
                            "JOIN" => Instruction::Join,
                            "STOP" => Instruction::Stop,
                            _ => unexpected_input!(inner_pair),
                        });
                    }
                    Rule::load_instruction => {
                        if let Some(symbol_pair) = inner_pair.into_inner().next() {
                            instructions.push(Instruction::Load(Identifier::from_str_unchecked(
                                symbol_pair.as_str(),
                            )));
                        } else {
                            unimplemented!()
                        }
                    }
                    Rule::load_constant_instruction => {
                        instructions.push(Instruction::LoadConstant(parse_datum(
                            inner_pair.into_inner().next().unwrap(),
                        )?))
                    }
                    Rule::load_function_instruction => {}
                    Rule::EOI => {}
                    _ => unexpected_input!(inner_pair),
                }
            }
        }
        _ => unexpected_input!(input_pair),
    }
    Ok(instructions)
}

fn parse_maybe_datum(input_pair: Pair<'_, Rule>) -> Result<Option<Datum>, Error> {
    match input_pair.as_rule() {
        Rule::datum => Ok(Some(parse_datum_inner(input_pair)?)),
        Rule::EOI => Ok(None),
        _ => unexpected_input!(input_pair),
    }
}

fn parse_datum(input_pair: Pair<'_, Rule>) -> Result<Datum, Error> {
    match input_pair.as_rule() {
        Rule::datum => parse_datum_inner(input_pair),
        _ => unexpected_input!(input_pair),
    }
}

fn parse_datum_inner(input_pair: Pair<'_, Rule>) -> Result<Datum, Error> {
    let mut inner_pairs = input_pair.into_inner();
    let input_pair = inner_pairs.next().unwrap();
    let datum: Datum = match input_pair.as_rule() {
        Rule::symbol => {
            let symbol = Identifier::from_str_unchecked(input_pair.as_str());
            symbol.into()
        }
        Rule::boolean => string_to_boolean(input_pair.as_str())?.into(),
        Rule::number => parse_number(input_pair)?.simplify().into(),
        Rule::character => string_to_char(input_pair.as_str())?.into(),
        Rule::string => SchemeString::from_str(input_pair.as_str())?.into(),
        Rule::pair => parse_pair(input_pair)?.into(),
        Rule::list => parse_list(input_pair)?.into(),
        Rule::vector => parse_vector(input_pair)?.into(),
        Rule::abbreviation => parse_abbreviation(input_pair)?.into(),
        _ => unexpected_input!(input_pair),
    };
    assert!(inner_pairs.next().is_none());
    Ok(datum)
}

fn parse_list(input_pair: Pair<'_, Rule>) -> Result<Datum, Error> {
    let mut list_data: Vec<Datum> = Vec::default();
    for inner_pair in input_pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::datum => list_data.push(parse_datum_inner(inner_pair)?),
            _ => unexpected_input!(inner_pair),
        }
    }
    Ok(Datum::List(vector_to_list(Vector::from(list_data))))
}

fn parse_pair(input_pair: Pair<'_, Rule>) -> Result<DatumPair, Error> {
    let mut data = Vec::new();
    for next_pair in input_pair.into_inner() {
        if next_pair.as_rule() == Rule::datum {
            data.push(parse_datum_inner(next_pair)?)
        } else {
            unexpected_input!(next_pair);
        }
    }

    let cdr = data.remove(data.len() - 1);
    let car = data.remove(data.len() - 1);
    let mut head = DatumPair::cons(car.into(), cdr.into());

    for datum in data.into_iter().rev() {
        head = DatumPair::cons_list(Ref::new(datum), head);
    }
    Ok(head)
}

fn parse_vector(input_pair: Pair<'_, Rule>) -> Result<Datum, Error> {
    let mut vector: Vec<Ref<Datum>> = Vec::default();
    for inner_pair in input_pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::datum => vector.push(Ref::new(parse_datum_inner(inner_pair)?.into())),
            _ => unexpected_input!(inner_pair),
        }
    }
    Ok(Datum::Vector(Vector::from(vector)))
}

fn parse_abbreviation(input_pair: Pair<'_, Rule>) -> Result<Datum, Error> {
    let mut inner_pairs = input_pair.into_inner();
    let input_pair = inner_pairs.next().unwrap();
    let abbreviation = match input_pair.as_rule() {
        Rule::abbrev_prefix => Abbreviation::from_str(input_pair.as_str())?,
        _ => unexpected_input!(input_pair),
    };
    let input_pair = inner_pairs.next().unwrap();
    let datum = match input_pair.as_rule() {
        Rule::datum => parse_datum_inner(input_pair)?,
        _ => unexpected_input!(input_pair),
    };
    Ok(Datum::Abbreviation(abbreviation, Ref::new(datum)))
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------
