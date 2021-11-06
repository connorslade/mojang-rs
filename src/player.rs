use crate::common;
use crate::MojangError;

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub uuid: String,

    pub skin_url: Option<String>,
    pub name_changes: Option<Vec<(u64, String)>>,
}

impl Player {
    pub fn new<T>(name_uuid: T) -> Result<Player, MojangError>
    where
        T: std::fmt::Display,
    {
        // If length is less than 16 input must be a name
        if name_uuid.to_string().len() < 16 {
            let resp = common::get_uuid(name_uuid.to_string())?;
            return Ok(Player {
                uuid: resp.1,
                name: resp.0,
                skin_url: None,
                name_changes: None,
            });
        }

        // If length is 16 or more input must be a UUID
        // ... or some other nonsense but thats not the point
        let resp = common::get_profile(name_uuid.to_string())?;
        Ok(Player {
            name: resp.0,
            uuid: resp.1,
            skin_url: Some(resp.2),
            name_changes: None,
        })
    }

    pub fn add_skin(self) -> Result<Player, MojangError> {
        if self.skin_url.is_some() {
            return Ok(self);
        }

        let resp = common::get_profile(self.uuid.to_string())?;
        Ok(Player {
            skin_url: Some(resp.2),
            ..self
        })
    }

    pub fn add_name_change(self) -> Result<Player, MojangError> {
        if self.name_changes.is_some() {
            return Ok(self);
        }

        let resp = common::get_name_history(self.uuid.to_string())?;
        Ok(Player {
            name_changes: Some(resp),
            ..self
        })
    }
}
