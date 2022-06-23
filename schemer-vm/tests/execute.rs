use schemer_lang::read::datum::Datum;
use schemer_lang::types::Number;
use schemer_vm::machine::Machine;
use schemer_vm::machine::{make_machine, Instruction};

#[test]
fn do_simple_add() {
    let code = [
        Instruction::LoadConstant(Datum::Number(Number::Integer(1))),
        Instruction::LoadConstant(Datum::Number(Number::Integer(2))),
        Instruction::Add,
        Instruction::Stop,
    ]
    .to_vec();
    let machine = make_machine(code).unwrap();
    println!("0: {:#?}", machine);
    machine.stack_dump();
    machine.instruction_dump(..);
    let machine = machine.step().unwrap().unwrap();
    println!("1: {:#?}", machine);
    machine.stack_dump();
    let machine = machine.step().unwrap().unwrap();
    println!("2: {:#?}", machine);
    machine.stack_dump();
    let machine = machine.step().unwrap().unwrap();
    machine.stack_dump();
    println!("3: {:#?}", machine);
    let machine = machine.step().unwrap();
    println!("4: {:#?}", machine);
}
