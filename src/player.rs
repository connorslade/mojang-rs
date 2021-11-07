use crate::common;
use crate::MojangError;

/// A Player...
///
/// Not much more to say
#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    /// Player Name
    pub name: String,
    /// Player UUID
    ///
    /// All lowercase with no dashes
    pub uuid: String,

    /// Url of current player skin
    pub skin_url: Option<String>,
    /// List of all player name changes
    ///
    /// Due to API limitations anything before the first name change will be the accounts original name.
    pub name_changes: Option<Vec<(u64, String)>>,
}

impl Player {
    /// Make a new player
    ///
    /// You can supply the Name or UUID
    ///
    /// If you supply the uuid you will automaticly get the skin URL
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

    /// Add Skin URL to a player
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::Player;
    ///
    /// // Load Skin Data into Player
    /// let p = Player::new("Sigma76").unwrap().add_skin().unwrap();
    /// ```
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

    /// Add Name History Data to a Player
    ///
    /// Required if you want to use `player.name_at(n)`
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::Player;
    ///
    /// // Load Name History Data into Player
    /// let p = Player::new("Sigma76").unwrap().add_name_change().unwrap();
    /// ```
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

    /// Get play name at Timestamp (ms)
    ///
    /// You must have called `add_name_change` on the player before useing this
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::Player;
    ///
    /// // Load Name History Data into Player
    /// let p = Player::new("Sigma76").unwrap().add_name_change().unwrap();
    ///
    /// // Get name at timestamp
    /// assert_eq!(p.name_at(16362446560000).unwrap(), "Sigma76");
    /// ```
    pub fn name_at(&self, time: u64) -> Result<String, MojangError> {
        if self.name_changes.is_none() {
            return Err(MojangError::NotEnoughData);
        }

        let mut final_name = self.name.clone();
        for name in self.name_changes.clone().unwrap() {
            if name.0 <= time {
                final_name = name.1;
            }
        }

        Ok(final_name)
    }
}
