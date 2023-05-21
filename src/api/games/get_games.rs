use crate::{auth::AuthPayload, db_models};
use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct GetGamesRes<'a> {
    pub games: &'a Vec<db_models::Game>,
}

pub(crate) async fn get_games(_auth_payload: AuthPayload) -> impl IntoResponse {
    let games = db_models::Game::get_games();

    Json(GetGamesRes { games })
}
