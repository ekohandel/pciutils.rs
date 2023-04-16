use crate::bar::BAR;
use crate::caps::binary_parser::BinaryParser;
use crate::error::{Error, Result};
use pci_ids::{Device, FromId, Subclass, Vendor};
use std::ops::Range;

pub trait CommonHeader {
    const HEADER_TYPE_LAYOUT_MASK: u8 = 0b0111_1111;

    fn get_raw(&self) -> &[u8];
    fn bars(&self) -> Result<Vec<BAR>>;
    fn to_string(&self, verbosity: u8) -> Result<String>;

    fn vendor_id(&self) -> Result<u16> {
        BinaryParser::le16(
            self.get_raw(),
            Range {
                start: 0x000,
                end: 0x002,
            },
        )
    }

    fn device_id(&self) -> Result<u16> {
        BinaryParser::le16(
            self.get_raw(),
            Range {
                start: 0x002,
                end: 0x004,
            },
        )
    }

    fn command(&self) -> Result<u16> {
        BinaryParser::le16(
            self.get_raw(),
            Range {
                start: 0x004,
                end: 0x006,
            },
        )
    }

    fn status(&self) -> Result<u16> {
        BinaryParser::le16(
            self.get_raw(),
            Range {
                start: 0x006,
                end: 0x008,
            },
        )
    }

    fn revision_id(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x008,
                end: 0x009,
            },
        )
    }

    fn programming_interface(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x009,
                end: 0x00A,
            },
        )
    }

    fn sub_class_code(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x00A,
                end: 0x00B,
            },
        )
    }

    fn base_class_code(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x00B,
                end: 0x00C,
            },
        )
    }

    fn header_type(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x000E,
                end: 0x000F,
            },
        )
    }

    fn capability_pointer(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x034,
                end: 0x035,
            },
        )
    }

    fn vendor_name(&self) -> Result<String> {
        let vendor_id = self.vendor_id()?;
        let vendor = match Vendor::from_id(vendor_id) {
            Some(vendor) => vendor.name().to_string(),
            None => format!("Vendor {:0>4x}", vendor_id),
        };

        Ok(vendor)
    }

    fn device_string(&self) -> Result<String> {
        let base_class_id = self.base_class_code()?;
        let sub_class_id = self.sub_class_code()?;
        let sub_class = match Subclass::from_cid_sid(base_class_id, sub_class_id) {
            Some(sub_class) => sub_class.name().to_string(),
            None => format!("SubClass {:0>2x}", sub_class_id),
        };

        let vendor_id = self.vendor_id()?;
        let device_id = self.device_id()?;
        let device = match Device::from_vid_pid(vendor_id, device_id) {
            Some(device) => device.name().to_string(),
            None => format!("Device {:0>4x}", device_id),
        };

        let revision_id = self.revision_id()?;
        let revision = if revision_id > 0 {
            format!("(rev {:0>2x})", revision_id)
        } else {
            String::new()
        };

        let text = format!(
            "{}: {} {} {}",
            sub_class,
            self.vendor_name()?,
            device,
            revision
        )
        .trim()
        .to_string();

        Ok(text)
    }

    fn header_layout(b: &[u8]) -> Result<u8> {
        let layout = BinaryParser::le8(
            b,
            Range {
                start: 0x00E,
                end: 0x00F,
            },
        )?;

        Ok(layout & Self::HEADER_TYPE_LAYOUT_MASK)
    }
}

#[derive(Debug)]
pub struct Type0Header {
    raw: Vec<u8>,
}

impl CommonHeader for Type0Header {
    fn get_raw(&self) -> &[u8] {
        &self.raw
    }

    fn bars(&self) -> Result<Vec<BAR>> {
        let range = Range {
            start: 0x010,
            end: 0x028,
        };
        Ok(BAR::new(
            self.raw
                .get(range.clone())
                .ok_or(Error::slice_parse_error(&self.raw, &range))?,
        ))
    }

    fn to_string(&self, verbosity: u8) -> Result<String> {
        let mut text = self.device_string()?;

        if verbosity >= 1 {
            text = format!("{}\n\t{}", text, self.subsytem_string()?);
        }

        Ok(text.trim().to_string())
    }
}

impl Type0Header {
    pub fn new(b: &[u8]) -> Result<Self> {
        Ok(Self { raw: b.to_vec() })
    }

    pub fn subsystem_vendor_id(&self) -> Result<u16> {
        BinaryParser::le16(
            self.get_raw(),
            Range {
                start: 0x02C,
                end: 0x02E,
            },
        )
    }

    pub fn subsystem_id(&self) -> Result<u16> {
        BinaryParser::le16(
            self.get_raw(),
            Range {
                start: 0x02E,
                end: 0x030,
            },
        )
    }

    fn subsytem_string(&self) -> Result<String> {
        let subsystem_device = self.subsystem_id()?;
        let subsystem_vendor = self.subsystem_vendor_id()?;

        let device_id = self.device_id()?;
        let vendor_id = self.vendor_id()?;

        let device = Device::from_vid_pid(vendor_id, device_id);
        if device.is_none() {
            return Ok(format!(
                "Subsystem: {} Device {:0>4x}",
                self.vendor_name()?,
                subsystem_device
            ));
        }

        let device = device.unwrap();
        if let Some(device) = device
            .subsystems()
            .find(|d| d.subdevice() == subsystem_device)
        {
            return Ok(format!(
                "Subsystem: {} {}",
                self.vendor_name()?,
                device.name()
            ));
        }

        Ok(format!(
            "Subsystem: Vendor {:0>4x} Device {:0>4x}",
            subsystem_vendor, subsystem_device
        ))
    }
}

#[derive(Debug)]
pub struct Type1Header {
    raw: Vec<u8>,
}

impl CommonHeader for Type1Header {
    fn get_raw(&self) -> &[u8] {
        &self.raw
    }

    fn bars(&self) -> Result<Vec<BAR>> {
        let range = Range {
            start: 0x010,
            end: 0x014,
        };
        Ok(BAR::new(
            self.raw
                .get(range.clone())
                .ok_or(Error::slice_parse_error(&self.raw, &range))?,
        ))
    }

    fn to_string(&self, verbosity: u8) -> Result<String> {
        let mut text = self.device_string()?;

        if verbosity >= 1 {
            text = format!("{}\n\t{}", text, self.bus_string()?);
        }

        Ok(text.trim().to_string())
    }
}

impl Type1Header {
    pub fn new(b: &[u8]) -> Result<Self> {
        Ok(Self { raw: b.to_vec() })
    }

    pub fn primary_bus_number(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x018,
                end: 0x019,
            },
        )
    }

    pub fn secondary_bus_number(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x019,
                end: 0x01A,
            },
        )
    }

    pub fn subordinate_bus_number(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x01A,
                end: 0x01B,
            },
        )
    }

    pub fn secondary_latency_timer(&self) -> Result<u8> {
        BinaryParser::le8(
            self.get_raw(),
            Range {
                start: 0x01B,
                end: 0x01C,
            },
        )
    }

    fn bus_string(&self) -> Result<String> {
        Ok(format!(
            "Bus: primary={:0>2x}, secondary={:0>2x}, subordiante={:0>2x}, sec-latency={}",
            self.primary_bus_number()?,
            self.secondary_bus_number()?,
            self.subordinate_bus_number()?,
            self.secondary_latency_timer()?,
        ))
    }
}

#[derive(Debug)]
pub enum Header {
    Type0(Type0Header),
    Type1(Type1Header),
}

impl Header {
    pub fn new(b: &[u8]) -> Result<Self> {
        if <Type0Header as CommonHeader>::header_layout(b)? == 0 {
            Ok(Header::Type0(Type0Header::new(b)?))
        } else {
            Ok(Header::Type1(Type1Header::new(b)?))
        }
    }
}

impl CommonHeader for Header {
    fn get_raw(&self) -> &[u8] {
        match self {
            Header::Type0(h) => h.get_raw(),
            Header::Type1(h) => h.get_raw(),
        }
    }

    fn bars(&self) -> Result<Vec<BAR>> {
        match self {
            Header::Type0(h) => h.bars(),
            Header::Type1(h) => h.bars(),
        }
    }

    fn to_string(&self, verbosity: u8) -> Result<String> {
        match self {
            Header::Type0(h) => h.to_string(verbosity),
            Header::Type1(h) => h.to_string(verbosity),
        }
    }
}
