use std::str::FromStr;

use uuid::Uuid;

pub mod blocked_servers;
pub mod profile;
pub mod public_profile;
pub mod username;

#[derive(Debug)]
pub struct Url(String);

/// Skin model type
#[derive(Debug)]
pub enum Model {
    /// The original player model
    Normal,
    /// The slim player model
    Slim,
}

impl From<String> for Model {
    fn from(value: String) -> Self {
        if matches!(value.to_ascii_lowercase().as_str(), "slim") {
            return Model::Slim;
        }

        Model::Normal
    }
}
