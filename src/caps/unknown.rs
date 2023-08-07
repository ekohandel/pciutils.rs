use std::fmt::Display;
use std::rc::Rc;

use crate::access::Access;
use crate::caps::Capability;
use crate::error::Result;

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
