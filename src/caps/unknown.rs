use std::fmt::Display;
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
}

impl Capability for UnknownCapability {
    fn cap_string(&self, _verbosity: u8) -> Result<String> {
        let id = self
            .access
            .read(self.offset.into(), 1)?
            .pop()
            .unwrap_or_default();
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
}

impl Capability for UnknownExtendedCapability {
    fn cap_string(&self, _verbosity: u8) -> Result<String> {
        let id = binary_parser::BinaryParser::le16(
            &self.access.read(self.offset.into(), 2)?,
            Range { start: 0, end: 2 },
        )?;
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
