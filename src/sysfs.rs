use std::fs;
use std::str::FromStr;

use crate::bdf::BusDeviceFunction;
use crate::error::Result;
use crate::function::Function;

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

        Ok(bdfs.into_iter().map(|bdf| Function { bdf }).collect())
    }
}
