use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use url::Url;

use crate::env;
use crate::errors::{AppError, AppResult};

static OAUTH2_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
static OAUTH2_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

pub async fn google_oauth() -> AppResult<(Url, CsrfToken)> {
    let client_id = ClientId::new(env::SHOKEN_WEBAPI_GOOGLE_OAUTH_CLIENT_ID.to_string());
    let client_secret =
        ClientSecret::new(env::SHOKEN_WEBAPI_GOOGLE_OAUTH_CLIENT_SECRET_ID.to_string());

    let redirect_url = RedirectUrl::new(env::SHOKEN_WEBAPI_REDIRECT_URL.to_string())
        .map_err(|e| AppError::AuthError(format!("無効なリダイレクトURL: {}", e)))?;

    let auth_url = AuthUrl::new(OAUTH2_AUTH_URL.to_string())
        .map_err(|e| AppError::AuthError(format!("無効な認証URL: {}", e)))?;

    let token_url = TokenUrl::new(OAUTH2_TOKEN_URL.to_string())
        .map_err(|e| AppError::AuthError(format!("無効なトークンURL: {}", e)))?;

    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    Ok(client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url())
}

#[cfg(test)]
mod tests {}
