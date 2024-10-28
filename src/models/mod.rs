use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordAccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Debug)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u32>,
    pub premium_type: Option<u32>,
    pub public_flags: Option<u32>,
}