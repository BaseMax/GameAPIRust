use crate::{
    auth::{AuthError, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

pub(crate) async fn get_game_results(
    _auth_payload: AuthPayload,
    Path(game_id): Path<u32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let games = db_models::Game::get_games();
    let game = games.iter().find(|game| game.id == game_id);

    match game {
        Some(game) => Ok(Json(game)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Game not found!"})),
        )
            .into_response()),
    }
}
