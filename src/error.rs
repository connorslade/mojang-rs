use std::io;

use ureq::Error;

/// Various errors that can come from the function in this crate
#[derive(Debug)]
pub enum MojangError {
    /// IO Error while reading a stream
    ReadError(io::Error),

    /// Ureq Request Error
    RequestError(Box<Error>),

    /// IO Error
    IoError(io::Error),

    /// Error parsing Json
    ParseError(serde_json::Error),
}

impl From<ureq::Error> for MojangError {
    fn from(value: ureq::Error) -> Self {
        MojangError::RequestError(Box::new(value))
    }
}

impl From<io::Error> for MojangError {
    fn from(value: io::Error) -> Self {
        MojangError::IoError(value)
    }
}

impl From<serde_json::Error> for MojangError {
    fn from(value: serde_json::Error) -> Self {
        MojangError::ParseError(value)
    }
}
