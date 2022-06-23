/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Error;
use crate::file::asm::assemble_into;
use crate::file::io::{Reader, Writer};
use crate::file::{FileHeader, FileType, VM_CURRENT_VERSION};
use crate::machine::Instruction;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Memory(Vec<u8>);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Vec<u8>> for Memory {
    fn from(vs: Vec<u8>) -> Self {
        Self(vs)
    }
}

impl TryFrom<Vec<Instruction>> for Memory {
    type Error = Error;

    fn try_from(value: Vec<Instruction>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}

impl TryFrom<&[Instruction]> for Memory {
    type Error = Error;

    fn try_from(value: &[Instruction]) -> Result<Self, Self::Error> {
        Ok(Self(assemble_into(value)?))
    }
}

impl Memory {
    pub fn load_from_image<T: AsRef<Path>>(&self, file_name: &T) -> Result<Self, Error> {
        let file = File::open(file_name)?;
        let data_bytes = file.metadata()?.len() as usize - FileHeader::read_len();
        let mut reader = BufReader::new(file);
        let mut reader = Reader::wrap(&mut reader);
        let file_header = reader.file_header()?;
        file_header.validate(FileType::Image, VM_CURRENT_VERSION)?;
        Ok(Self(reader.bytes(data_bytes)?.unwrap()))
    }

    pub fn save_to_image<T: AsRef<Path>>(&self, file_name: &T) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = Writer::wrap(&mut file);
        writer.file_header(FileHeader::new(FileType::Image))?;
        writer.bytes(&self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_vec(&self) -> &Vec<u8> {
        self.0.as_ref()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
