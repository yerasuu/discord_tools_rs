use dotenvy::dotenv;
use reqwest::Client;
use std::collections::HashMap;

use crate::{
    models::{DiscordAccessToken, DiscordUser},
    DiscordTools,
};

impl DiscordTools {
    pub async fn get_token(self, authorization_code: String) -> Option<DiscordAccessToken> {
        dotenv().ok();
        // Obtén las credenciales de la aplicación Discord desde las variables de entorno
        let client_id = self.discord_client_id;
        let client_secret = self.discord_client_secret;
        let redirect_uri = self.discord_redirect_uri;

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

    pub async fn get_user_info(
        self,
        access_token: &str,
    ) -> Result<DiscordUser, reqwest::Error> {
        let client = Client::new();
        let response = client
            .get("https://discord.com/api/v10/users/@me")
            .bearer_auth(access_token)
            .send()
            .await?;

        let user: DiscordUser = response.json::<DiscordUser>().await?;
        Ok(user)
    }
}
