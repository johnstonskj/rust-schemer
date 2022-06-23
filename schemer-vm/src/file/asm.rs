/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::file::io::Writer;
use crate::file::{FileHeader, FileType};
use crate::machine::datum::DatumType;
use crate::machine::instructions::{Instruction, InstructionType};
use schemer_lang::read::datum::Datum;
use schemer_lang::types::{
    Boolean, ByteVector, Char, ExactComplex, ExactReal, Identifier, InexactComplex, InexactReal,
    Integer, Number, Pair, Rational, SchemeString, Vector,
};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn assemble_into_file<T: AsRef<Path>>(
    byte_code: &[Instruction],
    file_name: &T,
    file_type: FileType,
) -> Result<(), Error> {
    let mut file = File::create(file_name)?;
    let mut writer = Writer::wrap(&mut file);
    writer.file_header(FileHeader::new(file_type))?;
    assemble(&mut writer, byte_code)
}

#[instrument]
pub fn assemble_into(instructions: &[Instruction]) -> Result<Vec<u8>, Error> {
    let mut inner = BufWriter::new(Vec::new());
    let mut writer = Writer::wrap(&mut inner);
    for instruction in instructions {
        write_instruction(&mut writer, instruction)?;
    }
    // TODO: better error handling here
    Ok(inner.into_inner().unwrap())
}

#[instrument]
fn assemble<W: Write>(writer: &mut Writer<W>, instructions: &[Instruction]) -> Result<(), Error> {
    for instruction in instructions {
        write_instruction(writer, instruction)?;
    }
    Ok(())
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

fn write_instruction<W: Write>(
    writer: &mut Writer<W>,
    instruction: &Instruction,
) -> Result<(), Error> {
    match instruction {
        Instruction::LoadConstant(v) => write_load_constant(writer, v),
        Instruction::Load(depth, index) => write_load(writer, *depth, *index),
        Instruction::LoadFunction(args, body) => write_load_function(writer, args, body),
        i => writer.instruction_type(i.into()),
    }
}

fn write_load<W: Write>(writer: &mut Writer<W>, depth: usize, index: usize) -> Result<(), Error> {
    writer.instruction_type(InstructionType::Load)?;
    writer.usize(depth)?;
    writer.usize(index)
}

fn write_identifier<W: Write>(writer: &mut Writer<W>, id: &Identifier) -> Result<(), Error> {
    writer.u8(DatumType::Identifier as u8)?;
    writer.string(&id.as_str())
}

fn write_load_constant<W: Write>(writer: &mut Writer<W>, constant: &Datum) -> Result<(), Error> {
    writer.instruction_type(InstructionType::LoadConstant)?;
    write_datum(writer, constant)
}

fn write_datum<W: Write>(writer: &mut Writer<W>, datum: &Datum) -> Result<(), Error> {
    match datum {
        Datum::Null => write_datum_null(writer),
        Datum::Boolean(v) => write_datum_boolean(writer, v),
        Datum::Number(n) => match n {
            Number::InexactComplex(n) => write_datum_inexact_complex(writer, n),
            Number::ExactComplex(n) => write_datum_exact_complex(writer, n),
            Number::InexactReal(n) => write_datum_inexact_real(writer, n),
            Number::ExactReal(n) => write_datum_exact_real(writer, n),
            Number::Rational(n) => write_datum_rational(writer, n),
            Number::Integer(n) => write_datum_integer(writer, n),
        },
        Datum::Character(v) => write_datum_character(writer, v),
        Datum::String(v) => write_datum_string(writer, v),
        Datum::ByteVector(v) => write_datum_byte_vector(writer, v),
        Datum::List(v) => write_datum_list(writer, v),
        Datum::Vector(v) => write_datum_vector(writer, v),
        _ => unreachable!(),
    }
}

fn write_datum_null<W: Write>(writer: &mut Writer<W>) -> Result<(), Error> {
    writer.data_type(DatumType::Null)
}

fn write_datum_boolean<W: Write>(writer: &mut Writer<W>, value: &Boolean) -> Result<(), Error> {
    writer.data_type(DatumType::Boolean)?;
    writer.u8(if value.is_true() { 1 } else { 0 })
}

fn write_datum_inexact_complex<W: Write>(
    writer: &mut Writer<W>,
    value: &InexactComplex,
) -> Result<(), Error> {
    writer.data_type(DatumType::InexactComplex)?;
    writer.f64(value.re)?;
    writer.f64(value.im)
}

fn write_datum_exact_complex<W: Write>(
    writer: &mut Writer<W>,
    value: &ExactComplex,
) -> Result<(), Error> {
    writer.data_type(DatumType::ExactComplex)?;
    writer.bytes(&value.re.serialize())?;
    writer.bytes(&value.im.serialize())
}

fn write_datum_inexact_real<W: Write>(
    writer: &mut Writer<W>,
    value: &InexactReal,
) -> Result<(), Error> {
    writer.data_type(DatumType::InexactReal)?;
    writer.f64(*value)
}

fn write_datum_exact_real<W: Write>(
    writer: &mut Writer<W>,
    value: &ExactReal,
) -> Result<(), Error> {
    writer.data_type(DatumType::ExactReal)?;
    writer.bytes(&value.serialize())
}

fn write_datum_rational<W: Write>(writer: &mut Writer<W>, value: &Rational) -> Result<(), Error> {
    writer.data_type(DatumType::Rational)?;
    writer.i64(*value.numer())?;
    writer.i64(*value.denom())
}

fn write_datum_integer<W: Write>(writer: &mut Writer<W>, value: &Integer) -> Result<(), Error> {
    writer.data_type(DatumType::Integer)?;
    writer.i64(*value)
}

fn write_datum_character<W: Write>(writer: &mut Writer<W>, value: &Char) -> Result<(), Error> {
    writer.data_type(DatumType::Character)?;
    writer.char(**value)
}

fn write_datum_string<W: Write>(writer: &mut Writer<W>, value: &SchemeString) -> Result<(), Error> {
    writer.data_type(DatumType::String)?;
    writer.bytes_with_length(&value.as_bytes())
}

fn write_datum_list<W: Write>(writer: &mut Writer<W>, value: &Pair) -> Result<(), Error> {
    writer.data_type(DatumType::List)?;
    writer.usize(value.length())?;
    for datum in value.iter() {
        write_source_datum(writer, datum)?;
    }
    Ok(())
}

fn write_source_datum<W: Write>(writer: &mut Writer<W>, value: &Datum) -> Result<(), Error> {
    match &value {
        Datum::Null => write_datum_null(writer),
        Datum::Boolean(v) => write_datum_boolean(writer, &v),
        Datum::Number(v) => match &v {
            Number::InexactComplex(v) => write_datum_inexact_complex(writer, v),
            Number::ExactComplex(v) => write_datum_exact_complex(writer, v),
            Number::InexactReal(v) => write_datum_inexact_real(writer, v),
            Number::ExactReal(v) => write_datum_exact_real(writer, v),
            Number::Rational(v) => write_datum_rational(writer, v),
            Number::Integer(v) => write_datum_integer(writer, v),
        },
        Datum::Character(v) => write_datum_character(writer, &v),
        Datum::String(v) => write_datum_string(writer, &v),
        Datum::Symbol(v) => write_identifier(writer, &v),
        Datum::ByteVector(v) => write_datum_byte_vector(writer, &v),
        Datum::List(v) => write_datum_list(writer, &v),
        Datum::Vector(v) => write_datum_vector(writer, &v),
        _ => return Err(ErrorKind::Format.into()),
    }
}

fn write_datum_vector<W: Write>(
    writer: &mut Writer<W>,
    value: &Vector<Datum>,
) -> Result<(), Error> {
    writer.data_type(DatumType::Vector)?;
    writer.usize(value.len())?;
    for datum in value.iter() {
        write_source_datum(writer, datum)?;
    }
    Ok(())
}

fn write_datum_byte_vector<W: Write>(
    writer: &mut Writer<W>,
    value: &ByteVector,
) -> Result<(), Error> {
    writer.data_type(DatumType::ByteVector)?;
    writer.bytes_with_length(value)
}

fn write_load_function<W: Write>(
    writer: &mut Writer<W>,
    args: &Vec<Identifier>,
    body: &Vec<Instruction>,
) -> Result<(), Error> {
    writer.instruction_type(InstructionType::LoadFunction)?;

    writer.usize(args.len())?;
    for id in args {
        write_identifier(writer, id)?;
    }

    writer.usize(body.len())?;
    for instruction in body {
        write_instruction(writer, instruction)?;
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
