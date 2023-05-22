use serde::Deserialize;
use uuid::Uuid;

use super::{Model, Url};
use crate::Result;

/// Profile of logged-in user
pub struct Profile {
    /// Account UUID
    pub id: Uuid,
    /// Current player name
    pub name: String,
    /// Skins uploaded to account
    pub skins: Vec<Skin>,
    /// Capes available to account
    pub capes: Vec<Url>,
}

/// An accounts skin
pub struct Skin {
    /// ID of the skin
    pub id: String,
    /// Is the skin active or disabled
    pub state: SkinState,
    /// The url of the skin image
    pub url: Url,
    /// The model type of the skin
    pub variant: Model,
}

/// State of a user's skin
pub enum SkinState {
    /// This is the current skin enabled
    Active,
    /// This skin is disabled
    Disabled,
}

/// Gets information on the profile you are logged in with.
///
/// Mojang API Docs: https://wiki.vg/Mojang_API#Profile_Information
pub fn profile_information(access_token: &str) -> Result<Profile> {
    #[derive(Deserialize)]
    struct Response {
        id: String,
        name: String,
        skins: Vec<ResponseSkin>,
        capes: Vec<String>,
    }

    #[derive(Deserialize)]
    struct ResponseSkin {
        id: String,
        state: String,
        url: String,
        variant: String,
    }

    let resp = ureq::get("https://api.minecraftservices.com/minecraft/profile")
        .set("Authorization", &format!("Bearer {}", access_token))
        .call()?
        .into_json::<Response>()?;

    Ok(Profile {
        id: resp.id.parse()?,
        name: resp.name,
        skins: resp
            .skins
            .into_iter()
            .map(|x| Skin {
                id: x.id,
                state: x.state.into(),
                url: Url(x.url),
                variant: x.variant.into(),
            })
            .collect(),
        capes: resp.capes.into_iter().map(|x| Url(x)).collect(),
    })
}

impl From<String> for SkinState {
    fn from(value: String) -> Self {
        if value == "ACTIVE" {
            return SkinState::Active;
        }

        SkinState::Disabled
    }
}
