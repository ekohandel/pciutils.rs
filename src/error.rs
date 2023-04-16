use std::convert::From;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::Range;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IntegerParseError,
    InvalidBusDeviceFunction,
    InvalidVendorDeviceClass,
    IoError(std::io::ErrorKind),
    FormatError,
    SliceParseError,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub error_kind: ErrorKind,
    pub message: String,
}

impl Error {
    pub fn invalid_bdf(message: &str) -> Self {
        Error {
            error_kind: ErrorKind::InvalidBusDeviceFunction,
            message: message.to_string(),
        }
    }

    pub fn invalid_vdc(message: &str) -> Self {
        Error {
            error_kind: ErrorKind::InvalidVendorDeviceClass,
            message: message.to_string(),
        }
    }

    pub fn is_file_not_found(&self) -> bool {
        if let ErrorKind::IoError(std::io::ErrorKind::NotFound) = self.error_kind {
            return true;
        }

        false
    }

    pub fn slice_parse_error(b: &[u8], r: &Range<usize>) -> Error {
        let message = format!("Index range {}:{} outside of 0:{}", r.start, r.end, b.len());
        Error {
            error_kind: ErrorKind::SliceParseError,
            message,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error {
            error_kind: ErrorKind::IntegerParseError,
            message: value.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            error_kind: ErrorKind::IoError(value.kind()),
            message: value.to_string(),
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, value.message)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Self {
        Error {
            error_kind: ErrorKind::FormatError,
            message: String::new(),
        }
    }
}

impl From<Error> for std::fmt::Error {
    fn from(_: Error) -> Self {
        std::fmt::Error {}
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(value: std::array::TryFromSliceError) -> Self {
        Error {
            error_kind: ErrorKind::SliceParseError,
            message: value.to_string(),
        }
    }
}
