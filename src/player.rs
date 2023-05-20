use std::cell::{Ref, RefCell};

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
    /// List of all player name changes
    ///
    /// Due to API limitations anything before the first name change will be the accounts original name.
    name_changes: RefCell<Option<Vec<NameChange>>>,
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
                name_changes: RefCell::new(None),
            });
        }

        // If length is 16 or more input must be a UUID
        // ... or some other nonsense but thats not the point
        let (name, uuid, skin_url) = common::get_profile(name_uuid.to_string())?;
        Ok(Player {
            name,
            uuid,
            skin_url: RefCell::new(Some(skin_url)),
            name_changes: RefCell::new(None),
        })
    }

    pub fn skin_url(&self) -> Result<String, MojangError> {
        if self.skin_url.borrow().is_none() {
            self.skin_url
                .replace(Some(common::get_profile(self.uuid.to_string())?.2));
        }

        Ok(self.skin_url.borrow().as_ref().unwrap().to_owned())
    }

    /// Get play name at Timestamp (ms)
    ///
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::Player;
    ///
    /// // Load Name History Data into Player
    /// let p = Player::new("Sigma76").unwrap();
    ///
    /// // Get name at timestamp
    /// assert_eq!(p.name_at(16362446560000).unwrap(), "Sigma76");
    /// ```
    pub fn name_at(&self, time: u64) -> Result<String, MojangError> {
        if self.name_changes.borrow().is_none() {
            self.name_changes
                .replace(Some(common::get_name_history(self.uuid.to_string())?));
        }

        let nc = self.name_changes.borrow();
        let mut final_name = &self.name;
        for name in nc.as_ref().unwrap() {
            if name.time <= time {
                final_name = &name.name;
            }
        }

        Ok(final_name.to_owned())
    }
}
