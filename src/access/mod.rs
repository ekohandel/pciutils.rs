pub mod dump;
pub mod sysfs;

use crate::error::Result;

pub trait Access {
    fn read(&self, offset: u64, length: usize) -> Result<Vec<u8>>;
    fn write(&self, offset: u64, value: &[u8]) -> Result<usize>;
}
