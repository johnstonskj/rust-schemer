/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::file::io::Reader;
use crate::file::{FileType, VM_CURRENT_VERSION};
use crate::machine::datum::DatumType;
use crate::machine::instructions::{Instruction, InstructionType};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::{
    Boolean, ByteVector, Char, ExactComplex, ExactReal, Identifier, InexactComplex, Number,
    Rational, SchemeString,
};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn disassemble_from_file<T: AsRef<Path>>(
    file_name: &T,
    file_type: FileType,
) -> Result<Vec<Instruction>, Error> {
    let mut file = BufReader::new(File::open(file_name)?);
    let mut reader = Reader::wrap(&mut file);
    let file_header = reader.file_header()?;
    if file_header.file_type() != file_type {
        Err(ErrorKind::FileHeader.into())
    } else if file_header.vm_version() > VM_CURRENT_VERSION {
        Err(ErrorKind::VersionMismatch(file_header.vm_version(), VM_CURRENT_VERSION).into())
    } else {
        disassemble(&mut reader)
    }
}

pub fn disassemble_from(memory: &[u8]) -> Result<Vec<Instruction>, Error> {
    println!("disassemble_from({:?})", memory);
    let mut reader = BufReader::new(memory);
    let mut reader = Reader::wrap(&mut reader);
    disassemble(&mut reader)
}

fn disassemble<R: Read>(reader: &mut Reader<R>) -> Result<Vec<Instruction>, Error> {
    println!("disassemble 1");
    let mut instructions = Vec::default();
    println!("disassemble 2");
    while let Some(instruction) = read_instruction(reader)? {
        println!("disassemble 3 - {:?}", instruction);
        instructions.push(instruction)
    }
    println!("disassemble 4");
    Ok(instructions)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn read_instruction<R: Read>(reader: &mut Reader<R>) -> Result<Option<Instruction>, Error> {
    println!("read_instruction");
    let instruction_type = reader.instruction_type()?;
    println!("read_instruction -> {:?}", instruction_type);
    match instruction_type {
        Some(InstructionType::Nil) => Ok(Some(Instruction::Nil)),
        Some(InstructionType::LoadConstant) => read_load_constant_instruction(reader),
        Some(InstructionType::Load) => read_load_instruction(reader),
        Some(InstructionType::LoadFunction) => read_load_function_instruction(reader),
        Some(InstructionType::Apply) => Ok(Some(Instruction::Apply)),
        Some(InstructionType::Return) => Ok(Some(Instruction::Return)),
        Some(InstructionType::Dummy) => Ok(Some(Instruction::Dummy)),
        Some(InstructionType::RecursiveApply) => Ok(Some(Instruction::RecursiveApply)),
        Some(InstructionType::Add) => Ok(Some(Instruction::Add)),
        Some(InstructionType::Sub) => Ok(Some(Instruction::Sub)),
        Some(InstructionType::Mul) => Ok(Some(Instruction::Mul)),
        Some(InstructionType::Div) => Ok(Some(Instruction::Div)),
        Some(InstructionType::Mod) => Ok(Some(Instruction::Mod)),
        Some(InstructionType::Equal) => Ok(Some(Instruction::Equal)),
        Some(InstructionType::NotEqual) => Ok(Some(Instruction::NotEqual)),
        Some(InstructionType::LessThan) => Ok(Some(Instruction::LessThan)),
        Some(InstructionType::LessOrEqual) => Ok(Some(Instruction::LessOrEqual)),
        Some(InstructionType::GreaterThan) => Ok(Some(Instruction::GreaterThan)),
        Some(InstructionType::GreaterOrEqual) => Ok(Some(Instruction::GreaterOrEqual)),
        Some(InstructionType::Cons) => Ok(Some(Instruction::Cons)),
        Some(InstructionType::Car) => Ok(Some(Instruction::Car)),
        Some(InstructionType::Cdr) => Ok(Some(Instruction::Cdr)),
        Some(InstructionType::IsAtom) => Ok(Some(Instruction::IsAtom)),
        Some(InstructionType::IsNull) => Ok(Some(Instruction::IsNull)),
        Some(InstructionType::Select) => Ok(Some(Instruction::Select)),
        Some(InstructionType::Join) => Ok(Some(Instruction::Join)),
        Some(InstructionType::Stop) => Ok(Some(Instruction::Stop)),
        None => Ok(None),
    }
}

fn read_load_instruction<R: Read>(reader: &mut Reader<R>) -> Result<Option<Instruction>, Error> {
    println!("read_load_instruction");
    Ok(Some(Instruction::Load(read_identifier(reader)?)))
}

fn read_identifier<R: Read>(reader: &mut Reader<R>) -> Result<Identifier, Error> {
    println!("read_identifier");
    if let Some(v) = reader.string()? {
        Identifier::from_str(&v).map_err(|e| Error::chain(Box::new(e), ErrorKind::Format))
    } else {
        Err(ErrorKind::Format.into())
    }
}

fn read_load_constant_instruction<R: Read>(
    reader: &mut Reader<R>,
) -> Result<Option<Instruction>, Error> {
    println!("read_load_constant_instruction");
    Ok(Some(Instruction::LoadConstant(read_datum(reader)?)))
}

fn read_datum<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_datum");
    match reader.data_type()? {
        Some(DatumType::Null) => Ok(Datum::Null),
        Some(DatumType::Boolean) => read_boolean(reader),
        Some(DatumType::Character) => read_char(reader),
        Some(DatumType::String) => read_string(reader),
        Some(DatumType::ByteVector) => read_byte_vector(reader),
        Some(DatumType::Integer) => read_integer(reader),
        Some(DatumType::Rational) => read_rational(reader),
        Some(DatumType::ExactReal) => read_exact_real(reader),
        Some(DatumType::InexactReal) => read_inexact_real(reader),
        Some(DatumType::ExactComplex) => read_exact_complex(reader),
        Some(DatumType::InexactComplex) => read_inexact_complex(reader),
        Some(DatumType::List) => read_list(reader),
        Some(DatumType::Vector) => read_vector(reader),
        _ => Err(ErrorKind::Format.into()),
    }
}

fn read_boolean<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_boolean");
    let value = reader.u8()?;
    let value = match value {
        Some(1) => true,
        Some(0) => false,
        _ => return Err(ErrorKind::Format.into()),
    };
    Ok(Datum::Boolean(Boolean::from(value)))
}

fn read_char<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_char");
    reader
        .char()?
        .map(|v| Datum::Character(Char::from(v)))
        .ok_or(ErrorKind::Format.into())
}

fn read_string<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_string");
    reader
        .string()?
        .map(|v| Datum::String(SchemeString::from(v)))
        .ok_or(ErrorKind::Format.into())
}

fn read_byte_vector<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_byte_vector");
    reader
        .bytes_with_length()?
        .map(|v| Datum::ByteVector(ByteVector::from(v)))
        .ok_or(ErrorKind::Format.into())
}

fn read_integer<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_integer");
    reader
        .i64()?
        .map(|v| Datum::Number(Number::Integer(v)))
        .ok_or(ErrorKind::Format.into())
}

fn read_rational<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_rational");
    let numer = reader.i64()?.ok_or::<Error>(ErrorKind::Format.into())?;
    let denom = reader.i64()?.ok_or::<Error>(ErrorKind::Format.into())?;
    Ok(Datum::Number(Number::Rational(Rational::new(numer, denom))))
}

fn read_exact_real<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_exact_real");
    Ok(Datum::Number(Number::ExactReal(read_exact_real_inner(
        reader,
    )?)))
}

fn read_exact_real_inner<R: Read>(reader: &mut Reader<R>) -> Result<ExactReal, Error> {
    println!("read_exact_real_inner");
    let bytes = reader.bytes(16)?.ok_or::<Error>(ErrorKind::Format.into())?;
    let byte_slice = <[u8; 16]>::try_from(bytes.as_slice())
        .map_err(|e| Error::chain(Box::new(e), ErrorKind::Format))?;
    Ok(ExactReal::deserialize(byte_slice))
}

fn read_inexact_real<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_inexact_real");
    reader
        .f64()?
        .map(|v| Datum::Number(Number::InexactReal(v)))
        .ok_or(ErrorKind::Format.into())
}

fn read_exact_complex<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_exact_complex");
    let re = read_exact_real_inner(reader)?;
    let im = read_exact_real_inner(reader)?;
    Ok(Datum::Number(Number::ExactComplex(ExactComplex::new(
        re, im,
    ))))
}

fn read_inexact_complex<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_inexact_complex");
    let re = reader.f64()?.ok_or::<Error>(ErrorKind::Format.into())?;
    let im = reader.f64()?.ok_or::<Error>(ErrorKind::Format.into())?;
    Ok(Datum::Number(Number::InexactComplex(InexactComplex::new(
        re, im,
    ))))
}

fn read_list<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_list");
    let len = reader.usize()?.ok_or::<Error>(ErrorKind::Format.into())?;
    todo!()
}

fn read_vector<R: Read>(reader: &mut Reader<R>) -> Result<Datum, Error> {
    println!("read_vector");
    let len = reader.usize()?.ok_or::<Error>(ErrorKind::Format.into())?;
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        result.insert(i, read_datum(reader)?)
    }
    Ok(Datum::Vector(result.into()))
}

fn read_load_function_instruction<R: Read>(
    reader: &mut Reader<R>,
) -> Result<Option<Instruction>, Error> {
    println!("read_load_function_instruction");
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
