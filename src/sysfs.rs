use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use crate::bdf::BusDeviceFunction;
use crate::error::Result;
use crate::function::Function;

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

        Ok(bdfs
            .into_iter()
            .map(|bdf| Function::new(bdf, Self))
            .collect())
    }

    fn get_function_sub_path(bdf: &BusDeviceFunction, sub: &str) -> PathBuf {
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

    pub fn config(&self, bdf: &BusDeviceFunction) -> Result<Vec<u8>> {
        let path = Self::get_function_sub_path(bdf, "config");
        let mut file = fs::File::open(path)?;

        let mut config = vec![];
        file.read_to_end(&mut config)?;

        Ok(config)
    }
}
