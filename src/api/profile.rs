use serde::{de::Visitor, Deserialize};
use uuid::Uuid;

use crate::Result;

pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub skin: Skin,
    pub cake: Option<Cape>,
}

pub struct Cape {
    pub url: String,
}

pub struct Skin {
    pub url: String,
    pub model: Model,
}

pub enum Model {
    Slim,
    Normal,
}

/// This will return the player's username plus any additional information about them (e.g. skins).
/// Example: https://sessionserver.mojang.com/session/minecraft/profile/3c358264-b456-4bde-ab1e-fe1023db6679
/// This rate limit is around 200 requests per minute.
///
/// Mojang API docs [here](https://wiki.vg/Mojang_API#UUID_to_Profile_and_Skin.2FCape).
pub fn uuid_to_profile(uuid: Uuid) -> Result<Profile> {
    todo!()
}

impl<'de> Deserialize<'de> for Profile {
    fn deserialize<D>(des: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ProfileVisitor;

        impl<'de> Visitor<'de> for ProfileVisitor {
            type Value = Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }

            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                todo!()
            }
        }

        des.deserialize_any(ProfileVisitor)
    }
}
