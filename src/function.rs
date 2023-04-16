use crate::caps::header::CommonHeader;
use crate::caps::header::Header;
use crate::error::Result;
use crate::vdc::VendorDeviceClass;
use crate::{bdf::BusDeviceFunction, sysfs::Sysfs};
use std::fmt::Display;

#[derive(Debug)]
pub struct Function {
    bdf: BusDeviceFunction,
    header: Header,
    accessor: Sysfs,
}

impl Function {
    pub fn new(bdf: BusDeviceFunction, accessor: Sysfs) -> Result<Self> {
        let config = accessor.config(&bdf)?;
        Ok(Function {
            bdf,
            accessor,
            header: Header::new(&config)?,
        })
    }

    pub fn vendor_id(&self) -> Result<u16> {
        self.header.vendor_id()
    }

    pub fn device_id(&self) -> Result<u16> {
        self.header.device_id()
    }

    pub fn revision_id(&self) -> Result<u8> {
        self.header.revision_id()
    }

    pub fn class_code(&self) -> Result<u16> {
        self.accessor.class_code(&self.bdf)
    }

    pub fn base_class_code(&self) -> Result<u8> {
        self.header.base_class_code()
    }

    pub fn sub_class_code(&self) -> Result<u8> {
        self.header.sub_class_code()
    }

    pub fn subsystem_vendor_id(&self) -> Result<Option<u16>> {
        match &self.header {
            Header::Type0(h) => Ok(Some(h.subsystem_vendor_id()?)),
            _ => Ok(None),
        }
    }

    pub fn subsystem_id(&self) -> Result<Option<u16>> {
        match &self.header {
            Header::Type0(h) => Ok(Some(h.subsystem_id()?)),
            _ => Ok(None),
        }
    }

    pub fn config(&self) -> Result<Vec<u8>> {
        Ok(self.header.get_raw().to_vec())
    }

    pub fn to_string(&self, verbosity: u8) -> Result<String> {
        Ok(
            format!("{} {}", self.bdf, self.header.to_string(verbosity)?,)
                .trim()
                .to_string(),
        )
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(0)?)
    }
}

impl PartialEq<VendorDeviceClass> for Function {
    fn eq(&self, other: &VendorDeviceClass) -> bool {
        let vendor_eq = other.vendor.is_none()
            || other.vendor.unwrap()
                == self.vendor_id().unwrap_or_else(|_| {
                    panic!("Cannot read vendor id for {}", self.to_string(0).unwrap())
                });

        let device_eq = other.device.is_none()
            || other.device.unwrap()
                == self.device_id().unwrap_or_else(|_| {
                    panic!("Cannot read device id for {}", self.to_string(0).unwrap())
                });

        let class_eq = other.class.is_none()
            || other.class.unwrap()
                == self.class_code().unwrap_or_else(|_| {
                    panic!("Cannot read class code for {}", self.to_string(0).unwrap())
                });

        vendor_eq && device_eq && class_eq
    }
}

impl PartialEq<BusDeviceFunction> for Function {
    fn eq(&self, other: &BusDeviceFunction) -> bool {
        self.bdf == *other
    }
}
