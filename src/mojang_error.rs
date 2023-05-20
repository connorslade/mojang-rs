use std::io;

use ureq::Error;

/// Various errors that can come from the function in this crate
#[derive(Debug)]
pub enum MojangError {
    /// Not enough data to preform action
    ///
    /// Ex: Used when calling Player.name_at(n) if player doesn't have nave history data
    NotEnoughData,

    /// IO Error while reading a stream
    ReadError(io::Error),

    /// Ureq Request Error
    RequestError(Box<Error>),

    /// IO Error
    IoError(io::Error),

    /// Error parsing Data
    ParseError,
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
