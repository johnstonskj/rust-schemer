/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::syntax::datum::Datum;
use crate::syntax::tokens::{string_from_str, Identifier, Token};
use crate::types::booleans::Boolean;
use crate::types::chars::Char;
use crate::types::numbers::{
    BaseFixed, BaseFloat, ExactComplex, ExactReal, InexactComplex, InexactReal, InfNan, Integer,
    Number, Rational, SchemeNum,
};
use num::complex::Complex;
use num::traits::{Num, Zero};
use pest::iterators::Pair;
use pest::Parser;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "syntax/simple.pest"]
struct SimpleSyntax;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse_token_str(source: &str) -> Result<Vec<Token>, Error> {
    let mut parsed = SimpleSyntax::parse(Rule::tokens, source)
        .map_err(|e| Error::chain(Box::new(e), ErrorKind::Parser))?;
    #[cfg(test)]
    println!(
        "{}",
        pest_ascii_tree::into_ascii_tree(parsed.clone())
            .map_err(|e| Error::chain(Box::new(e), ErrorKind::Parser))?
    );
    let pair = parsed.next().unwrap();
    parse_tokens(pair)
}

pub fn parse_datum_str(source: &str) -> Result<Datum, Error> {
    let mut parsed = SimpleSyntax::parse(Rule::datum, source)
        .map_err(|e| Error::chain(Box::new(e), ErrorKind::Parser))?;
    let pair = parsed.next().unwrap();
    parse_datum(pair)
}

pub fn parse_number_str(source: &str) -> Result<Number, Error> {
    let mut parsed = SimpleSyntax::parse(Rule::number, source)
        .map_err(|e| Error::chain(Box::new(e), ErrorKind::Parser))?;
    let pair = parsed.next().unwrap();
    parse_number(pair)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_tokens(input_pair: Pair<'_, Rule>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Default::default();
    match input_pair.as_rule() {
        Rule::tokens => {
            for inner_pair in input_pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::token => tokens.push(parse_token(inner_pair)?),
                    Rule::EOI => {}
                    _ => panic!("Unexpected input: {}", inner_pair),
                }
            }
        }
        _ => panic!("Unexpected input: {}", input_pair),
    }
    Ok(tokens)
}

fn parse_token(input_pair: Pair<'_, Rule>) -> Result<Token, Error> {
    match input_pair.as_rule() {
        Rule::token => {
            let span = input_pair.as_str();
            let mut inner_pairs = input_pair.into_inner();
            if let Some(inner_pair) = inner_pairs.next() {
                match inner_pair.as_rule() {
                    Rule::left_paren => Ok(Token::LeftParen),
                    Rule::right_paren => Ok(Token::RightParen),
                    Rule::left_vec => Ok(Token::LeftVector),
                    Rule::left_byte_vec => Ok(Token::LeftByteVector),
                    Rule::apostrophe => Ok(Token::Apostrophe),
                    Rule::back_tick => Ok(Token::BackTick),
                    Rule::comma => Ok(Token::Comma),
                    Rule::comma_at => Ok(Token::CommaAt),
                    Rule::period => Ok(Token::Dot),
                    Rule::identifier => {
                        let identifier = Identifier::from_str(inner_pair.as_str())?;
                        Ok(Token::Identifier(identifier))
                    }
                    Rule::boolean => Ok(Boolean::from_str(span)?.into()),
                    Rule::number => Ok(parse_number(inner_pair)?.into()),
                    Rule::character => Ok(Char::from_str(span)?.into()),
                    Rule::string => string_from_str(span),
                    _ => panic!("Unexpected input: {}", inner_pair),
                }
            } else {
                panic!("Missing input: {}", inner_pairs)
            }
        }
        _ => panic!("Unexpected input: {}", input_pair),
    }
}

fn parse_number(input_pair: Pair<'_, Rule>) -> Result<Number, Error> {
    let mut inner_pairs = input_pair.into_inner();
    let (radix, exact) = parse_number_prefix(inner_pairs.next().unwrap())?;

    let next_pair = inner_pairs.next().unwrap();
    let (negative, next_pair) = if next_pair.as_rule() == Rule::sign {
        (next_pair.as_str() == "-", inner_pairs.next().unwrap())
    } else {
        (false, next_pair)
    };

    let number: Number = match next_pair.as_rule() {
        Rule::polar_2 | Rule::polar_8 | Rule::polar_10 | Rule::polar_16 => {
            parse_polar_complex_number(next_pair, radix)?.into()
        }
        Rule::cartesian_2 | Rule::cartesian_8 | Rule::cartesian_10 | Rule::cartesian_16 => {
            parse_cartesian_complex_number(next_pair, radix)?.into()
        }
        Rule::uratio_2 | Rule::uratio_8 | Rule::uratio_10 | Rule::uratio_16 => {
            parse_rational_number(next_pair, radix, negative)?.into()
        }
        Rule::uinteger_2 | Rule::uinteger_8 | Rule::uinteger_10 | Rule::uinteger_16 => {
            parse_integer_number(next_pair, radix, negative)?.into()
        }
        Rule::decimal_10 => parse_decimal_number(next_pair, negative)?.into(),
        Rule::infnan => InexactReal::from(InfNan::from_str(next_pair.as_str())?).into(),
        _ => {
            panic!("Unexpected input: {}", next_pair)
        }
    };

    Ok(number)
}

fn parse_number_prefix(input_pair: Pair<'_, Rule>) -> Result<(u32, bool), Error> {
    fn is_exact(input_pair: Pair<'_, Rule>) -> bool {
        let mut inner_pairs = input_pair.into_inner();
        if let Some(inner_pair) = inner_pairs.next() {
            if inner_pair.as_rule() == Rule::exactness {
                return true;
            }
        }
        false
    }

    match input_pair.as_rule() {
        Rule::prefix_2 => Ok((2, is_exact(input_pair))),
        Rule::prefix_8 => Ok((8, is_exact(input_pair))),
        Rule::prefix_10 => Ok((10, is_exact(input_pair))),
        Rule::prefix_16 => Ok((16, is_exact(input_pair))),
        _ => panic!("Unexpected input: {}", input_pair),
    }
}

fn parse_polar_complex_number(
    input_pair: Pair<'_, Rule>,
    radix: u32,
) -> Result<InexactComplex, Error> {
    let mut inner_pairs = input_pair.into_inner();
    let next_pair = inner_pairs.next().unwrap();
    let (negative, next_pair) = if next_pair.as_rule() == Rule::sign {
        (next_pair.as_str() == "-", inner_pairs.next().unwrap())
    } else {
        (false, next_pair)
    };
    let r = parse_real_number(next_pair, radix, negative)?;
    assert!(!r.is_exact_complex() && !r.is_inexact_complex());

    let p = inner_pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::polar_sym);

    let next_pair = inner_pairs.next().unwrap();
    let (negative, next_pair) = if next_pair.as_rule() == Rule::sign {
        (next_pair.as_str() == "-", inner_pairs.next().unwrap())
    } else {
        (false, next_pair)
    };
    let t = parse_real_number(next_pair, radix, negative)?;
    assert!(!t.is_exact_complex() && !t.is_inexact_complex());

    println!("parse_polar_complex_number: {} {}", r, t);
    Ok(Complex::from_polar(r.into_inexact().into_inner(), r.into_inexact().into_inner()).into())
}

fn parse_cartesian_complex_number(input_pair: Pair<'_, Rule>, radix: u32) -> Result<Number, Error> {
    let mut real: Number = Integer::zero().into();
    let mut negative = false;
    let mut imag: Number = Integer::zero().into();

    let mut state = 0;
    for inner_pair in input_pair.into_inner() {
        match (state, inner_pair.as_rule()) {
            (
                0,
                Rule::uratio_2
                | Rule::uratio_8
                | Rule::uratio_10
                | Rule::uratio_16
                | Rule::uinteger_2
                | Rule::uinteger_8
                | Rule::uinteger_10
                | Rule::uinteger_16
                | Rule::decimal_10
                | Rule::infnan,
            ) => {
                real = parse_real_number(inner_pair, radix, false)?;
                state = 1;
            }
            (0, Rule::sign) => {
                negative = inner_pair.as_str() == "-";
                state = 3;
            }
            (1, Rule::infnan) => {
                imag = parse_real_number(inner_pair, radix, negative)?;
                state = 5;
            }
            (1, Rule::sign) => {
                negative = inner_pair.as_str() == "-";
                state = 2;
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
                | Rule::decimal_10
                | Rule::infnan,
            ) => {
                imag = parse_real_number(inner_pair, radix, negative)?;
                state = 5;
            }
            (5, Rule::i) => {}
            _ => panic!("Unexpected input: {} (state: {})", inner_pair, state),
        }
    }

    Ok(Complex::new(BaseFloat::from(real.int), BaseFloat::from(imag)).into())
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
        Rule::decimal_10 => parse_decimal_number(input_pair, negative),
        Rule::infnan => InfNan::from_str(input_pair.as_str()).map(|v| InexactReal::from(v).into()),
        _ => panic!("Unexpected input: {}", input_pair),
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
    println!("parse_rational_number: {} {}", n, p);

    Ok(Rational::new(n.into_inner(), d.into_inner()))
}

fn parse_integer_number(
    input_pair: Pair<'_, Rule>,
    radix: u32,
    negative: bool,
) -> Result<Integer, Error> {
    println!("parse_integer_number({:?})", input_pair);
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
            ErrorKind::Value {
                kind: "integer".to_string(),
                value: input_pair.as_str().to_string(),
            },
        )
    })
}

fn parse_decimal_number(input_pair: Pair<'_, Rule>, negative: bool) -> Result<BaseFixed, Error> {
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
            _ => panic!("Unexpected input: {} (state: {})", inner_pair, state),
        }
    }

    let decimal_str = format!("{}{}.{}{}", if negative { "-" } else { "" }, pre, post, exp);
    if exp.is_empty() {
        BaseFixed::from_str(&decimal_str)
    } else {
        BaseFixed::from_scientific(&decimal_str)
    }
    .map_err(|e| {
        Error::chain(
            Box::new(e),
            ErrorKind::Value {
                kind: "decimal".to_string(),
                value: decimal_str,
            },
        )
    })
}

fn parse_datum(_input_pair: Pair<'_, Rule>) -> Result<Datum, Error> {
    unimplemented!()
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parsed_ok(src: &str) {
        println!("{}", src);
        let result = parse_token_str(src);
        match result {
            Ok(tokens) => {
                println!("{:?}", tokens);
            }
            Err(e) => {
                eprintln!("{}", e);
                panic!("{:#?}", e);
            }
        }
    }

    fn assert_parsed_eq(src: &str, expected: Vec<Token>) {
        println!("{}", src);
        let result = parse_token_str(src);
        match result {
            Ok(tokens) => {
                assert_eq!(tokens, expected);
            }
            Err(e) => {
                eprintln!("{}", e);
                panic!("{:#?}", e);
            }
        }
    }

    #[test]
    fn test_simple_list() {
        assert_parsed_ok("(1 () (2 . 3) (4.0))");
    }

    #[test]
    fn test_string() {
        assert_parsed_eq("\"hello\"", vec![String::from("hello").into()]);
    }

    #[test]
    fn test_identifiers() {
        assert_parsed_eq("+", vec![Identifier::from_str("+").unwrap().into()]);
        assert_parsed_eq("-", vec![Identifier::from_str("-").unwrap().into()]);
        assert_parsed_eq("add", vec![Identifier::from_str("add").unwrap().into()]);
        assert_parsed_eq("add-1", vec![Identifier::from_str("add-1").unwrap().into()]);
        assert_parsed_eq(":add", vec![Identifier::from_str(":add").unwrap().into()]);
        assert_parsed_eq("add?", vec![Identifier::from_str("add?").unwrap().into()]);
        assert_parsed_eq("add!", vec![Identifier::from_str("add!").unwrap().into()]);
        assert_parsed_eq(
            "|add a number|",
            vec![Identifier::from_str("|add a number|").unwrap().into()],
        );
        //assert_parsed_ok("∆");
    }

    #[test]
    fn test_simple_fn() {
        assert_parsed_ok("(define (add x y) (+ x y))");
    }

    #[test]
    fn test_num_uinteger_10() {
        assert_parsed_eq(
            "3",
            vec![Token::Number(Number::from(Integer::from(3.into())))],
        );
        assert_parsed_eq(
            "#d3",
            vec![Token::Number(Number::from(Integer::from(3.into())))],
        );
        assert_parsed_eq(
            "#e3",
            vec![Token::Number(Number::from(Integer::from(3.into())))],
        );
        assert_parsed_eq(
            "#d#e3",
            vec![Token::Number(Number::from(Integer::from(3.into())))],
        );
        assert_parsed_eq(
            "#e#d3",
            vec![Token::Number(Number::from(Integer::from(3.into())))],
        );
    }

    #[test]
    fn test_num_integer_10() {
        assert_parsed_eq(
            "-3",
            vec![Token::Number(Number::from(Integer::from(-3.into())))],
        );
        assert_parsed_eq(
            "+3",
            vec![Token::Number(Number::from(Integer::from(3.into())))],
        );
    }

    #[test]
    fn test_num_rational_10() {
        assert_parsed_ok("1/3");
        assert_parsed_ok("-1/3");
        assert_parsed_ok("3⁄4");
        assert_parsed_ok("#e1/3");
    }

    #[test]
    fn test_num_rational_8() {
        assert_parsed_ok("#o1/3");
        assert_parsed_ok("#o-1/3");
        assert_parsed_ok("#o3⁄4");
        assert_parsed_ok("#o#e1/3");
    }

    #[test]
    fn test_num_rational_16() {
        assert_parsed_ok("#x1/3");
        assert_parsed_ok("#x-1/3");
        assert_parsed_ok("#x3⁄4");
        assert_parsed_ok("#x#e1/3");
    }

    #[test]
    fn test_num_decimal_10() {
        assert_parsed_ok("3.");
        assert_parsed_ok(".5");
        assert_parsed_ok("3.5");
        assert_parsed_ok("3.e2");
        assert_parsed_ok(".5e3");
        assert_parsed_ok("3.5e4");
        assert_parsed_ok("3e+2");
        assert_parsed_ok("3e-2");
        assert_parsed_ok("#e1e3");
    }

    #[test]
    fn test_num_polar_10() {
        assert_parsed_ok("1@3");
        assert_parsed_ok("-1@3");
        assert_parsed_ok("1@-3");
        assert_parsed_ok("1/3@3/4");
        assert_parsed_ok("+inf.0@3");
        assert_parsed_ok("1@+inf.0");
    }

    #[test]
    fn test_num_infnan() {
        assert_parsed_eq(
            "+inf.0",
            vec![Token::Number(Number::from(InexactReal::from(
                InfNan::PositiveInfinity,
            )))],
        );
        assert_parsed_eq(
            "-inf.0",
            vec![Token::Number(Number::from(InexactReal::from(
                InfNan::NegativeInfinity,
            )))],
        );
        assert_parsed_eq(
            "+nan.0",
            vec![Token::Number(Number::from(InexactReal::from(
                InfNan::PositiveNan,
            )))],
        );
        assert_parsed_eq(
            "-nan.0",
            vec![Token::Number(Number::from(InexactReal::from(
                InfNan::NegativeNan,
            )))],
        );
    }

    #[test]
    fn test_num_cartesian_10() {
        assert_parsed_ok("3+4i");
        assert_parsed_ok("-2.5+0i");
        assert_parsed_ok("-2.5+0.0i");
        assert_parsed_ok("3+0i");
        assert_parsed_ok("3.0+inf.0i");
        assert_parsed_ok("+nan.0+5.0i");
        assert_parsed_ok("1+2i");
    }

    #[test]
    fn test_boolean_short() {
        assert_parsed_ok("#t");
        assert_parsed_ok("#f");
    }

    #[test]
    fn test_boolean_long() {
        assert_parsed_ok("#true");
        assert_parsed_ok("#false");
    }

    #[test]
    fn test_char_single() {
        assert_parsed_ok("#\\A");
        assert_parsed_ok("#\\!");
    }

    #[test]
    fn test_char_spec_name() {
        assert_parsed_ok("#\\space");
        assert_parsed_ok("#\\alarm");
    }

    #[test]
    fn test_char_escaped() {
        assert_parsed_ok("#\\x2764");
    }

    #[test]
    fn test_char_unicode_name() {
        assert_parsed_ok("#\\black_star");
    }
}
