#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

type Result<T> = result::Result<T, MojangError>;

pub mod api;
mod common;
mod error;
mod player;
mod server_block;

use std::result;

pub use error::MojangError;
pub use player::Player;
pub use server_block::BlockedServers;
