use rmp_serde;
use sbe::zb_protocol;
use std::{error, io};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    DecodeError(String),
    EncodeError(String),
    SbeError(SbeError),
    SerdeDecodeError(rmp_serde::decode::Error),
    SerdeEncodeError(rmp_serde::encode::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref error) => error.description(),
            Error::DecodeError(ref error) |
            Error::EncodeError(ref error) => error,
            Error::SbeError(ref error) => error.as_str(),
            Error::SerdeDecodeError(ref error) => error.description(),
            Error::SerdeEncodeError(ref error) => error.description(),
        }

    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref error) => Some(error),
            Error::SerdeDecodeError(ref error) => Some(error),
            Error::SerdeEncodeError(ref error) => Some(error),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        error::Error::description(self).fmt(fmt)
    }
}

#[derive(Debug, PartialEq)]
pub enum SbeError {
    NotEnoughBytes,
    SliceIsLongerThanAllowedBySchema,
}

impl SbeError {
    fn as_str(&self) -> &'static str {
        match *self {
            SbeError::NotEnoughBytes => "not enough bytes",
            SbeError::SliceIsLongerThanAllowedBySchema => "slice is longer then allowed by schema",
        }
    }
}

impl From<zb_protocol::CodecErr> for Error {
    fn from(error: zb_protocol::CodecErr) -> Self {
        match error {
            zb_protocol::CodecErr::NotEnoughBytes => Error::SbeError(SbeError::NotEnoughBytes),
            zb_protocol::CodecErr::SliceIsLongerThanAllowedBySchema => Error::SbeError(SbeError::SliceIsLongerThanAllowedBySchema),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(error: rmp_serde::decode::Error) -> Self {
        Error::SerdeDecodeError(error)
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(error: rmp_serde::encode::Error) -> Self {
        Error::SerdeEncodeError(error)
    }
}
