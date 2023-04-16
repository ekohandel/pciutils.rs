use crate::error::{Error, Result};
use log::trace;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Eq, Ord, PartialOrd, Clone)]
pub struct BusDeviceFunction {
    domain: Option<u16>,
    bus: Option<u8>,
    device: Option<u8>,
    function: Option<u8>,
}

impl BusDeviceFunction {
    pub const FORMAT: &str = "[[[[<domain>]:]<bus>]:][<slot>][.[<func>]]";

    pub fn bus(&self) -> Option<u8> {
        self.bus
    }

    pub fn bdf_string(&self) -> String {
        let domain = match self.domain {
            Some(domain) => format!("{:0>4x}:", domain),
            None => String::new(),
        };

        let bus = match self.bus {
            Some(bus) => format!("{:0>2x}", bus),
            None => String::new(),
        };

        let device = match self.device {
            Some(device) => format!("{:0>2x}", device),
            None => String::new(),
        };

        let function = match self.function {
            Some(function) => format!("{:1x}", function),
            None => String::new(),
        };

        format!("{}{}:{}.{}", domain, bus, device, function)
            .trim()
            .to_owned()
    }

    pub fn canonical_bdf_string(&self) -> String {
        if self.domain.is_none() {
            let mut bdf = self.clone();
            bdf.domain = Some(0);
            return bdf.bdf_string();
        }

        self.bdf_string()
    }

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<_> = s.split('.').collect();

        if parts.len() > 2 {
            return Err(Error::invalid_bdf(s));
        }

        let mut function = None;
        if parts.len() == 2 && !parts[1].is_empty() {
            function = Some(u8::from_str_radix(parts[1], 16).map_err(|_| Error::invalid_bdf(s))?);
        }

        let mut parts: Vec<_> = parts[0].split(':').collect();

        let mut device = None;
        if let Some(part) = parts.pop() {
            if !part.is_empty() {
                device = Some(u8::from_str_radix(part, 16).map_err(|_| Error::invalid_bdf(s))?);
            }
        }

        let mut bus = None;
        if let Some(part) = parts.pop() {
            if !part.is_empty() {
                bus = Some(u8::from_str_radix(part, 16).map_err(|_| Error::invalid_bdf(s))?);
            }
        }

        let mut domain = None;
        if let Some(part) = parts.pop() {
            if !part.is_empty() {
                domain = Some(u16::from_str_radix(part, 16).map_err(|_| Error::invalid_bdf(s))?);
            }
        }

        if !parts.is_empty() {
            return Err(Error::invalid_bdf(s));
        }

        Ok(BusDeviceFunction {
            domain,
            bus,
            device,
            function,
        })
    }
}

impl PartialEq for BusDeviceFunction {
    fn eq(&self, other: &Self) -> bool {
        let eq = (self.domain.is_none()
            || other.domain.is_none()
            || self.domain.unwrap() == other.domain.unwrap())
            && (self.bus.is_none()
                || other.bus.is_none()
                || self.bus.unwrap() == other.bus.unwrap())
            && (self.device.is_none()
                || other.device.is_none()
                || self.device.unwrap() == other.device.unwrap())
            && (self.function.is_none()
                || other.function.is_none()
                || self.function.unwrap() == other.function.unwrap());

        trace!(
            target: "bdf",
            "Comparing for eq of PartialEq:\n {} => {}: {}",
            self, other, eq
        );

        eq
    }
}

impl FromStr for BusDeviceFunction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_str(s).map_err(std::io::Error::from)
    }
}

impl Display for BusDeviceFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bdf_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_string_parse() {
        assert_eq!(
            BusDeviceFunction::from_str("0").unwrap(),
            BusDeviceFunction {
                domain: None,
                bus: None,
                device: Some(0x00),
                function: None,
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str(".0").unwrap(),
            BusDeviceFunction {
                domain: None,
                bus: None,
                device: None,
                function: Some(0x0),
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("0000:00:00.0").unwrap(),
            BusDeviceFunction {
                domain: None,
                bus: Some(0x00),
                device: Some(0x00),
                function: Some(0x0),
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("01:02.3").unwrap(),
            BusDeviceFunction {
                domain: None,
                bus: Some(0x01),
                device: Some(0x02),
                function: Some(0x3),
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("0001:02:03.4").unwrap(),
            BusDeviceFunction {
                domain: Some(0x0001),
                bus: Some(0x02),
                device: Some(0x03),
                function: Some(0x4),
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("0002:34:12.f").unwrap(),
            BusDeviceFunction {
                domain: Some(0x0002),
                bus: Some(0x34),
                device: Some(0x12),
                function: Some(0xf),
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str(":03.4").unwrap(),
            BusDeviceFunction {
                domain: None,
                bus: None,
                device: Some(0x03),
                function: Some(0x4),
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("::03.4").unwrap(),
            BusDeviceFunction {
                domain: None,
                bus: None,
                device: Some(0x03),
                function: Some(0x4),
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str(".").unwrap(),
            BusDeviceFunction {
                domain: None,
                bus: None,
                device: None,
                function: None,
            }
        );
    }
    #[test]
    fn test_invalid_string_parse() {
        assert_eq!(
            BusDeviceFunction::from_str(":0000:00:00.0"),
            Err(Error::invalid_bdf(":0000:00:00.0"))
        );
        assert_eq!(
            BusDeviceFunction::from_str("hello world!"),
            Err(Error::invalid_bdf("hello world!"))
        );
    }
    #[test]
    fn test_string_format() {
        assert_eq!(
            BusDeviceFunction {
                domain: None,
                bus: None,
                device: Some(0x00),
                function: Some(0x0),
            }
            .to_string(),
            ":00.0"
        );
        assert_eq!(
            BusDeviceFunction {
                domain: None,
                bus: Some(0x00),
                device: Some(0x00),
                function: Some(0x0),
            }
            .to_string(),
            "00:00.0"
        );
        assert_eq!(
            BusDeviceFunction {
                domain: Some(0x0001),
                bus: Some(0x02),
                device: Some(0x03),
                function: Some(0x4),
            }
            .to_string(),
            "0001:02:03.4"
        );
        assert_eq!(
            BusDeviceFunction {
                domain: Some(0x0002),
                bus: Some(0x34),
                device: Some(0x12),
                function: Some(0xf),
            }
            .to_string(),
            "0002:34:12.f"
        );
    }
}
