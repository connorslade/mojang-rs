#[cfg(feature = "timeout")]
use std::time::Duration;

use tinyjson::JsonValue;
use ureq::Agent;
use ureq::AgentBuilder;

use crate::MojangError;

const USER_AGENT: &str = concat!("mojang-rs/", env!("CARGO_PKG_VERSION"));

/// Default Agent for all Requests
pub fn ureq_agent() -> Agent {
    #[cfg(feature = "timeout")]
    return AgentBuilder::new()
        .timeout(Duration::from_secs(4))
        .user_agent(USER_AGENT)
        .build();

    #[cfg(not(feature = "timeout"))]
    return AgentBuilder::new().user_agent(USER_AGENT).build();
}

/// Returns (name, uuid, skin_url)
pub fn get_profile(uuid: String) -> Result<(String, String, String), MojangError> {
    let agent = ureq_agent();
    let json = match agent
        .get(&format!(
            "https://sessionserver.mojang.com/session/minecraft/profile/{}",
            uuid.replace('-', "").to_lowercase()
        ))
        .call()
    {
        Ok(i) => i.into_string().unwrap().parse::<JsonValue>().unwrap(),
        Err(e) => return Err(MojangError::RequestError(Box::new(e))),
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
        JsonValue::String(i) => match base64::decode(i) {
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

/// Returns (name, uuid)
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

        Err(e) => Err(MojangError::RequestError(Box::new(e))),
    }
}

fn parse_skin_json(raw: String) -> Option<String> {
    let json = raw.parse::<JsonValue>().ok()?;
    let url = match &json["textures"]["SKIN"]["url"] {
        JsonValue::String(i) => i,
        _ => return None,
    };

    Some(url.to_owned())
}
