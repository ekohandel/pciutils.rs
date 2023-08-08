use crate::access::Access;
use crate::error::Result;
use std::collections::HashSet;
use std::rc::Rc;

use self::header::{CommonHeader, Header};
use self::power_management::PowerManagementCapability;
use self::unknown::{UnknownCapability, UnknownExtendedCapability};

pub mod binary_parser;
pub mod header;
pub mod power_management;
pub mod unknown;

pub trait Capability {
    fn cap_string(&self, _verbosity: u8) -> Result<String>;
    fn offset(&self) -> Result<u64>;
}

pub struct CapabilityFactory {
    access: Rc<Box<dyn Access>>,
}

impl CapabilityFactory {
    pub fn new(access: Rc<Box<dyn Access>>) -> CapabilityFactory {
        CapabilityFactory {
            access: Rc::clone(&access),
        }
    }

    pub fn scan(&self) -> Result<Vec<Box<dyn Capability>>> {
        let mut capabilities = self.scan_trad()?;

        let header = Header::new(&self.access.read(0, 0x40)?)?;

        //TODO: use dynamic traits to make this handling easier
        let has_extended_capabilities = match header {
            Header::Type0(h) => h.status()? & (1 << 4) > 0,
            Header::Type1(h) => h.status()? & (1 << 4) > 0,
        };
        if has_extended_capabilities {
            // Some devices advertise extended capabilities eventhough they don't have them. Just
            // ignore errors in that case.
            capabilities.append(&mut self.scan_extended().unwrap_or_default());
        }

        Ok(capabilities)
    }

    fn scan_trad(&self) -> Result<Vec<Box<dyn Capability>>> {
        let mut capabilities = vec![];
        let mut seen = HashSet::from([0]);

        let mut offset: u8 = self.access.read(0x34, 1)?.pop().unwrap_or_default();

        while !seen.contains(&offset) {
            seen.insert(offset);

            let id = UnknownCapability::id(&self.access, offset)?;
            capabilities.push(self.new_trad(id, offset)?);
            offset = UnknownCapability::next(&self.access, offset)?;
        }

        Ok(capabilities)
    }

    fn scan_extended(&self) -> Result<Vec<Box<dyn Capability>>> {
        let mut capabilities = vec![];
        let mut seen = HashSet::from([0]);

        let mut offset = 0x100;

        while !seen.contains(&offset) {
            seen.insert(offset);

            let id = UnknownExtendedCapability::id(&self.access, offset)?;
            capabilities.push(self.new_extended(id, offset)?);
            offset = UnknownExtendedCapability::next(&self.access, offset)?;
        }

        Ok(capabilities)
    }

    fn new_trad(&self, id: u8, offset: u8) -> Result<Box<dyn Capability>> {
        match id {
            0x1 => Ok(Box::new(PowerManagementCapability::new(
                Rc::clone(&self.access),
                offset,
            )?)),
            _ => Ok(Box::new(UnknownCapability::new(
                Rc::clone(&self.access),
                offset,
            )?)),
        }
    }

    fn new_extended(&self, _id: u16, offset: u16) -> Result<Box<dyn Capability>> {
        Ok(Box::new(UnknownExtendedCapability::new(
            Rc::clone(&self.access),
            offset,
        )?))
    }
}
