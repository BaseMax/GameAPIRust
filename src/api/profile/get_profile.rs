use crate::{
    auth::{self, AuthError, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct GetProfileResBody {
    pub id: u32,
    pub balance: u64,
    pub total_winnings: u32,
    pub username: String,
}

pub(crate) async fn get_profile(auth_payload: AuthPayload) -> impl IntoResponse {
    let user = auth::get_user_by_auth_payload(&auth_payload);

    Json(GetProfileResBody {
        id: user.id,
        balance: user.balance,
        total_winnings: user.total_winnings,
        username: user.username.clone(),
    })
}
