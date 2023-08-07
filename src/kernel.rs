use crate::access::sysfs::Sysfs;
use crate::bdf::BusDeviceFunction;
use crate::error::Result;
use std::fs::read_link;

#[derive(Debug)]
pub struct Kernel;

impl Kernel {
    pub fn text(&self, bdf: &BusDeviceFunction, verbosity: u8) -> Result<String> {
        Ok(format!(
            "{}\n{}",
            self.driver_text(bdf, verbosity)?,
            self.module_text(bdf, verbosity)?
        ))
    }

    pub fn driver_text(&self, bdf: &BusDeviceFunction, _verbosity: u8) -> Result<String> {
        let driver_symlink = Sysfs::get_function_sub_path(bdf, "driver");
        let driver_path = read_link(driver_symlink);

        if driver_path.is_err() {
            return Ok(String::new());
        }

        let driver_path = driver_path?;
        let driver_path = driver_path.to_str().unwrap_or_default();

        Ok(format!(
            "\tKernel driver in use: {}",
            driver_path.split('/').last().unwrap_or_default()
        ))
    }

    pub fn module_text(&self, bdf: &BusDeviceFunction, _verbosity: u8) -> Result<String> {
        let module_symlink = Sysfs::get_function_sub_path(bdf, "driver/module");
        let module_path = read_link(module_symlink);

        if module_path.is_err() {
            return Ok(String::new());
        }

        let module_path = module_path?;
        let module_path = module_path.to_str().unwrap_or_default();

        Ok(format!(
            "\tKernel modules: {}",
            module_path.split('/').last().unwrap_or_default()
        ))
    }
}
