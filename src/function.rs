use crate::error::Result;
use crate::vdc::VendorDeviceClass;
use crate::{bdf::BusDeviceFunction, sysfs::Sysfs};
use pci_ids::{Device, FromId, Subclass, Vendor};
use std::fmt::Display;

#[derive(Debug)]
pub struct Function {
    bdf: BusDeviceFunction,
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

    pub fn class_code(&self) -> Result<u16> {
        self.accessor.class_code(&self.bdf)
    }

    pub fn base_class_code(&self) -> Result<u8> {
        self.accessor.base_class_code(&self.bdf)
    }

    pub fn sub_class_code(&self) -> Result<u8> {
        self.accessor.sub_class_code(&self.bdf)
    }

    pub fn config(&self) -> Result<Vec<u8>> {
        self.accessor.config(&self.bdf)
    }

    pub fn brief(&self) -> Result<String> {
        let base_class_id = self.base_class_code()?;
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

impl PartialEq<VendorDeviceClass> for Function {
    fn eq(&self, other: &VendorDeviceClass) -> bool {
        let vendor_eq = other.vendor.is_none()
            || other.vendor.unwrap()
                == self.vendor_id().unwrap_or_else(|_| {
                    panic!("Cannot read vendor id for {}", self.brief().unwrap())
                });

        let device_eq = other.device.is_none()
            || other.device.unwrap()
                == self.device_id().unwrap_or_else(|_| {
                    panic!("Cannot read device id for {}", self.brief().unwrap())
                });

        let class_eq = other.class.is_none()
            || other.class.unwrap()
                == self.class_code().unwrap_or_else(|_| {
                    panic!("Cannot read class code for {}", self.brief().unwrap())
                });

        vendor_eq && device_eq && class_eq
    }
}

impl PartialEq<BusDeviceFunction> for Function {
    fn eq(&self, other: &BusDeviceFunction) -> bool {
        self.bdf == *other
    }
}
