use std::fs;
use std::io::Read;
use std::os::unix::prelude::FileExt;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use crate::access::Access;
use crate::bdf::BusDeviceFunction;
use crate::error::Result;
use crate::function::Function;
use crate::kernel::Kernel;

#[derive(Debug)]
pub struct Sysfs;

impl Sysfs {
    const PCI_FUNCTIONS_PATH: &str = "/sys/bus/pci/devices";

    pub fn discover() -> Result<Vec<Function>> {
        let mut bdfs = vec![];

        for entry in fs::read_dir(Self::PCI_FUNCTIONS_PATH)? {
            let entry = entry?;
            bdfs.push(BusDeviceFunction::from_str(
                entry.file_name().to_str().unwrap_or_else(|| {
                    panic!(
                        "PCI device directory names in {} are expected to be valid utf-8 string.",
                        Self::PCI_FUNCTIONS_PATH
                    )
                }),
            )?);
        }

        bdfs.sort();

        let mut functions = vec![];
        for bdf in bdfs {
            functions.push(Function::new(
                bdf,
                Rc::new(Box::new(SysfsAccess::new(bdf))),
                Kernel,
            )?);
        }

        Ok(functions)
    }

    pub fn get_function_sub_path(bdf: &BusDeviceFunction, sub: &str) -> PathBuf {
        let mut path: PathBuf = Self::PCI_FUNCTIONS_PATH.into();

        path.push(bdf.canonical_bdf_string());
        path.push(sub);

        path
    }

    fn get_function_parameter(bdf: &BusDeviceFunction, parameter: &str) -> Result<String> {
        let path = Self::get_function_sub_path(bdf, parameter);
        let content = fs::read_to_string(path)?;
        Ok(content.trim().trim_start_matches("0x").to_string())
    }

    fn parse_function_parameter_u8(bdf: &BusDeviceFunction, parameter: &str) -> Result<u8> {
        let parameter = Self::get_function_parameter(bdf, parameter)?;
        Ok(u8::from_str_radix(&parameter, 16)?)
    }

    fn parse_function_parameter_u16(bdf: &BusDeviceFunction, parameter: &str) -> Result<u16> {
        let parameter = Self::get_function_parameter(bdf, parameter)?;
        Ok(u16::from_str_radix(&parameter, 16)?)
    }

    fn parse_function_parameter_u16_optional(
        bdf: &BusDeviceFunction,
        parameter: &str,
    ) -> Result<Option<u16>> {
        match Self::parse_function_parameter_u16(bdf, parameter) {
            Ok(value) => Ok(Some(value)),
            Err(error) => {
                if error.is_file_not_found() {
                    Ok(None)
                } else {
                    Err(error)
                }
            }
        }
    }

    fn parse_function_parameter_u8_optional(
        bdf: &BusDeviceFunction,
        parameter: &str,
    ) -> Result<Option<u8>> {
        match Self::parse_function_parameter_u8(bdf, parameter) {
            Ok(value) => Ok(Some(value)),
            Err(error) => {
                if error.is_file_not_found() {
                    Ok(None)
                } else {
                    Err(error)
                }
            }
        }
    }

    fn parse_function_parameter_u32(bdf: &BusDeviceFunction, parameter: &str) -> Result<u32> {
        let parameter = Self::get_function_parameter(bdf, parameter)?;
        Ok(u32::from_str_radix(&parameter, 16)?)
    }

    pub fn vendor_id(&self, bdf: &BusDeviceFunction) -> Result<u16> {
        Self::parse_function_parameter_u16(bdf, "vendor")
    }

    pub fn device_id(&self, bdf: &BusDeviceFunction) -> Result<u16> {
        Self::parse_function_parameter_u16(bdf, "device")
    }

    pub fn revision_id(&self, bdf: &BusDeviceFunction) -> Result<u8> {
        Self::parse_function_parameter_u8(bdf, "revision")
    }

    pub fn class_code(&self, bdf: &BusDeviceFunction) -> Result<u16> {
        let class_code = Self::parse_function_parameter_u32(bdf, "class")?;
        Ok(((class_code >> 8) & 0xffff).try_into().unwrap())
    }

    pub fn base_class_code(&self, bdf: &BusDeviceFunction) -> Result<u8> {
        Ok(((self.class_code(bdf)? >> 8) & 0xff).try_into().unwrap())
    }

    pub fn sub_class_code(&self, bdf: &BusDeviceFunction) -> Result<u8> {
        Ok((self.class_code(bdf)? & 0xff).try_into().unwrap())
    }

    pub fn subsystem_vendor(&self, bdf: &BusDeviceFunction) -> Result<Option<u16>> {
        Self::parse_function_parameter_u16_optional(bdf, "subsystem_vendor")
    }

    pub fn subsystem_device(&self, bdf: &BusDeviceFunction) -> Result<Option<u16>> {
        Self::parse_function_parameter_u16_optional(bdf, "subsystem_device")
    }

    pub fn secondary_bus_number(&self, bdf: &BusDeviceFunction) -> Result<Option<u8>> {
        Self::parse_function_parameter_u8_optional(bdf, "secondary_bus_number")
    }

    pub fn subordinate_bus_number(&self, bdf: &BusDeviceFunction) -> Result<Option<u8>> {
        Self::parse_function_parameter_u8_optional(bdf, "subordinate_bus_number")
    }

    pub fn config(&self, bdf: &BusDeviceFunction) -> Result<Vec<u8>> {
        let path = Self::get_function_sub_path(bdf, "config");
        let mut file = fs::File::open(path)?;

        let mut config = vec![];
        file.read_to_end(&mut config)?;

        Ok(config)
    }
}

pub struct SysfsAccess {
    bdf: BusDeviceFunction,
}

impl SysfsAccess {
    pub fn new(bdf: BusDeviceFunction) -> SysfsAccess {
        SysfsAccess { bdf }
    }
}

impl Access for SysfsAccess {
    fn read(&self, offset: u64, length: usize) -> Result<Vec<u8>> {
        let path = Sysfs::get_function_sub_path(&self.bdf, "config");
        let file = fs::File::open(path)?;

        let mut buffer = Vec::new();
        buffer.resize(length, 0);

        file.read_exact_at(&mut buffer[..], offset)?;

        Ok(buffer)
    }

    fn write(&self, offset: u64, buffer: &[u8]) -> Result<usize> {
        let path = Sysfs::get_function_sub_path(&self.bdf, "config");
        let file = fs::File::options().write(true).open(path)?;

        Ok(file.write_at(buffer, offset)?)
    }
}
