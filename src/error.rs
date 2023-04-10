use std::convert::From;
use std::fmt::Debug;
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IntegerParseError,
    InvalidBusDeviceFunction,
    InvalidVendorDeviceClass,
    IoError(i32),
    FormatError,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    error_kind: ErrorKind,
    message: String,
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
            error_kind: ErrorKind::IoError(value.raw_os_error().unwrap_or_default()),
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
