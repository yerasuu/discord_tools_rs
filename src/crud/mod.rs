use reqwest::Client;
use std::collections::HashMap;

use crate::{
    models::{DiscordAccessToken, DiscordUser},
    DiscordTools,
};

impl DiscordTools {
    pub async fn get_token(self, authorization_code: String) -> Option<DiscordAccessToken> {
        Self::get_discord_token(
            self.discord_client_id,
            self.discord_client_secret,
            self.discord_redirect_uri,
            authorization_code,
        )
        .await
    }

    pub fn get_token_sync(self, authorization_code: String) -> Option<DiscordAccessToken> {
        Self::get_discord_token_sync(
            self.discord_client_id,
            self.discord_client_secret,
            self.discord_redirect_uri,
            authorization_code,
        )
    }

    pub async fn get_user_info(access_token: &str) -> Result<DiscordUser, reqwest::Error> {
        let client = Client::new();
        let response = client
            .get("https://discord.com/api/v10/users/@me")
            .bearer_auth(access_token)
            .send()
            .await?;

        let user: DiscordUser = response.json::<DiscordUser>().await?;
        Ok(user)
    }

    pub fn get_user_info_sync(access_token: &str) -> Result<DiscordUser, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get("https://discord.com/api/v10/users/@me")
            .bearer_auth(access_token)
            .send()?;

        let user: DiscordUser = response.json()?;
        Ok(user)
    }

    pub async fn get_discord_token(
        discord_client_id: String,
        discord_client_secret: String,
        discord_redirect_uri: String,
        authorization_code: String,
    ) -> Option<DiscordAccessToken> {
        // Obtén las credenciales de la aplicación Discord desde las variables de entorno
        let client_id = discord_client_id;
        let client_secret = discord_client_secret;
        let redirect_uri = discord_redirect_uri;

        let authorization_code_str = "authorization_code".to_string();

        // Intercambia el código de autorización por un token de acceso
        let mut params: HashMap<&str, &String> = HashMap::new();
        params.insert("client_id", &client_id);
        params.insert("client_secret", &client_secret);
        params.insert("grant_type", &authorization_code_str);
        params.insert("code", &authorization_code);
        params.insert("redirect_uri", &redirect_uri);

        let client = Client::new();
        let discord_token_url = "https://discord.com/api/oauth2/token";

        let response = client
            .post(discord_token_url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(serde_urlencoded::to_string(&params).expect("Failed to encode form data"))
            .send()
            .await
            .expect("Failed to send token request");

        if response.status().is_success() {
            let token: DiscordAccessToken = response
                .json()
                .await
                .expect("Failed to parse JSON response");
            Some(token)
        } else {
            None
        }
    }

    pub fn get_discord_token_sync(
        discord_client_id: String,
        discord_client_secret: String,
        discord_redirect_uri: String,
        authorization_code: String,
    ) -> Option<DiscordAccessToken> {
        // Obtén las credenciales de la aplicación Discord desde las variables de entorno
        let client_id = discord_client_id;
        let client_secret = discord_client_secret;
        let redirect_uri = discord_redirect_uri;

        let authorization_code_str = "authorization_code".to_string();

        // Intercambia el código de autorización por un token de acceso
        let mut params: HashMap<&str, &String> = HashMap::new();
        params.insert("client_id", &client_id);
        params.insert("client_secret", &client_secret);
        params.insert("grant_type", &authorization_code_str);
        params.insert("code", &authorization_code);
        params.insert("redirect_uri", &redirect_uri);

        let client = reqwest::blocking::Client::new();
        let discord_token_url = "https://discord.com/api/oauth2/token";

        let response = client
            .post(discord_token_url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(serde_urlencoded::to_string(&params).expect("Failed to encode form data"))
            .send()
            .expect("Failed to send token request");

        if response.status().is_success() {
            let token: DiscordAccessToken = response.json().expect("Failed to parse JSON response");
            Some(token)
        } else {
            None
        }
    }
}
