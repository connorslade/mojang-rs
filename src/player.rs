use std::cell::RefCell;

use uuid::Uuid;

/// A public Minecraft account.
#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    /// Player Name
    pub name: String,
    /// Player UUID
    ///
    /// All lowercase with no dashes
    pub id: Uuid,

    /// Url of current player skin
    skin_url: RefCell<Option<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NameChange {
    pub time: u64,
    pub name: String,
}

impl Player {}
