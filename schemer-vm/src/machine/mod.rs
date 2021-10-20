/*!
One-line description.

More detailed description, with

# Example

*/

use schemer_lang::read::datum::Datum;
use std::collections::{BTreeMap, LinkedList};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct AbstractMachine {
    stack: LinkedList<Datum>,
    environment: LinkedList<BTreeMap<String, Datum>>,
    memory: Vec<u8>,
    memory_pointer: usize,
    dump: LinkedList<MachineState>,
    ticks: u64,
    running: bool,
}
#[derive(Debug)]
pub struct MachineState {
    stack: LinkedList<Datum>,
    environment: LinkedList<BTreeMap<String, Datum>>,
    memory_pointer: usize,
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod datum;
pub use datum::DatumType;

pub mod memory;
pub use memory::Memory;

pub mod instructions;
pub use instructions::Instruction;

mod execute;
