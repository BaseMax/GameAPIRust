// Here we validate tokens and parse them.

use crate::{db::models::User, db_models};
use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{
    self, DecodingKey as JwtDecodingKey, EncodingKey as JwtEncodingKey, Validation as JwtValidation,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub(crate) static mut JWT_ENCODING_KEY: Option<JwtEncodingKey> = None;
pub(crate) static mut JWT_DECODING_KEY: Option<JwtDecodingKey> = None;
pub fn init_auth_keys(secret: String) {
    unsafe {
        JWT_ENCODING_KEY = Some(JwtEncodingKey::from_secret(secret.as_bytes()));
        JWT_DECODING_KEY = Some(JwtDecodingKey::from_secret(secret.as_bytes()));
    }
}

#[derive(Debug)]
pub(crate) enum AuthError {
    InvalidToken,
    MissingCredentials,
    TokenCreation,
    WrongCredentials,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthPayload {
    pub id: u32,
    pub exp: u128,
}
#[async_trait]
impl<S> FromRequestParts<S> for AuthPayload
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // Decode the user data
        let token_data = jsonwebtoken::decode::<AuthPayload>(
            bearer.token(),
            unsafe { JWT_DECODING_KEY.as_ref().unwrap() },
            &JwtValidation::new(jsonwebtoken::Algorithm::HS512),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        let users = db_models::User::get_users();
        let user = users
            .iter()
            .find(|user| user.id == token_data.claims.id && user.token == bearer.token());

        match user {
            Some(_) => Ok(token_data.claims),
            None => Err(AuthError::InvalidToken),
        }
    }
}

pub(crate) fn get_user_by_auth_payload(auth_payload: &AuthPayload) -> &mut User {
    let users = db_models::User::get_users_mut();

    users
        .iter_mut()
        .find(|user| user.id == auth_payload.id)
        .unwrap()
}
