use crate::SharedState;
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub(crate) struct UserData {
    pub(crate) auth_time: u64,
    pub(crate) client_id: String,
    #[serde(rename = "cognito:groups", default)]
    pub(crate) groups: std::collections::HashSet<String>,
    pub(crate) event_id: String,
    pub(crate) exp: u64,
    pub(crate) iat: u64,
    pub(crate) iss: String,
    pub(crate) jti: String,
    pub(crate) origin_jti: String,
    pub(crate) scope: String,
    pub(crate) sub: String,
    pub(crate) token_use: String,
    pub(crate) username: String,
}

/// Tower layer that adds an [`Option<UserData>`] as an extension to the request
/// This can be used by handlers for authentication as well as for the value
pub(crate) async fn auth<B>(
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
pub(crate) async fn verify_jwt(token: &str, state: SharedState) -> Option<UserData> {
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
