#[cfg(feature = "timeout")]
use std::time::Duration;

use tinyjson::JsonValue;
use ureq::Agent;
use ureq::AgentBuilder;

use crate::player::NameChange;
use crate::MojangError;

/// Default Agent for all Requests
pub fn ureq_agent() -> Agent {
    #[cfg(feature = "timeout")]
    return AgentBuilder::new()
        .timeout(Duration::from_secs(4))
        .user_agent("rust-mojang/0.1.0")
        .build();

    #[cfg(not(feature = "timeout"))]
    return AgentBuilder::new().user_agent("rust-mojang/0.1.0").build();
}

/// Rerutns (name, uuid, skin_url)
pub fn get_profile(uuid: String) -> Result<(String, String, String), MojangError> {
    let agent = ureq_agent();
    let json = match agent
        .get(&format!(
            "https://sessionserver.mojang.com/session/minecraft/profile/{}",
            uuid.replace("-", "").to_lowercase()
        ))
        .call()
    {
        Ok(i) => i.into_string().unwrap().parse::<JsonValue>().unwrap(),
        Err(e) => return Err(MojangError::RequestError(e)),
    };

    let name = match &json["name"] {
        JsonValue::String(i) => i.to_string(),
        _ => return Err(MojangError::ParseError),
    };

    let uuid = match &json["id"] {
        JsonValue::String(i) => i.to_string(),
        _ => return Err(MojangError::ParseError),
    };

    let raw_skin = match &json["properties"][0]["value"] {
        JsonValue::String(i) => match base64::decode(i.to_string()) {
            Ok(i) => (*String::from_utf8_lossy(&i)).to_string(),
            _ => return Err(MojangError::ParseError),
        },
        _ => return Err(MojangError::ParseError),
    };

    let skin = match parse_skin_json(raw_skin) {
        Some(i) => i,
        None => return Err(MojangError::ParseError),
    };

    Ok((name, uuid, skin))
}

/// Rerutns (name, uuid)
pub fn get_uuid(name: String) -> Result<(String, String), MojangError> {
    let agent = ureq_agent();
    match agent
        .get(&format!(
            "https://api.mojang.com/users/profiles/minecraft/{}",
            name
        ))
        .call()
    {
        Ok(i) => {
            let json = &i.into_string().unwrap().parse::<JsonValue>().unwrap();

            let name = match &json["name"] {
                JsonValue::String(i) => i.to_owned(),
                _ => return Err(MojangError::ParseError),
            };

            let uuid = match &json["id"] {
                JsonValue::String(i) => i.to_owned(),
                _ => return Err(MojangError::ParseError),
            };

            Ok((name, uuid))
        }

        Err(e) => Err(MojangError::RequestError(e)),
    }
}

pub fn get_name_history(uuid: String) -> Result<Vec<NameChange>, MojangError> {
    let agent = ureq_agent();
    let json = match agent
        .get(&format!(
            "https://api.mojang.com/user/profiles/{}/names",
            uuid.replace("-", "").to_lowercase()
        ))
        .call()
    {
        Ok(i) => i.into_string().unwrap().parse::<JsonValue>().unwrap(),
        Err(e) => return Err(MojangError::RequestError(e)),
    };

    let names = match json {
        JsonValue::Array(i) => i,
        _ => return Err(MojangError::ParseError),
    };

    let mut names_out = Vec::new();
    for name_json in names {
        let name = match &name_json["name"] {
            JsonValue::String(i) => i.to_owned(),
            _ => return Err(MojangError::ParseError),
        };

        let time = match &name_json {
            JsonValue::Object(i) => match i.get("changedToAt") {
                Some(i) => match i {
                    JsonValue::Number(i) => *i as u64,
                    _ => return Err(MojangError::ParseError),
                },
                None => 0,
            },
            _ => return Err(MojangError::ParseError),
        };

        names_out.push(NameChange { name, time });
    }

    Ok(names_out)
}

fn parse_skin_json(raw: String) -> Option<String> {
    let json = raw.parse::<JsonValue>().ok()?;
    let url = match &json["textures"]["SKIN"]["url"] {
        JsonValue::String(i) => i,
        _ => return None,
    };

    Some(url.to_owned())
}
