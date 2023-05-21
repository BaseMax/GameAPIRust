use crate::{
    auth::{self, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

pub(crate) async fn delete_profile(auth_payload: AuthPayload) -> impl IntoResponse {
    let bets = db_models::Bet::get_bets_mut();
    bets.retain_mut(|bet| bet.user_id != auth_payload.id);

    let users = db_models::User::get_users_mut();
    users.retain_mut(|user| user.id != auth_payload.id);

    Json(json!({
        "result": "success"
    }))
}
