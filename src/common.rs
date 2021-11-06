use std::time::Duration;

use tinyjson::JsonValue;
use ureq::Agent;
use ureq::AgentBuilder;

use crate::MojangError;

/// Defult Agent for all Requests
pub fn ureq_agent() -> Agent {
    AgentBuilder::new()
        .timeout(Duration::from_secs(4))
        .user_agent("rust-mojang/0.0.0")
        .build()
}

/// Rerutns (name, uuid)
pub fn uuid_to_name(uuid: String) -> Result<(String, String), MojangError> {
    let agent = ureq_agent();
    let json = match agent
        .get(&format!(
            "https://sessionserver.mojang.com/session/minecraft/profile/{}",
            uuid.replace("-", "").to_lowercase()
        ))
        .call()
    {
        Ok(i) => i.into_string().unwrap().parse::<JsonValue>().unwrap(),
        Err(_) => return Err(MojangError::RequestError),
    };

    let name = match &json["name"] {
        JsonValue::String(i) => i.to_string(),
        _ => return Err(MojangError::ParseError),
    };

    let uuid = match &json["id"] {
        JsonValue::String(i) => i.to_string(),
        _ => return Err(MojangError::ParseError),
    };

    return Ok((name, uuid));
}

/// Rerutns (name, uuid)
pub fn name_to_uuid(name: String) -> Result<(String, String), MojangError> {
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
                JsonValue::String(i) => i.to_string(),
                _ => return Err(MojangError::ParseError),
            };

            let uuid = match &json["id"] {
                JsonValue::String(i) => i.to_string(),
                _ => return Err(MojangError::ParseError),
            };

            return Ok((name, uuid));
        }

        Err(_) => return Err(MojangError::RequestError),
    };
}
