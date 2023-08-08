use crate::access::Access;
use crate::error::Result;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

use self::power_management::PowerManagementCapability;
use self::unknown::UnknownCapability;

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
        let capabilities = self.scan_trad()?;

        Ok(capabilities)
    }

    fn scan_trad(&self) -> Result<Vec<Box<dyn Capability>>> {
        let mut capabilities = vec![];

        let mut offset: u8 = self.access.read(0x34, 1)?.pop().unwrap_or_default();

        while offset != 0 {
            let id = self.access.read(offset.into(), 1)?.pop().ok_or(Error::new(
                ErrorKind::PermissionDenied,
                format!("Unable to read offset {}", offset),
            ))?;
            capabilities.push(self.new_trad(id, offset)?);

            offset = self
                .access
                .read(offset as u64 + 1, 1)?
                .pop()
                .ok_or(Error::new(
                    ErrorKind::PermissionDenied,
                    format!("Unable to read offset {}", offset + 1),
                ))?;
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
}
