use crate::{
    auth::{AuthError, AuthPayload},
    db_models,
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct PostBetsReq {
    game_id: u32,
    expected_results: String,
    amount: u32,
}

pub(crate) async fn post_bets(
    auth_payload: AuthPayload,
    Json(req_body): Json<PostBetsReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let bets = db_models::Bet::get_bets_mut();
    let games = db_models::Game::get_games();
    let game = games.iter().find(|game| game.id == req_body.game_id);

    if let Some(game) = game {
        if db_models::get_now_iso() > game.end_time {
            return Err((
                StatusCode::NOT_ACCEPTABLE,
                Json(json!({"error": "Game is finished!"})),
            )
                .into_response());
        }
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Game not found!"})),
        )
            .into_response());
    }

    let bet_id = if let Some(bet) = bets.last() {
        bet.id + 1
    } else {
        1
    };

    let bet = db_models::Bet {
        game_id: req_body.game_id,
        user_id: auth_payload.id,
        expected_results: req_body.expected_results,
        id: bet_id,
        amount: req_body.amount,
    };
    let cloned_bet = bet.clone();
    bets.push(bet);

    Ok(Json(cloned_bet))
}
