use crate::SharedState;
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize, Debug, Clone, Default)]
// pub struct UserData {
//     pub aud: String,
//     pub auth_time: u32,
//     pub email: String,
//     pub email_verified: bool,
//     pub event_id: String,
//     pub exp: u32,
//     pub iat: u32,
//     pub iss: String,
//     pub jti: String,
//     /// Username
//     /// User generated on sign-up
//     pub name: String,
//     pub origin_jti: String,
//     /// Random User ID
//     pub sub: String,
//     pub token_use: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UserData {
    pub auth_time: u64,
    pub client_id: String,
    #[serde(rename = "cognito:groups", default)]
    pub groups: std::collections::HashSet<String>,
    pub event_id: String,
    pub exp: u64,
    pub iat: u64,
    pub iss: String,
    pub jti: String,
    pub origin_jti: String,
    pub scope: String,
    pub sub: String,
    pub token_use: String,
    pub username: String,
}

/// Tower layer that adds an [`Option<UserData>`] as an extension to the request
/// This can be used by handlers for authentication as well as for the value
pub async fn auth<B>(
    State(state): State<SharedState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let mut user: Option<UserData> = None;

    if let Some(a_header) = auth_header {
        user = verify_jwt(a_header, state).await;
    }
    req.extensions_mut().insert(user);
    next.run(req).await
}

/// Interface for [`decrypt_jwt`]
pub async fn verify_jwt(token: &str, state: SharedState) -> Option<UserData> {
    match decrypt_jwt(token, &state.keyset, &state.verifier).await {
        Ok(user) => {
            log::info!("Successful JWT Verification for user {}", user.sub);
            Some(user)
        }
        Err(_) => {
            log::info!("Failed JWT Verification");
            None
        }
    }
}

/// Decrypt JWT token from Cognito
async fn decrypt_jwt(
    token: &str,
    keyset: &jsonwebtokens_cognito::KeySet,
    verifier: &jsonwebtokens::Verifier,
) -> Result<UserData, jsonwebtokens_cognito::Error> {
    match keyset.verify(token, verifier).await {
        Ok(res) => Ok(serde_json::from_value(res).unwrap()),
        Err(err) => Err(err),
    }
}
