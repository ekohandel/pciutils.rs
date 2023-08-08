use crate::access::Access;
use crate::bdf::BusDeviceFunction;
use crate::caps::header::CommonHeader;
use crate::caps::header::Header;
use crate::caps::Capability;
use crate::caps::CapabilityFactory;
use crate::error::Result;
use crate::kernel::Kernel;
use crate::vdc::VendorDeviceClass;
use std::fmt::Display;

pub struct Function {
    bdf: BusDeviceFunction,
    header: Header,
    kernel: Kernel,
    capabilities: Result<Vec<Box<dyn Capability>>>,
}

impl Function {
    pub fn new(bdf: BusDeviceFunction, accessor: Box<dyn Access>, kernel: Kernel) -> Result<Self> {
        let function = Function {
            bdf,
            header: Header::new(&accessor.read(0, 0x40)?)?,
            kernel,
            capabilities: CapabilityFactory::new(accessor).scan(),
        };

        Ok(function)
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
        Ok((self.base_class_code()? as u16) << 8 | self.sub_class_code()? as u16)
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
        let mut text = format!("{} {}\n", self.bdf, self.header.to_string(verbosity)?,);

        if verbosity > 0 {
            match &self.capabilities {
                Err(_) => text += "\tCapabilities: <access denied>\n",
                Ok(capabilities) => {
                    for cap in capabilities {
                        text += &format!(
                            "\tCapabilities: [{:x}] {}\n",
                            cap.offset()?,
                            cap.cap_string(verbosity)?
                        );
                    }
                }
            }

            text += &self.kernel.text(&self.bdf, verbosity)?;
        }

        Ok(text.trim().to_string())
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
