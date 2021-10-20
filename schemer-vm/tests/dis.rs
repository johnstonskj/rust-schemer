use pretty_assertions::assert_eq;
use schemer_lang::read::datum::Datum;
use schemer_lang::types::Number;
use schemer_vm::error::ErrorKind;
use schemer_vm::file::asm::assemble_into;
use schemer_vm::file::dis::disassemble_from;
use schemer_vm::machine::datum::DatumType;
use schemer_vm::machine::instructions::InstructionType;
use schemer_vm::machine::Instruction;

#[test]
fn test_dis_nil() {
    let result = disassemble_from(&[InstructionType::Nil as u8]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [Instruction::Nil].to_vec());
}

#[test]
fn test_dis_stop() {
    let result = disassemble_from(&[InstructionType::Stop as u8]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [Instruction::Stop].to_vec());
}

#[test]
fn test_dis_bad_instruction() {
    let result = disassemble_from(&[0xFE]);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), &ErrorKind::Format)
}

#[test]
fn test_dis_add() {
    let result = disassemble_from(&[
        InstructionType::LoadConstant as u8,
        DatumType::Integer as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        InstructionType::LoadConstant as u8,
        DatumType::Integer as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        2,
        InstructionType::Add as u8,
    ]);

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
