use std::{collections::HashMap, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize,
};
use uuid::Uuid;

use super::{Url, Model};
use crate::Result;

/// A player's public profile
#[derive(Debug)]
pub struct Profile {
    /// The player's UUID
    pub id: Uuid,
    /// The player's current username
    pub name: String,
    /// The player's skin
    pub skin: Skin,
}

/// A player's skin.
/// Includes the skin and cape URLs, as well as the model type.
#[derive(Debug)]
pub struct Skin {
    /// Skin URL
    pub url: Url,
    /// Player model type
    pub model: Model,
    /// Optional cape URL
    pub cape: Option<Url>,
}

/// This will return the player's username plus any additional information about them (e.g. skins).
/// Example: https://sessionserver.mojang.com/session/minecraft/profile/3c358264-b456-4bde-ab1e-fe1023db6679
/// This rate limit is around 200 requests per minute.
///
/// Mojang API docs [here](https://wiki.vg/Mojang_API#UUID_to_Profile_and_Skin.2FCape).
pub fn uuid_to_profile(uuid: Uuid) -> Result<Profile> {
    let resp = ureq::get(&format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid.to_string()
    ))
    .call()?
    .into_string()?;

    let profile: Profile = serde_json::from_str(&resp)?;
    Ok(profile)
}

impl<'de> Deserialize<'de> for Profile {
    fn deserialize<D>(des: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TextureContainer {
            textures: Textures,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "UPPERCASE")]
        struct Textures {
            skin: SkinTexture,
            cape: Option<CapeTexture>,
        }

        #[derive(Deserialize)]
        struct SkinTexture {
            url: String,
            metadata: Option<SkinMetadata>,
        }

        #[derive(Deserialize)]
        struct CapeTexture {
            url: String,
        }

        #[derive(Deserialize)]
        struct SkinMetadata {
            model: String,
        }

        struct ProfileVisitor;

        impl<'de> Visitor<'de> for ProfileVisitor {
            type Value = Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Profile")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut properties = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "id" => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }

                            id = Some(map.next_value()?);
                        }
                        "name" => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }

                            name = Some(map.next_value()?);
                        }
                        "properties" => {
                            if properties.is_some() {
                                return Err(de::Error::duplicate_field("properties"));
                            }

                            properties = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                    }
                }

                let properties: Vec<HashMap<String, String>> =
                    properties.ok_or_else(|| de::Error::missing_field("properties"))?;
                let textures = properties
                    .get(0)
                    .ok_or_else(|| de::Error::missing_field("properties"))?;
                let value = textures
                    .get("value")
                    .ok_or_else(|| de::Error::missing_field("properties[0].value"))?;
                let decoded = base64::decode(value)
                    .map_err(|e| {
                        de::Error::custom(format!(
                            "Error decoding base64 string: {}",
                            e.to_string()
                        ))
                    })
                    .map(|x| String::from_utf8_lossy(&x).into_owned())?;

                println!("{}", decoded);

                let textures = serde_json::from_str::<TextureContainer>(&decoded)
                    .map_err(|x| {
                        de::Error::custom(format!("Error deserializing skin: {}", x.to_string()))
                    })?
                    .textures;

                let model = textures
                    .skin
                    .metadata
                    .map_or(Model::Normal, |x| x.model.parse().unwrap_or(Model::Normal));

                Ok(Profile {
                    id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                    name: name.ok_or_else(|| de::Error::missing_field("name"))?,
                    skin: Skin {
                        url: Url(textures.skin.url),
                        model,
                        cape: textures.cape.map(|x| Url(x.url)),
                    },
                })
            }
        }

        const FIELDS: &[&str] = &["id", "name", "properties"];
        des.deserialize_struct("Profile", FIELDS, ProfileVisitor)
    }
}

impl FromStr for Model {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "slim" => Ok(Model::Slim),
            "normal" => Ok(Model::Normal),
            _ => Err(format!("Unknown model: {}", s)),
        }
    }
}
