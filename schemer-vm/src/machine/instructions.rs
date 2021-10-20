/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::{Identifier, SchemeRepr};
use std::collections::LinkedList;
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
    Load(Identifier),
    /// (s, e, LDF.(args body).c^, d) => (((args body).e).s, e, c, d)
    LoadFunction(Vec<Identifier>, LinkedList<Instruction>),
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
    /// (v1.v2.s, e, MOD.c, d)  => (v.s, e, c, d)
    Mod,
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
    Mod = 0x25,
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
                Instruction::Load(v) => format!("LD {}", v.to_repr_string()),
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
                Instruction::Mod => "MOD".to_string(),
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

impl TryFrom<u8> for InstructionType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Nil),
            0x01 => Ok(Self::LoadConstant),
            0x02 => Ok(Self::Load),
            0x03 => Ok(Self::LoadFunction),
            0x11 => Ok(Self::Apply),
            0x12 => Ok(Self::Return),
            0x13 => Ok(Self::Dummy),
            0x14 => Ok(Self::RecursiveApply),
            0x21 => Ok(Self::Add),
            0x22 => Ok(Self::Sub),
            0x23 => Ok(Self::Mul),
            0x24 => Ok(Self::Div),
            0x25 => Ok(Self::Mod),
            0x31 => Ok(Self::Equal),
            0x32 => Ok(Self::NotEqual),
            0x33 => Ok(Self::LessThan),
            0x34 => Ok(Self::LessOrEqual),
            0x35 => Ok(Self::GreaterThan),
            0x36 => Ok(Self::GreaterOrEqual),
            0x41 => Ok(Self::Cons),
            0x42 => Ok(Self::Car),
            0x43 => Ok(Self::Cdr),
            0x51 => Ok(Self::IsAtom),
            0x52 => Ok(Self::IsNull),
            0x61 => Ok(Self::Select),
            0x71 => Ok(Self::Join),
            0xFF => Ok(Self::Stop),
            _ => Err(ErrorKind::Format.into()),
        }
    }
}

impl From<&Instruction> for InstructionType {
    fn from(v: &Instruction) -> Self {
        match v {
            Instruction::Nil => Self::Nil,
            Instruction::Load(_) => Self::Load,
            Instruction::LoadConstant(_) => Self::LoadConstant,
            Instruction::LoadFunction(_, _) => Self::LoadFunction,
            Instruction::Apply => Self::Apply,
            Instruction::Return => Self::Return,
            Instruction::Dummy => Self::Dummy,
            Instruction::RecursiveApply => Self::RecursiveApply,
            Instruction::Add => Self::Add,
            Instruction::Sub => Self::Sub,
            Instruction::Mul => Self::Mul,
            Instruction::Div => Self::Div,
            Instruction::Mod => Self::Mod,
            Instruction::Equal => Self::Equal,
            Instruction::NotEqual => Self::NotEqual,
            Instruction::LessThan => Self::LessThan,
            Instruction::LessOrEqual => Self::LessOrEqual,
            Instruction::GreaterThan => Self::GreaterThan,
            Instruction::GreaterOrEqual => Self::GreaterOrEqual,
            Instruction::Cons => Self::Cons,
            Instruction::Car => Self::Car,
            Instruction::Cdr => Self::Cdr,
            Instruction::IsAtom => Self::IsAtom,
            Instruction::IsNull => Self::IsNull,
            Instruction::Select => Self::Select,
            Instruction::Join => Self::Join,
            Instruction::Stop => Self::Stop,
        }
    }
}

impl From<Instruction> for InstructionType {
    fn from(v: Instruction) -> Self {
        InstructionType::from(&v)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
