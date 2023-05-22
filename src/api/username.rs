use serde::Deserialize;
use serde_json::json;
use urlencoding;
use uuid::Uuid;

use crate::{MojangError, Result};

#[derive(Deserialize)]
struct Response {
    id: String,
}

/// Gets the UUID of a player from their username.
/// Example: https://api.mojang.com/users/profiles/minecraft/Notch
///
/// Mojang API docs [here](https://wiki.vg/Mojang_API#Username_to_UUID).
pub fn username_to_uuid(username: &str) -> Result<Uuid> {
    let resp = ureq::get(&format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        urlencoding::encode(username)
    ))
    .call()?
    .into_json::<Response>()?;

    let uuid = Uuid::parse_str(&resp.id)?;
    Ok(uuid)
}

/// Gets the UUIDs of multiple players from their usernames.
/// You *MUST* provide between 1 and 10 usernames.
///
/// Mojang API docs [here](https://wiki.vg/Mojang_API#Username_to_UUID).
pub fn usernames_to_uuids(usernames: &[&str]) -> Result<Vec<Uuid>> {
    if !(1..=10).contains(&usernames.len()) {
        return Err(MojangError::InvalidRequest(
            "Must provide between 1 and 10 usernames".to_string(),
        ));
    }

    let payload = json!(usernames);

    let resp = ureq::post("https://api.mojang.com/profiles/minecraft")
        .send_json(payload)?
        .into_json::<Vec<Response>>()?;

    let mut uuids = Vec::with_capacity(resp.len());
    for response in resp {
        let uuid = Uuid::parse_str(&response.id)?;
        uuids.push(uuid);
    }

    Ok(uuids)
}
