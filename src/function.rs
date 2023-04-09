use crate::error::Result;
use crate::{bdf::BusDeviceFunction, sysfs::Sysfs};
use pci_ids::{Device, FromId, Subclass, Vendor};
use std::fmt::Display;

#[derive(Debug)]
pub struct Function {
    pub bdf: BusDeviceFunction,
    accessor: Sysfs,
}

impl Function {
    pub fn new(bdf: BusDeviceFunction, accessor: Sysfs) -> Self {
        Function { bdf, accessor }
    }

    pub fn vendor_id(&self) -> Result<u16> {
        self.accessor.vendor_id(&self.bdf)
    }

    pub fn device_id(&self) -> Result<u16> {
        self.accessor.device_id(&self.bdf)
    }

    pub fn revision_id(&self) -> Result<u8> {
        self.accessor.revision_id(&self.bdf)
    }

    pub fn class_code(&self) -> Result<u8> {
        self.accessor.class_code(&self.bdf)
    }

    pub fn sub_class_code(&self) -> Result<u8> {
        self.accessor.sub_class_code(&self.bdf)
    }

    pub fn config(&self) -> Result<Vec<u8>> {
        self.accessor.config(&self.bdf)
    }

    pub fn brief(&self) -> Result<String> {
        let base_class_id = self.class_code()?;
        let sub_class_id = self.sub_class_code()?;
        let vendor_id = self.vendor_id()?;
        let device_id = self.device_id()?;
        let revision_id = self.revision_id()?;

        let sub_class = match Subclass::from_cid_sid(base_class_id, sub_class_id) {
            Some(sub_class) => sub_class.name().to_string(),
            None => format!("SubClass {:0>2x}", vendor_id),
        };

        let vendor = match Vendor::from_id(vendor_id) {
            Some(vendor) => vendor.name().to_string(),
            None => format!("Vendor {:0>4x}", vendor_id),
        };

        let device = match Device::from_vid_pid(vendor_id, device_id) {
            Some(device) => device.name().to_string(),
            None => format!("Device {:0>4x}", device_id),
        };

        let revision = if revision_id > 0 {
            format!("(rev {:0>2x})", revision_id)
        } else {
            String::new()
        };

        Ok(format!(
            "{} {}: {} {} {}",
            self.bdf, sub_class, vendor, device, revision
        )
        .trim()
        .to_string())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.brief()?)
    }
}
