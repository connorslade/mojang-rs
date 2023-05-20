use std::cell::RefCell;

use crate::common;
use crate::MojangError;

/// A Minecraft player.
#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    /// Player Name
    pub name: String,
    /// Player UUID
    ///
    /// All lowercase with no dashes
    pub uuid: String,

    /// Url of current player skin
    skin_url: RefCell<Option<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NameChange {
    pub time: u64,
    pub name: String,
}

impl Player {
    /// Make a new player
    ///
    /// You can supply the Name or UUID
    ///
    /// If you supply the uuid you will automatically get the skin URL
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::Player;
    ///
    /// // Make a player with Username
    /// let p = Player::new("Sigma76").unwrap();
    ///
    /// // Make a player with UUID
    /// let p2 = Player::new("3c358264b4564bdeab1efe1023db6679").unwrap();
    /// ```
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
                skin_url: RefCell::new(None),
            });
        }

        // If length is 16 or more input must be a UUID
        // ... or some other nonsense but thats not the point
        let (name, uuid, skin_url) = common::get_profile(name_uuid.to_string())?;
        Ok(Player {
            name,
            uuid,
            skin_url: RefCell::new(Some(skin_url)),
        })
    }

    /// Gets the skin url of the player.
    /// ## Example
    /// ```rust
    /// # use mojang::Player;
    /// let p = Player::new("Sigma76").unwrap();
    /// let skin_url = p.skin_url().unwrap();
    /// ```
    pub fn skin_url(&self) -> Result<String, MojangError> {
        if self.skin_url.borrow().is_none() {
            self.skin_url
                .replace(Some(common::get_profile(self.uuid.to_string())?.2));
        }

        Ok(self.skin_url.borrow().as_ref().unwrap().to_owned())
    }
}
