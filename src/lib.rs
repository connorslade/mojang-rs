use tinyjson::JsonValue;

mod common;

#[derive(Debug, Clone, Copy)]
pub enum MojangError {
    NoNameOrUUID,

    RequestError,
    ParseError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub uuid: Option<String>,
    pub name: Option<String>,
}

impl Player {
    pub fn new<T>(name_uuid: T) -> Player
    where
        T: std::fmt::Display,
    {
        // If length is less than 16 input must be a name
        if name_uuid.to_string().len() < 16 {
            return Player {
                name: Some(name_uuid.to_string()),
                ..Player::default()
            };
        }

        // If length is 16 ore more input must be a UUID
        // ... or some other nonsense but thats not the point
        Player {
            uuid: Some(name_uuid.to_string().to_lowercase().replace("-", "")),
            ..Player::default()
        }
    }

    pub fn name(&self) -> Result<String, MojangError> {
        // Return name if we have it alredy
        if self.name.is_some() {
            return Ok(self.name.clone().unwrap());
        }

        // If not get it from the Mojang API
        if self.uuid.is_some() {
            let agent = common::ureq_agent();
            match agent
                .get(&format!(
                    "https://sessionserver.mojang.com/session/minecraft/profile/{}",
                    self.uuid.clone().unwrap()
                ))
                .call()
            {
                Ok(i) => match &i.into_string().unwrap().parse::<JsonValue>().unwrap()["name"] {
                    JsonValue::String(i) => return Ok(i.to_string()),
                    _ => return Err(MojangError::ParseError),
                },

                Err(_) => return Err(MojangError::RequestError),
            };
        }

        Err(MojangError::NoNameOrUUID)
    }

    pub fn uuid(&self) -> Result<String, MojangError> {
        unimplemented!()
    }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            uuid: None,
            name: None,
        }
    }
}
