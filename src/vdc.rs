use crate::error::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct VendorDeviceClass {
    pub vendor: Option<u16>,
    pub device: Option<u16>,
    pub class: Option<u16>,
}

impl VendorDeviceClass {
    pub const FORMAT: &str = "[<vendor>]:[<device>][:<class>]";
    fn from_str(s: &str) -> Result<Self> {
        let mut parts: Vec<_> = s.split(':').collect();

        if parts.len() < 2 || parts.len() > 3 {
            return Err(Error::invalid_vdc(s));
        }

        let mut class = None;
        if parts.len() == 3 {
            if let Some(part) = parts.pop() {
                if !part.is_empty() {
                    class = Some(u16::from_str_radix(part, 16).map_err(|_| Error::invalid_bdf(s))?);
                }
            }
        }

        let mut device = None;
        if let Some(part) = parts.pop() {
            if !part.is_empty() {
                device = Some(u16::from_str_radix(part, 16).map_err(|_| Error::invalid_bdf(s))?);
            }
        }

        let mut vendor = None;
        if let Some(part) = parts.pop() {
            if !part.is_empty() {
                vendor = Some(u16::from_str_radix(part, 16).map_err(|_| Error::invalid_bdf(s))?);
            }
        }

        Ok(VendorDeviceClass {
            vendor,
            device,
            class,
        })
    }
}

impl FromStr for VendorDeviceClass {
    type Err = std::io::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_str(s).map_err(std::io::Error::from)
    }
}

impl PartialEq for VendorDeviceClass {
    fn eq(&self, other: &Self) -> bool {
        (self.vendor.is_none()
            || other.vendor.is_none()
            || self.vendor.unwrap() == other.vendor.unwrap())
            && (self.device.is_none()
                || other.device.is_none()
                || self.device.unwrap() == other.device.unwrap())
            && (self.class.is_none()
                || other.class.is_none()
                || self.class.unwrap() == other.class.unwrap())
    }
}
