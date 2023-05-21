use crate::{
    auth::{AuthError, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub(crate) async fn delete_bet(
    auth_payload: AuthPayload,
    Path(bet_id): Path<u32>,
) -> impl IntoResponse {
    let bets = db_models::Bet::get_bets_mut();
    bets.retain_mut(|bet| bet.id != bet_id || bet.user_id != auth_payload.id);

    Json(json!({
        "result": "success"
    }))
}
