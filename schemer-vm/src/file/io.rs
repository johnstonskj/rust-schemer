/*!
One-line description.

More detailed description, with

# Example

 */

use crate::error::{Error, ErrorKind};
use crate::file::FileHeader;
use crate::machine::datum::DatumType;
use crate::machine::instructions::InstructionType;
use std::convert::TryFrom;
use std::io::{Read, Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(crate) struct Writer<'a, W: Write> {
    inner: &'a mut W,
}

pub(crate) struct Reader<'a, R: Read> {
    inner: &'a mut R,
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

impl<'a, W: Write> Writer<'a, W> {
    pub fn wrap(inner: &'a mut W) -> Self {
        Self { inner }
    }
    pub fn file_header(&mut self, header: FileHeader) -> Result<(), Error> {
        println!("Writer::file_header({:?})", header);
        let header_bytes: Vec<u8> = header.into();
        self.bytes(&header_bytes)
    }
    pub fn instruction_type(&mut self, v: InstructionType) -> Result<(), Error> {
        println!("Writer::instruction_type({:?})", v);
        self.u8(v as u8)
    }
    pub fn data_type(&mut self, v: DatumType) -> Result<(), Error> {
        println!("Writer::data_type({:?})", v);
        self.u8(v as u8)
    }
    pub fn u8(&mut self, v: u8) -> Result<(), Error> {
        println!("Writer::u8({:x})", v);
        self.inner.write(&[v])?;
        Ok(())
    }
    pub fn usize(&mut self, v: usize) -> Result<(), Error> {
        println!("Writer::usize({:?})", v);
        self.u32(v as u32)
    }
    pub fn u32(&mut self, v: u32) -> Result<(), Error> {
        println!("Writer::u32({:?})", v);
        self.bytes(&v.to_be_bytes())
    }
    pub fn i64(&mut self, v: i64) -> Result<(), Error> {
        println!("Writer::i64({:?})", v);
        self.bytes(&v.to_be_bytes())
    }
    pub fn f64(&mut self, v: f64) -> Result<(), Error> {
        println!("Writer::f64({:?})", v);
        self.bytes(&v.to_be_bytes())
    }
    pub fn char(&mut self, v: char) -> Result<(), Error> {
        println!("Writer::char({:?})", v);
        self.u32(v as u32)
    }
    pub fn string(&mut self, v: &str) -> Result<(), Error> {
        println!("Writer::string({:?})", v);
        self.bytes_with_length(v.as_bytes())
    }
    pub fn bytes(&mut self, v: &[u8]) -> Result<(), Error> {
        println!("Writer::bytes {:?}", v);
        self.inner.write(v)?;
        Ok(())
    }
    pub fn bytes_with_length(&mut self, v: &[u8]) -> Result<(), Error> {
        println!("Writer::bytes_with_length({:?})", v);
        self.usize(v.len())?;
        self.bytes(v)
    }
}

const BW_U8: usize = 1;
const BW_U32: usize = 4;
const BW_I64: usize = 8;
const BW_F64: usize = 8;

impl<'a, R: Read> Reader<'a, R> {
    pub fn wrap(inner: &'a mut R) -> Self {
        Self { inner }
    }

    pub fn file_header(&mut self) -> Result<FileHeader, Error> {
        println!("Reader::file_header");
        FileHeader::try_from(self.bytes(FileHeader::read_len())?.unwrap())
    }

    pub fn instruction_type(&mut self) -> Result<Option<InstructionType>, Error> {
        println!("Reader::instruction_type");
        if let Some(v) = self.u8()? {
            println!("{} as InstructionType", v);
            InstructionType::try_from(v).map(|v| Some(v))
        } else {
            Ok(None)
        }
    }

    pub fn data_type(&mut self) -> Result<Option<DatumType>, Error> {
        println!("Reader::data_type");
        if let Some(v) = self.u8()? {
            println!("{} as DatumType", v);
            DatumType::try_from(v).map(|v| Some(v))
        } else {
            Ok(None)
        }
    }

    pub fn u8(&mut self) -> Result<Option<u8>, Error> {
        println!("Reader::u8");
        let mut buffer: [u8; BW_U8] = [0; BW_U8];
        if self.inner.read(&mut buffer)? == BW_U8 {
            println!("{:?}u8", buffer);
            Ok(Some(buffer[0]))
        } else {
            Ok(None)
        }
    }

    pub fn usize(&mut self) -> Result<Option<usize>, Error> {
        println!("Reader::usize");
        Ok(self.u32()?.and_then(|v| Some(v as usize)))
    }

    pub fn u32(&mut self) -> Result<Option<u32>, Error> {
        println!("Reader::u32");
        let mut buffer: [u8; BW_U32] = [0; BW_U32];
        if self.inner.read(&mut buffer)? == BW_U32 {
            println!("{:?}u32", buffer);
            Ok(Some(u32::from_be_bytes(buffer)))
        } else {
            Ok(None)
        }
    }

    pub fn i64(&mut self) -> Result<Option<i64>, Error> {
        println!("Reader::i64");
        let mut buffer: [u8; BW_I64] = [0; BW_I64];
        if self.inner.read(&mut buffer)? == BW_I64 {
            println!("{:?}i64", buffer);
            Ok(Some(i64::from_be_bytes(buffer)))
        } else {
            Ok(None)
        }
    }

    pub fn f64(&mut self) -> Result<Option<f64>, Error> {
        println!("Reader::f64");
        let mut buffer: [u8; BW_F64] = [0; BW_F64];
        if self.inner.read(&mut buffer)? == BW_F64 {
            println!("{:?}f64", buffer);
            Ok(Some(f64::from_be_bytes(buffer)))
        } else {
            Ok(None)
        }
    }

    pub fn char(&mut self) -> Result<Option<char>, Error> {
        println!("Reader::char");
        if let Some(v) = self.u32()? {
            println!("{:?} as char", v);
            Ok(char::from_u32(v))
        } else {
            Ok(None)
        }
    }

    pub fn string(&mut self) -> Result<Option<String>, Error> {
        println!("Reader::string");
        if let Some(v) = self.bytes_with_length()? {
            println!("{:?} as str", v);
            String::from_utf8(v)
                .map(|v| Some(v))
                .map_err(|e| Error::chain(Box::new(e), ErrorKind::Format))
        } else {
            Ok(None)
        }
    }

    pub fn bytes(&mut self, length: usize) -> Result<Option<Vec<u8>>, Error> {
        println!("Reader::bytes({})", length);
        let mut buffer = Vec::with_capacity(length);
        if self.inner.read(buffer.as_mut_slice())? == length {
            println!("[u8;{}] = {:?}", length, buffer);
            Ok(Some(buffer))
        } else {
            Ok(None)
        }
    }

    pub fn bytes_with_length(&mut self) -> Result<Option<Vec<u8>>, Error> {
        println!("Reader::bytes_with_length");
        if let Some(v) = self.usize()? {
            self.bytes(v)
        } else {
            Ok(None)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
