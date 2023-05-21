use crate::{auth::AuthError, db_models};
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthenticateReq {
    username: String,
    password: String,
}
#[derive(Serialize)]
pub struct AuthenticateRes {
    pub access_token: String,
    pub token_type: String,
}

pub(crate) async fn authenticate(
    Json(payload): Json<AuthenticateReq>,
) -> Result<impl IntoResponse, AuthError> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let users = db_models::User::get_users_mut();
    let user = users
        .iter_mut()
        .find(|user| user.username == payload.username && user.password == payload.password);

    match user {
        Some(user) => {
            user.refresh_token()?;

            Ok(Json(AuthenticateRes {
                access_token: user.token.clone(),
                token_type: String::from("Bearer"),
            }))
        }
        None => Err(AuthError::WrongCredentials),
    }
}
