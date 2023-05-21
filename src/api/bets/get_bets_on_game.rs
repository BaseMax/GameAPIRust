use crate::{auth::AuthPayload, db_models};
use axum::{extract::Path, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct GetBetsOnGameRes<'a> {
    pub bets: Vec<&'a db_models::Bet>,
}

pub(crate) async fn get_bets_on_game(
    _auth_payload: AuthPayload,
    Path(game_id): Path<u32>,
) -> impl IntoResponse {
    let bets = db_models::Bet::get_bets();
    let user_bets = bets
        .into_iter()
        .filter(|bet| bet.game_id == game_id)
        .collect::<Vec<&db_models::Bet>>();

    Json(GetBetsOnGameRes { bets: user_bets })
}
