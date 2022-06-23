/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::instructions::Instruction;
use schemer_lang::read::datum::Datum;
use schemer_lang::types::{Identifier, SchemeRepr};
use std::fmt::{Debug, Write};
use std::ops::RangeFull;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Cell {
    Datum(Datum),
    Closure(Vec<Identifier>, Vec<Instruction>),
}

#[derive(Clone, Debug)]
pub struct Stack(Vec<Cell>);

#[derive(Clone, Debug)]
pub struct Environment(Vec<Vec<Cell>>);

#[derive(Clone, Debug)]
pub struct Code(Vec<Instruction>);

#[derive(Clone, Debug)]
pub struct Dump(Vec<DumpFrame>);

#[derive(Clone, Debug)]
pub struct DumpFrame {
    stack: Stack,
    environment: Environment,
    code_ptr: usize,
}

#[derive(Clone, Debug)]
pub struct Machine {
    stack: Stack,
    environment: Environment,
    code: Code,
    code_ptr: usize,
    dump: Dump,
}

pub trait WriteState {
    fn write_to<W: Write>(&self, range: RangeFull, w: &mut W) -> Result<(), Error>;
    fn write(&self, range: RangeFull) -> Result<(), Error> {
        self.write_to(range, &mut std::io::stdout())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Stack {
    fn default() -> Self {
        Self {
            0: Default::default(),
        }
    }
}

impl WriteState for Stack {
    fn write_to<W: Write>(&self, range: RangeFull, w: &mut W) -> Result<(), Error> {
        writeln!(w, "Stack:")?;
        for (idx, cell) in self.current_state.stack.iter().rev().enumerate() {
            writeln!(
                w,
                "{:>02}: {}",
                idx,
                match cell {
                    Cell::Datum(v) => format!("{} ({})", v.to_repr_string(), v.inner_type_name()),
                    Cell::Closure(_, _) => cell_type_name(cell),
                }
            )?;
        }
        Ok(())
    }
}

impl Stack {
    pub fn push(&mut self, cell: Cell) {
        self.0.push(cell)
    }

    pub fn pop(&mut self) -> Option<Cell> {
        self.0.pop()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Environment {
    fn default() -> Self {
        Self {
            0: Default::default(),
        }
    }
}

impl WriteState for Environment {
    fn write_to<W: Write>(&self, range: RangeFull, w: &mut W) -> Result<(), Error> {
        writeln!(w, "Environment:")?;
        for (idx, state) in self.dump[..max_depth].iter().rev().enumerate() {
            writeln!(w, "{:>04}:-----", idx,)?;
            for (idx, cell) in state.environment.iter().rev().enumerate() {
                writeln!(w, "     {:>04}: {:?}", idx, cell,)?;
            }
        }
        Ok(())
    }
}

impl Environment {
    pub fn push_empty(&mut self) {
        self.0.push(Default::default())
    }

    pub fn push(&mut self, row: Vec<Cell>) {
        self.0.push(row)
    }

    pub fn pop(&mut self) -> Option<Vec<Cell>> {
        self.0.pop()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Code {
    fn default() -> Self {
        Self {
            0: Default::default(),
        }
    }
}

impl WriteState for Code {
    fn write_to<W: Write>(&self, range: RangeFull, w: &mut W) -> Result<(), Error> {
        writeln!(w, "Code:")?;
        for (idx, instruction) in self.memory[range].iter().enumerate() {
            writeln!(
                w,
                "{} {:>04}: {}",
                if idx == self.current_state.code_ptr {
                    "*"
                } else {
                    " "
                },
                idx,
                instruction
            )?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Dump {
    fn default() -> Self {
        Self {
            0: Default::default(),
        }
    }
}

impl WriteState for Dump {
    fn write_to<W: Write>(&self, range: RangeFull, w: &mut W) -> Result<(), Error> {
        todo!()
    }
}

impl Dump {
    pub fn push(&mut self, stack: Stack, environment: Environment, code_ptr: usize) {
        self.0.push(DumpFrame {
            stack,
            environment,
            code_ptr,
        })
    }

    pub fn pop(&mut self) -> Option<DumpFrame> {
        self.0.pop()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }
}

// ------------------------------------------------------------------------------------------------

impl Machine {
    pub fn new(code: Code) -> Result<Self, Error> {
        Self::new_with_start(code, 0)
    }

    pub fn new_with_start(code: Code, start: usize) -> Result<Self, Error> {
        Ok(Self {
            stack: Default::default(),
            environment: Default::default(),
            code,
            code_ptr: start,
            dump: Default::default(),
        })
    }

    pub fn run_to_completion(&mut self) -> Result<(), Error> {
        while self.step()? {}
        Ok(())
    }

    fn step(&mut self) -> Result<bool, Error> {
        todo!()
    }

    fn stack_push(&mut self, cell: Cell) -> Result<(), Error> {
        self.stack.push(cell);
        Ok(())
    }

    fn stack_pop(&mut self) -> Result<Cell, Error> {
        self.stack
            .pop()
            .ok_or(ErrorKind::InsufficientStack(self.stack.depth()).into())
    }

    fn environment_push(&mut self, row: Vec<Cell>) -> Result<(), Error> {
        self.environment.push(row);
        Ok(())
    }

    fn environment_push_dummy(&mut self) -> Result<(), Error> {
        self.environment.push_empty();
        Ok(())
    }

    fn dump_state(&mut self) -> Result<(), Error> {
        self.dump
            .push(self.stack.clone(), self.environment.clone(), self.code_ptr);
        self.stack = Default::default();
        self.environment = Default::default();
        Ok(())
    }

    fn restore_state(&mut self, row: Vec<Cell>) -> Result<(), Error> {
        let old_state = self.dump.pop().unwrap();
        self.stack = old_state.stack;
        self.environment = old_state.environment;
        self.code_ptr = old_state.code_ptr;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod datum;
pub use datum::DatumType;

pub mod instructions;

mod execute;
pub use execute::make_machine;
