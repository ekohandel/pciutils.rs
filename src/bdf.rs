use crate::error::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct BusDeviceFunction {
    domain: u16,
    bus: u8,
    device: u8,
    function: u8,
}

impl BusDeviceFunction {
    fn bdf_string(&self) -> String {
        if self.domain == 0 {
            format!("{:0>2x}:{:0>2x}.{:x}", self.bus, self.device, self.function)
        } else {
            self.bdf_string_with_domain()
        }
    }

    fn bdf_string_with_domain(&self) -> String {
        format!(
            "{:0>4x}:{:0>2x}:{:0>2x}.{:x}",
            self.domain, self.bus, self.device, self.function
        )
    }

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([0-9A-Fa-f]{4})?:?([0-9A-Fa-f]{2}):([0-9A-Fa-f]{2}).([0-9A-Fa-f]{1})$"
            )
            .unwrap();
        }

        let caps = RE.captures(s);

        if caps.is_none() {
            return Err(Error::invalid_bdf(s));
        }

        let caps = caps.unwrap();

        if caps.get(2).is_none() || caps.get(3).is_none() || caps.get(4).is_none() {
            return Err(Error::invalid_bdf(s));
        }

        Ok(BusDeviceFunction {
            domain: match caps.get(1) {
                Some(domain) => u16::from_str_radix(domain.as_str(), 16)?,
                _ => 0,
            },
            bus: u8::from_str_radix(caps.get(2).unwrap().as_str(), 16)?,
            device: u8::from_str_radix(caps.get(3).unwrap().as_str(), 16)?,
            function: u8::from_str_radix(caps.get(4).unwrap().as_str(), 16)?,
        })
    }
}

impl FromStr for BusDeviceFunction {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_str(s)
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
            BusDeviceFunction::from_str("0000:00:00.0").unwrap(),
            BusDeviceFunction {
                domain: 0x0000,
                bus: 0x00,
                device: 0x00,
                function: 0x0,
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("01:02.3").unwrap(),
            BusDeviceFunction {
                domain: 0x0000,
                bus: 0x01,
                device: 0x02,
                function: 0x3,
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("0001:02:03.4").unwrap(),
            BusDeviceFunction {
                domain: 0x0001,
                bus: 0x02,
                device: 0x03,
                function: 0x4,
            }
        );
        assert_eq!(
            BusDeviceFunction::from_str("0002:34:12.f").unwrap(),
            BusDeviceFunction {
                domain: 0x0002,
                bus: 0x34,
                device: 0x12,
                function: 0xf,
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
            BusDeviceFunction::from_str(":03.4"),
            Err(Error::invalid_bdf(":03.4"))
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
                domain: 0x0000,
                bus: 0x00,
                device: 0x00,
                function: 0x0,
            }
            .to_string(),
            "00:00.0"
        );
        assert_eq!(
            BusDeviceFunction {
                domain: 0x0001,
                bus: 0x02,
                device: 0x03,
                function: 0x4,
            }
            .to_string(),
            "0001:02:03.4"
        );
        assert_eq!(
            BusDeviceFunction {
                domain: 0x0002,
                bus: 0x34,
                device: 0x12,
                function: 0xf,
            }
            .to_string(),
            "0002:34:12.f"
        );
    }
}
