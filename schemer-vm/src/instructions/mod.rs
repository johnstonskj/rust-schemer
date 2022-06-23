/*!
One-line description.

More detailed description, with

# Example

 */

use crate::error::{Error, ErrorKind};
use crate::machine::Cell;
use crate::machine::Machine;
use schemer_lang::read::datum::Datum;
use schemer_lang::types::Identifier;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::sync::RwLock;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type OpCode = u8;

pub type OpName = &'static str;

pub const MACHINE_CONTINUE: bool = true;
pub const MACHINE_HALT: bool = false;

#[derive(Clone)]
struct InstructionDefinition {
    op_code: OpCode,
    op_name: OpName,
    stack_min: usize,
    exec_fn: &'static dyn Fn(Machine, Option<Cell>) -> Result<bool, Error>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const INSTRUCTION_TABLE: &[InstructionDefinition] = &[InstructionDefinition {
    op_code: 0,
    op_name: "NIL",
    stack_min: 0,
    exec_fn: &do_nil,
}];

lazy_static! {
    static ref INSTRUCTIONS: RwLock<Vec<InstructionDefinition>> = Default::default();
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn register_operation(
    op_code: OpCode,
    op_name: OpName,
    stack_min: usize,
    exec_fn: &'static dyn Fn(&mut dyn Machine, Option<Cell>) -> Result<bool, Error>,
) -> Result<InstructionDefinition, Error> {
    let mut instructions = INSTRUCTIONS.write().unwrap();
    if instructions
        .iter()
        .find(|instruction| instruction.op_code == op_code || instruction.op_name == op_name)
        .is_some()
    {
        Err(ErrorKind::InvalidOperationRegistration.into())
    } else {
        instructions.push(InstructionDefinition {
            op_code,
            op_name,
            stack_min,
            exec_fn,
        });
        Ok(Instruction(instructions.len() - 1))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

unsafe impl Send for InstructionDefinition {}

unsafe impl Sync for InstructionDefinition {}

// ------------------------------------------------------------------------------------------------

impl Display for InstructionDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.op_name(), self.op_code())
    }
}

impl TryFrom<u8> for InstructionDefinition {
    type Error = Error;

    fn try_from(op_code: u8) -> Result<&Self, Self::Error> {
        INSTRUCTION_TABLE
            .iter()
            .find_map(|instruction| {
                if instruction.op_code == op_code {
                    Some(instruction)
                } else {
                    None
                }
            })
            .ok_or(ErrorKind::InvalidOpCode(op_code).into())
    }
}

impl TryFrom<&str> for InstructionDefinition {
    type Error = Error;

    fn try_from(op_name: &str) -> Result<&Self, Self::Error> {
        INSTRUCTION_TABLE
            .iter()
            .find_map(|instruction| {
                if instruction.op_name == op_name {
                    Some(instruction)
                } else {
                    None
                }
            })
            .ok_or(ErrorKind::InvalidOpName(op_name.to_string()).into())
    }
}

impl InstructionDefinition {
    pub fn op_code(&self) -> u8 {
        self.op_code
    }

    pub fn op_name(&self) -> &'static str {
        self.op_name
    }

    pub fn minimum_stack_required(&self) -> usize {
        self.stack_min
    }

    pub fn execute(&self, machine: &mut dyn Machine) -> Result<bool, Error> {
        self.exec_fn(machine, None)
    }

    pub fn execute_with_datum(
        &self,
        machine: &mut dyn Machine,
        datum: Datum,
    ) -> Result<bool, Error> {
        self.exec_fn(machine, Some(Cell::Datum(datum)))
    }

    pub fn execute_with_identifier(
        &self,
        machine: &mut dyn Machine,
        id: Identifier,
    ) -> Result<bool, Error> {
        self.exec_fn(machine, Some(Cell::Datum(Datum::Symbol(id))))
    }

    pub fn execute_with_closure(
        &self,
        machine: &mut dyn Machine,
        args: Vec<Identifier>,
        body: Vec<InstructionDefinition>,
    ) -> Result<bool, Error> {
        self.exec_fn(machine, Some(Cell::Closure(args, body)))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
