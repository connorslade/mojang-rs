use std::io;

use ureq::Error;

#[derive(Debug)]
pub enum MojangError {
    NotEnoughData,

    ReadError(io::Error),
    RequestError(Error),
    ParseError,
}
