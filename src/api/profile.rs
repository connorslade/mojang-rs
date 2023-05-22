use std::collections::HashMap;

use serde::{
    de::{self, Visitor},
    Deserialize,
};
use serde_json::Value;
use uuid::Uuid;

use crate::Result;

use super::Url;

/*
{
    "id": "<profile identifier>",
    "name": "<player name>",
    "properties": [
        {
            "name": "textures",
            "value": "<base64 string>",
            "signature": "<base64 string; signed data using Yggdrasil's private key>" // Only provided if ?unsigned=false is appended to url
        }
    ]
}
*/

/*
{
    "timestamp": <java time in ms>,
    "profileId": "<profile uuid>",
    "profileName": "<player name>",
    "signatureRequired": true, // Only present if ?unsigned=false is appended to url
    "textures": {
        "SKIN": {
            "url": "<player skin URL>",
            "metadata": {
                "model": "slim"
            }
        },
        "CAPE": {
            "url": "<player cape URL>"
        }
    }
}
*/

#[derive(Debug)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub skin: Skin,
}

#[derive(Debug)]
pub struct Skin {
    pub url: Url,
    pub model: Model,
    pub cape: Option<Url>,
}

#[derive(Debug)]
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

                Ok(Profile {
                    id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                    name: name.ok_or_else(|| de::Error::missing_field("name"))?,
                    skin: serde_json::from_str(&decoded).map_err(|x| {
                        de::Error::custom(format!("Error deserializing skin: {}", x.to_string()))
                    })?,
                })
            }
        }

        const FIELDS: &[&str] = &["id", "name", "properties"];
        des.deserialize_struct("Profile", FIELDS, ProfileVisitor)
    }
}

impl<'de> Deserialize<'de> for Skin {
    fn deserialize<D>(des: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SkinVisitor;

        impl<'de> Visitor<'de> for SkinVisitor {
            type Value = Skin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Skin")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                while let Some((_, _)) = map.next_entry::<Value, Value>()? {}

                Ok(Skin {
                    url: Url("".to_owned()),
                    model: Model::Normal,
                    cape: None,
                })
            }
        }

        const FIELDS: &[&str] = &["skin", "model", "cape"];
        des.deserialize_struct("Skin", FIELDS, SkinVisitor)
    }
}
