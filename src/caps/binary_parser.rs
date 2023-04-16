use crate::error::{Error, Result};
use std::ops::Range;

pub struct BinaryParser;

impl BinaryParser {
    pub fn le32(b: &[u8], r: Range<usize>) -> Result<u32> {
        Ok(u32::from_le_bytes(
            (*b.get(r.clone()).ok_or(Error::slice_parse_error(b, &r))?).try_into()?,
        ))
    }

    pub fn le16(b: &[u8], r: Range<usize>) -> Result<u16> {
        Ok(u16::from_le_bytes(
            (*b.get(r.clone()).ok_or(Error::slice_parse_error(b, &r))?).try_into()?,
        ))
    }

    pub fn le8(b: &[u8], r: Range<usize>) -> Result<u8> {
        Ok(u8::from_le_bytes(
            (*b.get(r.clone()).ok_or(Error::slice_parse_error(b, &r))?).try_into()?,
        ))
    }
}
