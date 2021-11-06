use crate::common;
use crate::MojangError;

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub uuid: String,
    pub name: String,
}

impl Player {
    pub fn new<T>(name_uuid: T) -> Result<Player, MojangError>
    where
        T: std::fmt::Display,
    {
        // If length is less than 16 input must be a name
        if name_uuid.to_string().len() < 16 {
            let resp = common::name_to_uuid(name_uuid.to_string())?;
            return Ok(Player {
                uuid: resp.1,
                name: resp.0,
            });
        }

        // If length is 16 ore more input must be a UUID
        // ... or some other nonsense but thats not the point
        let resp = common::uuid_to_name(name_uuid.to_string())?;
        Ok(Player {
            uuid: resp.1,
            name: resp.0,
        })
    }
}
