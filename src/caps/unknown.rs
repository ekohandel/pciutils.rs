use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::ops::Range;
use std::rc::Rc;

use crate::access::Access;
use crate::caps::Capability;
use crate::error::Result;

use super::binary_parser;

pub struct UnknownCapability {
    access: Rc<Box<dyn Access>>,
    offset: u8,
}

impl UnknownCapability {
    pub fn new(access: Rc<Box<dyn Access>>, offset: u8) -> Result<UnknownCapability> {
        Ok(UnknownCapability { access, offset })
    }

    pub fn id(access: &Rc<Box<dyn Access>>, offset: u8) -> Result<u8> {
        Ok(access.read(offset.into(), 1)?.pop().ok_or(Error::new(
            ErrorKind::PermissionDenied,
            format!("Unable to read offset {}", offset + 1),
        ))?)
    }

    pub fn next(access: &Rc<Box<dyn Access>>, offset: u8) -> Result<u8> {
        Ok(access.read(offset as u64 + 1, 1)?.pop().ok_or(Error::new(
            ErrorKind::PermissionDenied,
            format!("Unable to read offset {}", offset + 1),
        ))?)
    }
}

impl Capability for UnknownCapability {
    fn cap_string(&self, _verbosity: u8) -> Result<String> {
        let id = Self::id(&self.access, self.offset)?;
        Ok(format!("Capability {:#x} at {:#x}", id, self.offset))
    }

    fn offset(&self) -> Result<u64> {
        Ok(self.offset.into())
    }
}

impl Display for UnknownCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cap_string(0)?)
    }
}

pub struct UnknownExtendedCapability {
    access: Rc<Box<dyn Access>>,
    offset: u16,
}

impl UnknownExtendedCapability {
    pub fn new(access: Rc<Box<dyn Access>>, offset: u16) -> Result<UnknownExtendedCapability> {
        Ok(UnknownExtendedCapability { access, offset })
    }

    pub fn id(access: &Rc<Box<dyn Access>>, offset: u16) -> Result<u16> {
        binary_parser::BinaryParser::le16(
            &access.read(offset.into(), 2)?,
            Range { start: 0, end: 2 },
        )
    }

    pub fn next(access: &Rc<Box<dyn Access>>, offset: u16) -> Result<u16> {
        Ok(binary_parser::BinaryParser::le16(
            &access.read(offset as u64 + 2, 2)?,
            Range { start: 0, end: 2 },
        )? >> 4)
    }
}

impl Capability for UnknownExtendedCapability {
    fn cap_string(&self, _verbosity: u8) -> Result<String> {
        let id = Self::id(&self.access, self.offset)?;
        Ok(format!("Capability {:#x} at {:#x}", id, self.offset))
    }

    fn offset(&self) -> Result<u64> {
        Ok(self.offset.into())
    }
}

impl Display for UnknownExtendedCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cap_string(0)?)
    }
}
