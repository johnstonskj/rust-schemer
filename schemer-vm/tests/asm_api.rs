use pretty_assertions::assert_eq;
use schemer_lang::read::datum::Datum;
use schemer_lang::types::Number;
use schemer_vm::file::asm::assemble_into;
use schemer_vm::machine::datum::DatumType;
use schemer_vm::machine::instructions::InstructionType;
use schemer_vm::machine::Instruction;

#[test]
fn test_asm_nil() {
    let result = assemble_into(&[Instruction::Nil]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [InstructionType::Nil as u8].to_vec());
}

#[test]
fn test_asm_stop() {
    let result = assemble_into(&[Instruction::Stop]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [InstructionType::Stop as u8].to_vec());
}

#[test]
fn test_asm_add() {
    let result = assemble_into(&[
        Instruction::LoadConstant(Datum::Number(Number::Integer(1.into()))),
        Instruction::LoadConstant(Datum::Number(Number::Integer(2.into()))),
        Instruction::Add,
    ]);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        [
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
        ]
        .to_vec()
    );
}
