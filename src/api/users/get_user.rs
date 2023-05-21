use crate::{
    auth::{AuthError, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct GetUserResBody {
    pub id: u32,
    pub username: String,
    pub total_winnings: u32,
}

pub(crate) async fn get_user(
    _auth_payload: AuthPayload,
    Path(user_id): Path<u32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let users = db_models::User::get_users();
    let user = users.iter().find(|user| user.id == user_id);

    match user {
        Some(user) => Ok(Json(GetUserResBody {
            id: user.id,
            username: user.username.clone(),
            total_winnings: user.total_winnings,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found!"})),
        )
            .into_response()),
    }
}
