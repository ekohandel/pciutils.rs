use crate::access::Access;
use crate::error::Result;

pub struct DumpAccess {
    dump: Vec<u8>,
}

impl DumpAccess {
    pub fn new(dump: &[u8]) -> DumpAccess {
        DumpAccess {
            dump: dump.to_vec(),
        }
    }
}

impl Access for DumpAccess {
    fn read(&self, offset: u64, length: usize) -> Result<Vec<u8>> {
        Ok(self
            .dump
            .clone()
            .into_iter()
            .skip(offset as usize)
            .take(length)
            .collect())
    }

    fn write(&self, _offset: u64, buffer: &[u8]) -> Result<usize> {
        Ok(buffer.len())
    }
}
