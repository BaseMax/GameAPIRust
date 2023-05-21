use crate::{auth::AuthError, db_models};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
pub struct PostUserReqBody {
    username: String,
    password: String,
}
#[derive(Serialize)]
pub struct PostUserResBody {
    pub access_token: String,
    pub token_type: String,
}

pub(crate) async fn post_user(
    Json(payload): Json<PostUserReqBody>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials.into_response());
    }

    let users = db_models::User::get_users_mut();
    let user = users
        .iter_mut()
        .find(|user| user.username == payload.username);
    let access_token = match user {
        Some(user) => {
            if payload.password != user.password {
                return Err((
                    StatusCode::NOT_ACCEPTABLE,
                    Json(serde_json::json!({
                        "error": "User exists and password is wrong!"
                    })),
                )
                    .into_response());
            }

            user.refresh_token().map_err(|err| err.into_response())?;

            user.token.clone()
        }
        None => {
            let user_id = if let Some(user) = users.last() {
                user.id + 1
            } else {
                1
            };
            let user = db_models::User::new(payload.username, payload.password, user_id)
                .map_err(|err| err.into_response())?;
            let access_token = user.token.clone();

            users.push(user);

            access_token
        }
    };

    Ok(Json(PostUserResBody {
        access_token,
        token_type: String::from("Bearer"),
    }))
}
