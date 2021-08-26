use schemer_lang::read::datum::Datum;
use schemer_lang::types::{Identifier, InexactReal, InfNan, Integer, Number, SchemeRepr};
use schemer_parse::parser::parse_datum_str;
use std::str::FromStr;

fn assert_parsed_ok(src: &str) {
    println!("<< {}", src);
    let result = parse_datum_str(src);
    match result {
        Ok(datum) => {
            println!(">> {:?}", datum);
            println!(">> {}", datum.to_repr_string());
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!("{:#?}", e);
        }
    }
}

fn assert_parsed_eq(src: &str, expected: Datum) {
    println!("{}", src);
    let result = parse_datum_str(src);
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
fn test_pairs() {
    assert_parsed_ok("(2 . 3)");
}

#[test]
fn test_simple_list() {
    assert_parsed_ok("(1 (list) (2 . 3) (4.0))");
}

#[test]
fn test_string() {
    assert_parsed_eq("\"hello\"", Datum::from("hello").into());
}

#[test]
fn test_identifiers() {
    assert_parsed_eq("+", Identifier::from_str("+").unwrap().into());
    assert_parsed_eq("-", Identifier::from_str("-").unwrap().into());
    assert_parsed_eq("add", Identifier::from_str("add").unwrap().into());
    assert_parsed_eq("add-1", Identifier::from_str("add-1").unwrap().into());
    assert_parsed_eq(":add", Identifier::from_str(":add").unwrap().into());
    assert_parsed_eq("add?", Identifier::from_str("add?").unwrap().into());
    assert_parsed_eq("add!", Identifier::from_str("add!").unwrap().into());
    assert_parsed_eq(
        "|add a number|",
        Identifier::from_str("|add a number|").unwrap().into(),
    );
    //assert_parsed_ok("∆");
}

#[test]
fn test_simple_fn() {
    assert_parsed_ok("(define (add x y) (+ x y))");
}

#[test]
fn test_num_uinteger_10() {
    assert_parsed_eq("3", Number::from(Integer::from(3)).into());
    assert_parsed_eq("#d3", Number::from(Integer::from(3)).into());
    assert_parsed_eq("#e3", Number::from(Integer::from(3)).into());
    assert_parsed_eq("#d#e3", Number::from(Integer::from(3)).into());
    assert_parsed_eq("#e#d3", Number::from(Integer::from(3)).into());
}

#[test]
fn test_num_integer_10() {
    assert_parsed_eq("-3", Number::from(Integer::from(-3)).into());

    assert_parsed_eq("+3", Number::from(Integer::from(3)).into());
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
    assert_parsed_ok("#x11/32");
    assert_parsed_ok("#xA0/B0B");
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
        Number::from(InexactReal::from(InfNan::PositiveInfinity)).into(),
    );
    assert_parsed_eq(
        "-inf.0",
        Number::from(InexactReal::from(InfNan::NegativeInfinity)).into(),
    );
    let result = parse_datum_str("+nan.0");
    match result {
        Ok(datum) => {
            if let Datum::Number(Number::InexactReal(v)) = datum {
                assert!(v.is_nan());
                assert!(v.is_sign_positive());
            } else {
                panic!("{:#?}", datum);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!("{:#?}", e);
        }
    }
    let result = parse_datum_str("-nan.0");
    match result {
        Ok(datum) => {
            if let Datum::Number(Number::InexactReal(v)) = datum {
                assert!(v.is_nan());
                assert!(v.is_sign_negative());
            } else {
                panic!("{:#?}", datum);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!("{:#?}", e);
        }
    }
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
    assert_parsed_ok("1/2+2/3i");
    assert_parsed_ok("#xA0/B0B+1i");
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

#[cfg(feature = "char-names")]
#[test]
fn test_char_unicode_name() {
    assert_parsed_ok("#\\black_star");
}
