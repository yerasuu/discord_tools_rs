# discord_tools_rs

Simple Discord Helper


## ENV

In order to use the env based config we need an env with the following vars

```env
DISCORD_CLIENT_ID=
DISCORD_CLIENT_SECRET=
DISCORD_REDIRECT_URI=http://localhost:8080/redirection-uri
ACCESS_TOKEN=
```

## Example

Instance using dot env file

```rust
let config_result = DiscordTools::from_env();
let discord_tools = match config_result {
    Ok(config) => {
        println!("Config loaded successfully: {:?}", config);
        config
    }
    Err(e) => {
        eprintln!("Failed to load config: {}", e);
        return; // Exit if the config cannot be loaded
    }
};
```

Instance using params

```rust
let config_result = DiscordTools::new(discord_client_id,discord_client_secret,discord_redirect_uri);
let discord_tools = match config_result {
    Ok(config) => {
        println!("Config loaded successfully: {:?}", config);
        config
    }
    Err(e) => {
        eprintln!("Failed to load config: {}", e);
        return; // Exit if the config cannot be loaded
    }
};
```

Get access token

```rust
let discord_authorize_url = format!(
    "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri{}&response_type=code&scope=identify",
    client_id, redirect_uri
);
match discord_tools.get_token(authorization_code).await {
    Some(token) => {
        println!("Access Token: {}", token.access_token);
    }
    None => {
        println!("Failed to retrieve access token");
    }
}
```

Get user info

```rust
match discord_tools.get_user_info(&access_token).await {
    Ok(user) => {
        println!("User Info: {:?}", user);
    }
    Err(e) => eprintln!("Error fetching user info: {:?}", e),
}
```