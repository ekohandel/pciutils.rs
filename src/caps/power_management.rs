use crate::access::Access;
use crate::error::{Error, Result};
use std::fmt::Display;
use std::rc::Rc;

use super::Capability;

pub struct PowerManagementCapability {
    access: Rc<Box<dyn Access>>,
    offset: u8,
}

impl PowerManagementCapability {
    pub fn new(access: Rc<Box<dyn Access>>, offset: u8) -> Result<PowerManagementCapability> {
        Ok(PowerManagementCapability { access, offset })
    }
}

impl Capability for PowerManagementCapability {
    fn cap_string(&self, _verbosity: u8) -> Result<String> {
        Ok(format!(
            "Power Management version {}\n",
            self.access
                .read(self.offset as u64 + 2, 1)?
                .pop()
                .ok_or(Error::unknown_capability(0))?
                & 0x7
        )
        .trim()
        .to_string())
    }

    fn offset(&self) -> Result<u64> {
        Ok(self.offset.into())
    }
}

impl Display for PowerManagementCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cap_string(0)?)
    }
}
