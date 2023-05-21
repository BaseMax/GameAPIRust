use crate::{
    auth::{self, AuthError, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct PutProfileReqBody {
    pub username: Option<String>,
    pub password: Option<String>,
}

pub(crate) async fn put_profile(
    auth_payload: AuthPayload,
    Json(req_body): Json<PutProfileReqBody>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user = auth::get_user_by_auth_payload(&auth_payload);

    if let Some(username) = req_body.username {
        let users = db_models::User::get_users();
        let user_with_specified_username = users.iter().find(|user| user.username == username);

        match user_with_specified_username {
            None => user.username = username,
            Some(user_with_specified_username) => {
                if user_with_specified_username.id != user.id {
                    return Err((
                        StatusCode::CONFLICT,
                        Json(json!({"error": "Username is in use!"})),
                    )
                        .into_response());
                }
            }
        }
    }
    if let Some(password) = req_body.password {
        user.password = password;

        user.refresh_token().map_err(|err| err.into_response())?;
    }

    Ok(Json(user.clone()))
}
