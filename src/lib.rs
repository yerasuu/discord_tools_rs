pub mod crud;
pub mod models;

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DiscordTools {
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_redirect_uri: String,
}

impl DiscordTools {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok(); // Load the .env file
        Ok(Self {
            discord_client_id: env::var("DISCORD_CLIENT_ID")?,
            discord_client_secret: env::var("DISCORD_CLIENT_SECRET")?,
            discord_redirect_uri: env::var("DISCORD_REDIRECT_URI")?,
        })
    }

    pub fn new(
        discord_client_id: String,
        discord_client_secret: String,
        discord_redirect_uri: String,
    ) -> Self {
        Self {
            discord_client_id,
            discord_client_secret,
            discord_redirect_uri,
        }
    }
}
