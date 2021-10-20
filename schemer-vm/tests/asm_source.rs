use pretty_assertions::assert_eq;
use schemer_lang::read::datum::Datum;
use schemer_lang::types::Number;
use schemer_vm::file::parser::parse_instructions_str;
use schemer_vm::machine::Instruction;

#[test]
fn test_parse_nil() {
    let result = parse_instructions_str("NIL");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [Instruction::Nil].to_vec())
}

#[test]
fn test_parse_stop() {
    let result = parse_instructions_str("STOP");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [Instruction::Stop].to_vec())
}

#[test]
fn test_parse_add() {
    let result = parse_instructions_str("LDC 1 LDC 2 ADD");

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        [
            Instruction::LoadConstant(Datum::Number(Number::Integer(1.into()))),
            Instruction::LoadConstant(Datum::Number(Number::Integer(2.into()))),
            Instruction::Add,
        ]
        .to_vec()
    );
}

#[test]
fn test_parse_bad_instruction() {
    let result = parse_instructions_str("HELLO");

    assert!(result.is_err());
}
