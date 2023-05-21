use crate::{
    auth::{AuthError, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct PutBetReqBody {
    pub amount: Option<u32>,
}

pub(crate) async fn put_bet(
    _auth_payload: AuthPayload,
    Path(bet_id): Path<u32>,
    Json(req_body): Json<PutBetReqBody>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let bets = db_models::Bet::get_bets_mut();
    let bet = bets.into_iter().find(|bet| bet.id == bet_id);

    if let Some(bet) = bet {
        let games = db_models::Game::get_games();
        let game = games.iter().find(|game| game.id == bet.game_id);

        if let Some(game) = game {
            if db_models::get_now_iso() > game.end_time {
                return Err((
                    StatusCode::NOT_ACCEPTABLE,
                    Json(json!({"error": "Game is finished!"})),
                )
                    .into_response());
            }

            if let Some(amount) = req_body.amount {
                bet.amount = amount
            }

            Ok(Json(bet))
        } else {
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Game not found!"})),
            )
                .into_response())
        }
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Bet not found!"})),
        )
            .into_response())
    }
}
