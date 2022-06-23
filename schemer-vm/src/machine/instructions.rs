/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::instructions::C;
use crate::instructions::{register_operation, Cell, MACHINE_CONTINUE, MACHINE_HALT};
use crate::machine::Machine;
use schemer_lang::read::datum::Datum;
use schemer_lang::types::{Identifier, Number, SchemeRepr};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    /*
        ========== Load Operations ==========
    */
    /// (s, e, NIL.c, d) => (nil.s, e, c, d)
    Nil,
    /// (s, e, LDC.v.c^, d) => (v.s, e, c, d)
    LoadConstant(Datum),
    /// (v1.s, e, LD.v1.c^, d) => (v2.s, e, c, d)
    Load(usize, usize),
    /// (s, e, LDF.(args body).c^, d) => (((args body).e).s, e, c, d)
    LoadFunction(Vec<Identifier>, Vec<Instruction>),
    /*
        ========== Function Application ==========
    */
    /// (((argnames body).e').argvals.s, e, AP.c, d)
    ///     => ( (), new-frame(argnames, argvals).e', body, s.e.c.d)
    Apply,
    /// (v.(), e', RTN.(), s.e.c.d) => (v.s, e, c, d)
    Return,
    /// (s, e, DUM.c, d) => (s, Ω.e, c, d)
    Dummy,
    /// (((args body).(Ω.e')).closures.s, Ω.e, RAP.c, d)
    ///     => ((), set-car!(Ω.e', new-frame(args, closures)).e', body, s.e.c.d)
    RecursiveApply,
    /*
        ========== Mathematical Operations ==========
    */
    /// (v1.v2.s, e, ADD.c, d)  => (v.s, e, c, d)
    Add,
    /// (v1.v2.s, e, SUB.c, d)  => (v.s, e, c, d)
    Sub,
    /// (v1.v2.s, e, MUL.c, d)  => (v.s, e, c, d)
    Mul,
    /// (v1.v2.s, e, DIV.c, d)  => (v.s, e, c, d)
    Div,
    /// (v1.v2.s, e, REM.c, d)  => (v.s, e, c, d)
    Rem,
    /*
        ========== Comparison Operations ==========
    */
    /// (v1.v2.s, e, EQ.c, d)  => (v.s, e, c, d)
    Equal,
    /// (v1.v2.s, e, NEQ.c, d)  => (v.s, e, c, d)
    NotEqual,
    /// (v1.v2.s, e, LT.c, d)  => (v.s, e, c, d)
    LessThan,
    /// (v1.v2.s, e, LEQ.c, d)  => (v.s, e, c, d)
    LessOrEqual,
    /// (v1.v2.s, e, GT.c, d)  => (v.s, e, c, d)
    GreaterThan,
    /// (v1.v2.s, e, GEQ.c, d)  => (v.s, e, c, d)
    GreaterOrEqual,
    /*
        ========== List Operations ==========
    */
    /// (head.tail.s, e, CONS.c,  d)  => ((head.tail).s, e, c, d)
    /// `()`: (1.().s, e, CONS.c, d) => ( (1).s, e, c, d)
    Cons,
    /// ((head.tail).s, e, CAR.c, d) => (head.s, e, c, d)
    Car,
    /// ((head.tail).s, e, CDR.c, d) => (tail.s, e, c, d)
    Cdr,
    /// (v.s, e, ATOM.c, d) => (atom?(v).s, e, c, d)
    IsAtom,
    /// (v.s, e, NULL.c, d) => (null?(v).s, e, c, d)
    IsNull,
    /*
        ========== Selection Operations ==========
    */
    /// (#t.s, e, SEL.then.else.c, d) => (s, e, then, c.d)
    /// (#f.s, e, SEL.then.else.c, d) => (s, e, else, c.d)
    Select,

    Join,

    Stop,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum InstructionType {
    Nil = 0x00,
    LoadConstant = 0x01,
    Load = 0x02,
    LoadFunction = 0x03,
    Apply = 0x11,
    Return = 0x12,
    Dummy = 0x13,
    RecursiveApply = 0x14,
    Add = 0x21,
    Sub = 0x22,
    Mul = 0x23,
    Div = 0x24,
    Rem = 0x25,
    Equal = 0x31,
    NotEqual = 0x32,
    LessThan = 0x33,
    LessOrEqual = 0x34,
    GreaterThan = 0x35,
    GreaterOrEqual = 0x36,
    Cons = 0x41,
    Car = 0x42,
    Cdr = 0x43,
    IsAtom = 0x51,
    IsNull = 0x52,
    Select = 0x61,
    Join = 0x71,
    Stop = 0xFF,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn register_operations() -> Result<(), Error> {
    register_operation(0x00, "NIL", 0, &do_nil)?;
    register_operation(0x01, "LDC", 0, &do_load_constant)?;
    register_operation(0x02, "LD", 0, &do_load)?;
    register_operation(0x03, "LDF", 0, &do_load_function)?;
    register_operation(0x11, "AP", 0, &do_apply)?;
    register_operation(0x12, "RTN", 0, &do_return)?;
    register_operation(0x13, "DUM", 0, &do_dummy)?;
    register_operation(0x14, "RAP", 0, &do_recursive_apply)?;
    register_operation(0x21, "ADD", 0, &do_add)?;
    register_operation(0x22, "SUB", 0, &do_sub)?;
    register_operation(0x23, "MUL", 0, &do_mul)?;
    register_operation(0x24, "DIV", 0, &do_div)?;
    register_operation(0x25, "REM", 0, &do_rem)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instruction::Nil => "NIL".to_string(),
                Instruction::LoadConstant(v) => format!("LDC {}", v.to_repr_string()),
                Instruction::Load(depth, index) => format!("LD ({} {})", depth, index),
                Instruction::LoadFunction(args, body) => format!(
                    "LDF ({}) ({})",
                    args.iter()
                        .map(|id| id.to_repr_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    body.iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ),
                Instruction::Apply => "AP".to_string(),
                Instruction::Return => "RTN".to_string(),
                Instruction::Dummy => "DUM".to_string(),
                Instruction::RecursiveApply => "RAP".to_string(),
                Instruction::Add => "ADD".to_string(),
                Instruction::Sub => "SUB".to_string(),
                Instruction::Mul => "MUL".to_string(),
                Instruction::Div => "DIV".to_string(),
                Instruction::Rem => "REM".to_string(),
                Instruction::Equal => "EQ".to_string(),
                Instruction::NotEqual => "NEQ".to_string(),
                Instruction::LessThan => "LT".to_string(),
                Instruction::LessOrEqual => "LEQ".to_string(),
                Instruction::GreaterThan => "GT".to_string(),
                Instruction::GreaterOrEqual => "GEQ".to_string(),
                Instruction::Cons => "CONS".to_string(),
                Instruction::Car => "CAR".to_string(),
                Instruction::Cdr => "CDR".to_string(),
                Instruction::IsAtom => "ATOM".to_string(),
                Instruction::IsNull => "NULL".to_string(),
                Instruction::Select => "SEL".to_string(),
                Instruction::Join => "JOIN".to_string(),
                Instruction::Stop => "STOP".to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[instrument(level = "trace")]
fn stack_push(machine: &mut dyn Machine, data: Option<Cell>) {
    machine.current_state.stack.push(value);
}

#[instrument(level = "trace")]
fn stack_pop(machine: &mut dyn Machine, data: Option<Cell>) -> Cell {
    machine.current_state.stack.pop().unwrap()
}

#[instrument(level = "trace")]
fn do_nil(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // pushes a nil pointer onto the stack
    machine.stack_push(Cell::Datum(Datum::Null));
    Ok(MACHINE_CONTINUE)
}

#[instrument(level = "trace")]
fn do_load_constant(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // pushes a constant argument onto the stack
    machine.stack_push(Cell::Datum(value));
    Ok(MACHINE_CONTINUE)
}

#[instrument(level = "trace")]
fn do_load(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // pushes the value of a variable onto the stack. The variable is indicated by the argument,
    // a pair. The pair's car specifies the level, the cdr the position. So (1 . 3) gives the
    // current function's (level 1) third parameter
    if let Some(state) = machine.dump.get(machine.dump.len() - depth) {
        if let Some(value) = state.environment.get(index) {
            machine.stack_push(value.clone());
            return Ok(MACHINE_CONTINUE);
        }
    }
    Err(ErrorKind::InvalidEnvironmentIndex(depth, index).into())
}

#[instrument(level = "trace")]
fn do_load_function(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // takes one list argument representing a function. It constructs a closure (a pair
    // containing the function and the current environment) and pushes that onto the stack
    machine.stack_push(Cell::Closure(args, body));
    Ok(MACHINE_CONTINUE)
}

#[instrument(level = "trace")]
fn do_apply(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // pops a closure and a list of parameter values from the stack. The closure is applied to
    // the parameters by installing its environment as the current one, pushing the parameter
    // list in front of that, clearing the stack, and setting C to the closure's function
    // pointer. The previous values of S, E, and the next value of C are saved on the dump.
    todo!()
}

#[instrument(level = "trace")]
fn do_return(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // pops one return value from the stack, restores S, E, and C from the dump, and pushes the
    // return value onto the now-current stack
    let ret_value = machine.stack_pop();
    let _ = machine.current_state.pop();
    machine.stack_push(ret_value);
    Ok(MACHINE_CONTINUE)
}

#[instrument(level = "trace")]
fn do_dummy(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // pushes a "dummy", an empty list, in front of the environment list
    todo!()
}

#[instrument(level = "trace")]
fn do_recursive_apply(
    machine: &mut dyn Machine,
    data: Option<Cell>,
) -> Result<Option<Self>, Error> {
    // works like apply, only that it replaces an occurrence of a dummy environment with the
    // current one, thus making recursive functions possible
    todo!()
}

#[instrument(level = "trace")]
fn do_select(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // expects two list arguments, and pops a value from the stack. The first list is executed
    // if the popped value was non-nil, the second list otherwise. Before one of these list
    // pointers is made the new C, a pointer to the instruction following sel is saved on the
    // dump
    todo!()
}

#[instrument(level = "trace")]
fn do_join(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    // pops a list reference from the dump and makes this the new value of C. This instruction
    // occurs at the end of both alternatives of a sel.
    todo!()
}

#[instrument(level = "trace")]
fn do_numeric_binary_op(
    machine: &mut dyn Machine,
    op: impl Fn(Number, Number) -> Number,
) -> Result<bool, Error> {
    if machine.stack_depth() < 2 {
        Err(ErrorKind::InsufficientStack(2).into())
    } else {
        let rhs = machine.stack_pop();
        let rhs = if let Cell::Datum(Datum::Number(value)) = rhs {
            value
        } else {
            return Err(ErrorKind::TypeMismatch("Number".to_string(), cell_type_name(&rhs)).into());
        };
        let lhs = machine.stack_pop();
        let lhs = if let Cell::Datum(Datum::Number(value)) = lhs {
            value
        } else {
            return Err(ErrorKind::TypeMismatch("Number".to_string(), cell_type_name(&lhs)).into());
        };
        let result = op(lhs, rhs);
        machine.stack_push(Cell::Datum(Datum::Number(result)));
        Ok(MACHINE_CONTINUE)
    }
}

#[instrument(level = "trace")]
fn do_add(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    do_numeric_binary_op(machine, Number::add)
}

#[instrument(level = "trace")]
fn do_sub(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    do_numeric_binary_op(machine, Number::sub)
}

#[instrument(level = "trace")]
fn do_mul(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    do_numeric_binary_op(machine, Number::mul)
}

#[instrument(level = "trace")]
fn do_div(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    do_numeric_binary_op(machine, Number::div)
}

#[instrument(level = "trace")]
fn do_rem(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    do_numeric_binary_op(machine, Number::rem)
}

#[instrument(level = "trace")]
fn do_equal(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_not_equal(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_less_than(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_less_or_equal(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_greater_than(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_greater_or_equal(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_cons(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_car(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_cdr(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_is_atom(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

#[instrument(level = "trace")]
fn do_is_null(machine: &mut dyn Machine, data: Option<Cell>) -> Result<bool, Error> {
    todo!()
}

fn do_stop(_: &mut dyn Machine, _: Option<Cell>) -> Result<bool, Error> {
    Ok(MACHINE_HALT)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
