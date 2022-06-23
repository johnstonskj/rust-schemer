/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::machine::{DebuggableMachine, Instruction, Machine};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::{Identifier, Number, SchemeRepr};
use std::collections::BTreeMap;
use std::ops::{Add, Div, Mul, RangeFull, Rem, Sub};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

type Environment = Vec<Vec<Cell>>;

#[derive(Clone, Debug)]
struct SimpleMachine {
    memory: Vec<Instruction>,
    current_state: State,
    dump: Vec<State>,
}

#[derive(Clone, Debug)]
enum Cell {
    Datum(Datum),
    Closure(Vec<Identifier>, Vec<Instruction>),
}

#[derive(Clone, Debug)]
struct State {
    stack: Vec<Cell>,
    environment: Environment,
    code_ptr: usize,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn make_machine(memory: Vec<Instruction>) -> Result<impl Machine, Error> {
    Ok(SimpleMachine {
        memory,
        current_state: Default::default(),
        dump: Default::default(),
    })
}

pub fn make_machine_with_start(
    memory: Vec<Instruction>,
    start: usize,
) -> Result<impl Machine, Error> {
    Ok(SimpleMachine {
        memory,
        current_state: State {
            stack: vec![],
            environment: vec![],
            code_ptr: start,
        },
        dump: vec![],
    })
}

pub fn run_to_completion(mut machine: impl Machine) -> Result<(), Error> {
    while machine.step()? {}
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for State {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            environment: Default::default(),
            code_ptr: 0,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Machine for SimpleMachine {
    fn step(&mut self) -> Result<bool, Error> {
        if self.current_state.code_ptr >= self.memory.len() {
            Err(ErrorKind::InvalidCodePointer(self.current_state.code_ptr).into())
        } else {
            let next_instruction = self
                .memory
                .get(self.current_state.code_ptr)
                .unwrap()
                .clone();
            self.current_state.code_ptr += 1;
            match next_instruction {
                Instruction::Nil => self.do_nil(),
                Instruction::LoadConstant(v) => self.do_load_constant(v),
                Instruction::Load(depth, index) => self.do_load(depth, index),
                Instruction::LoadFunction(args, body) => self.do_load_function(args, body),
                Instruction::Apply => self.do_apply(),
                Instruction::Return => self.do_return(),
                Instruction::Dummy => self.do_dummy(),
                Instruction::RecursiveApply => self.do_recursive_apply(),
                Instruction::Select => self.do_select(),
                Instruction::Join => self.do_join(),
                Instruction::Stop => Ok(false),
                Instruction::Add => self.do_add(),
                Instruction::Sub => self.do_sub(),
                Instruction::Mul => self.do_mul(),
                Instruction::Div => self.do_div(),
                Instruction::Rem => self.do_rem(),
                Instruction::Equal => self.do_equal(),
                Instruction::NotEqual => self.do_not_equal(),
                Instruction::LessThan => self.do_less_than(),
                Instruction::LessOrEqual => self.do_less_or_equal(),
                Instruction::GreaterThan => self.do_greater_than(),
                Instruction::GreaterOrEqual => self.do_greater_or_equal(),
                Instruction::Cons => self.do_cons(),
                Instruction::Car => self.do_car(),
                Instruction::Cdr => self.do_cdr(),
                Instruction::IsAtom => self.do_is_atom(),
                Instruction::IsNull => self.do_is_null(),
            }
        }
    }
}
impl DebuggableMachine for SimpleMachine {
    fn environment_dump(&self, max_depth: usize) {
        println!("Environment:");
        for (idx, state) in self.dump[..max_depth].iter().rev().enumerate() {
            println!("{:>04}:-----", idx,);
            for (idx, cell) in state.environment.iter().rev().enumerate() {
                println!("     {:>04}: {:?}", idx, cell,)
            }
        }
    }

    fn instruction_dump(&self, memory_range: RangeFull) {
        println!("Code:");
        for (idx, instruction) in self.memory[memory_range].iter().enumerate() {
            println!(
                "{} {:>04}: {}",
                if idx == self.current_state.code_ptr {
                    "*"
                } else {
                    " "
                },
                idx,
                instruction
            )
        }
    }

    fn stack_dump(&self) {
        println!("Stack:");
        for (idx, cell) in self.current_state.stack.iter().rev().enumerate() {
            println!(
                "{:>02}: {}",
                idx,
                match cell {
                    Cell::Datum(v) => format!("{} ({})", v.to_repr_string(), v.inner_type_name()),
                    Cell::Closure(_, _) => cell_type_name(cell),
                }
            )
        }
    }
}

impl SimpleMachine {
    fn stack_push(&mut self, value: Cell) {
        self.current_state.stack.push(value);
    }
    fn stack_pop(&mut self) -> Cell {
        self.current_state.stack.pop().unwrap()
    }
    fn do_nil(mut self) -> Result<Option<Self>, Error> {
        // pushes a nil pointer onto the stack
        self.stack_push(Cell::Datum(Datum::Null));
        Ok(Some(self))
    }
    fn do_load_constant(mut self, value: Datum) -> Result<Option<Self>, Error> {
        // pushes a constant argument onto the stack
        self.stack_push(Cell::Datum(value));
        Ok(Some(self))
    }
    fn do_load(mut self, depth: usize, index: usize) -> Result<Option<Self>, Error> {
        // pushes the value of a variable onto the stack. The variable is indicated by the argument,
        // a pair. The pair's car specifies the level, the cdr the position. So (1 . 3) gives the
        // current function's (level 1) third parameter
        if let Some(state) = self.dump.get(self.dump.len() - depth) {
            if let Some(value) = state.environment.get(index) {
                self.stack_push(value.clone());
                return Ok(Some(self));
            }
        }
        Err(ErrorKind::InvalidEnvironmentIndex(depth, index).into())
    }
    fn do_load_function(
        mut self,
        args: Vec<Identifier>,
        body: Vec<Instruction>,
    ) -> Result<Option<Self>, Error> {
        // takes one list argument representing a function. It constructs a closure (a pair
        // containing the function and the current environment) and pushes that onto the stack
        self.stack_push(Cell::Closure(args, body));
        Ok(Some(self))
    }
    fn do_apply(self) -> Result<Option<Self>, Error> {
        // pops a closure and a list of parameter values from the stack. The closure is applied to
        // the parameters by installing its environment as the current one, pushing the parameter
        // list in front of that, clearing the stack, and setting C to the closure's function
        // pointer. The previous values of S, E, and the next value of C are saved on the dump.
        todo!()
    }
    fn do_return(mut self) -> Result<Option<Self>, Error> {
        // pops one return value from the stack, restores S, E, and C from the dump, and pushes the
        // return value onto the now-current stack
        let ret_value = self.stack_pop();
        let _ = self.current_state.pop();
        self.stack_push(ret_value);
        Ok(Some(self))
    }
    fn do_dummy(self) -> Result<Option<Self>, Error> {
        // pushes a "dummy", an empty list, in front of the environment list
        todo!()
    }
    fn do_recursive_apply(self) -> Result<Option<Self>, Error> {
        // works like apply, only that it replaces an occurrence of a dummy environment with the
        // current one, thus making recursive functions possible
        todo!()
    }
    fn do_select(self) -> Result<Option<Self>, Error> {
        // expects two list arguments, and pops a value from the stack. The first list is executed
        // if the popped value was non-nil, the second list otherwise. Before one of these list
        // pointers is made the new C, a pointer to the instruction following sel is saved on the
        // dump
        todo!()
    }
    fn do_join(self) -> Result<Option<Self>, Error> {
        // pops a list reference from the dump and makes this the new value of C. This instruction
        // occurs at the end of both alternatives of a sel.
        todo!()
    }
    fn do_numeric_binary_op(
        mut self,
        op: impl Fn(Number, Number) -> Number,
    ) -> Result<Option<Self>, Error> {
        if self.stack_depth() < 2 {
            Err(ErrorKind::InsufficientStack(2).into())
        } else {
            let rhs = self.stack_pop();
            let rhs = if let Cell::Datum(Datum::Number(value)) = rhs {
                value
            } else {
                return Err(
                    ErrorKind::TypeMismatch("Number".to_string(), cell_type_name(&rhs)).into(),
                );
            };
            let lhs = self.stack_pop();
            let lhs = if let Cell::Datum(Datum::Number(value)) = lhs {
                value
            } else {
                return Err(
                    ErrorKind::TypeMismatch("Number".to_string(), cell_type_name(&lhs)).into(),
                );
            };
            let result = op(lhs, rhs);
            self.stack_push(Cell::Datum(Datum::Number(result)));
            Ok(Some(self))
        }
    }
    fn do_add(mut self) -> Result<Option<Self>, Error> {
        self.do_numeric_binary_op(Number::add)
    }
    fn do_sub(self) -> Result<Option<Self>, Error> {
        self.do_numeric_binary_op(Number::sub)
    }
    fn do_mul(self) -> Result<Option<Self>, Error> {
        self.do_numeric_binary_op(Number::mul)
    }
    fn do_div(self) -> Result<Option<Self>, Error> {
        self.do_numeric_binary_op(Number::div)
    }
    fn do_rem(self) -> Result<Option<Self>, Error> {
        self.do_numeric_binary_op(Number::rem)
    }
    fn do_equal(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_not_equal(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_less_than(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_less_or_equal(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_greater_than(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_greater_or_equal(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_cons(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_car(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_cdr(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_is_atom(self) -> Result<Option<Self>, Error> {
        todo!()
    }
    fn do_is_null(self) -> Result<Option<Self>, Error> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn cell_type_name(cell: &Cell) -> String {
    match cell {
        Cell::Datum(v) => v.inner_type_name(),
        Cell::Closure(args, _) => format!(
            "(λ ({}) ...)",
            args.iter()
                .map(|i| i.to_repr_string())
                .collect::<Vec<String>>()
                .join(" ")
        ),
    }
}
// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod debug {}

mod trace {}
