use dotenv::dotenv;
use std::env;
use rocket::config::ConfigError;

pub struct AuthSettings {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    auth0_domain: String,
}

impl AuthSettings {
    pub fn from_env () -> Result<AuthSettings, ConfigError> {
        dotenv().ok();

        let client_id = env::var("AUTH0_CLIENT_ID").map_err(|_| ConfigError::NotFound)?;
        let client_secret = env::var("AUTH0_CLIENT_SECRET").map_err(|_| ConfigError::NotFound)?;
        let redirect_uri = env::var("AUTH0_REDIRECT_URI").map_err(|_| ConfigError::NotFound)?;
        let auth0_domain = env::var("AUTH0_DOMAIN").map_err(|_| ConfigError::NotFound)?;

        Ok(AuthSettings {
            client_id,
            client_secret,
            redirect_uri,
            auth0_domain,
        })
    }
}
