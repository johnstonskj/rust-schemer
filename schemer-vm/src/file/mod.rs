/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use std::convert::TryFrom;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const VM_HEADER_ID: [u8; 8] = [0x01, 0x4, 0x2D, 0x0, 0x09, 0x2D, 0x0, 0x08];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FileHeader {
    file_type: FileType,
    vm_version: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum FileType {
    Library = 0x10,
    Image = 0x20,
}

pub const VM_CURRENT_VERSION: u8 = 0x10;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Into<Vec<u8>> for FileHeader {
    fn into(self) -> Vec<u8> {
        let mut results = VM_HEADER_ID.to_vec();
        results.push(self.file_type as u8);
        results.push(self.vm_version);
        results
    }
}

impl TryFrom<Vec<u8>> for FileHeader {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}

impl TryFrom<&[u8]> for FileHeader {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != FileHeader::read_len() {
            Err(ErrorKind::FileHeader.into())
        } else if value[0..VM_HEADER_ID.len()] != VM_HEADER_ID {
            Err(ErrorKind::FileHeader.into())
        } else {
            let file_type = FileType::try_from(value[VM_HEADER_ID.len() + 1])?;
            let vm_version = value[VM_HEADER_ID.len() + 2];
            Ok(Self::new_with_version(file_type, vm_version))
        }
    }
}
impl FileHeader {
    pub fn new(file_type: FileType) -> Self {
        Self::new_with_version(file_type, VM_CURRENT_VERSION)
    }
    pub fn new_with_version(file_type: FileType, vm_version: u8) -> Self {
        Self {
            file_type,
            vm_version,
        }
    }
    pub fn file_type(&self) -> FileType {
        self.file_type
    }
    pub fn vm_version(&self) -> u8 {
        self.vm_version
    }
    pub fn read_len() -> usize {
        VM_HEADER_ID.len() + 2
    }
    pub fn validate(&self, expected_type: FileType, expected_version: u8) -> Result<(), Error> {
        if self.file_type != expected_type {
            Err(ErrorKind::FileHeader.into())
        } else if self.vm_version > expected_version {
            Err(ErrorKind::VersionMismatch(self.vm_version, expected_version).into())
        } else {
            Ok(())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl TryFrom<u8> for FileType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(Self::Library),
            0x20 => Ok(Self::Image),
            _ => Err(ErrorKind::FileHeader.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod asm;

pub mod dis;

pub(crate) mod io;

pub mod parser;
