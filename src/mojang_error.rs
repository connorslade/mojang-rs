use std::io;

use ureq::Error;

#[derive(Debug)]
pub enum MojangError {
    NoNameOrUUID,

    ReadError(io::Error),
    RequestError(Error),
    ParseError,
}
