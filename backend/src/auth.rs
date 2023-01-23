use crate::SharedState;
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserData {
    pub aud: String,
    pub auth_time: u32,
    pub email: String,
    pub email_verified: bool,
    pub event_id: String,
    pub exp: u32,
    pub iat: u32,
    pub iss: String,
    pub jti: String,
    pub name: String,
    pub origin_jti: String,
    pub sub: String,
    pub token_use: String,
}

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

pub async fn verify_jwt(token: &str, state: SharedState) -> Option<UserData> {
    match decrypt_jwt(token, &state.keyset, &state.verifier).await {
        Ok(user) => {
            log::info!("Successful JWT Verification for user {}", user.name);
            Some(user)
        }
        Err(_) => {
            log::info!("Failed JWT Verification");
            None
        }
    }
}

pub async fn decrypt_jwt(
    token: &str,
    keyset: &jsonwebtokens_cognito::KeySet,
    verifier: &jsonwebtokens::Verifier,
) -> Result<UserData, jsonwebtokens_cognito::Error> {
    match keyset.verify(token, verifier).await {
        Ok(res) => Ok(serde_json::from_value(res).unwrap()),
        Err(err) => Err(err),
    }
}
