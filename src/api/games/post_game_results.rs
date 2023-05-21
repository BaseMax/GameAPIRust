use crate::{
    auth::{self, AuthError, AuthPayload},
    db_models,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub(crate) struct PostGamesReq {
    results: String,
}

pub(crate) async fn post_game_results(
    auth_payload: AuthPayload,
    Path(game_id): Path<u32>,
    Json(req_body): Json<PostGamesReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user = auth::get_user_by_auth_payload(&auth_payload);
    if !user.is_superuser {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Access denied!"})),
        )
            .into_response());
    }

    let bets = db_models::Bet::get_bets();
    let users = db_models::User::get_users_mut();
    let games = db_models::Game::get_games_mut();
    let game = games.into_iter().find(|game| game.id == game_id);

    match game {
        Some(game) => {
            if game.results.is_none() {
                if db_models::get_now_iso() < game.end_time {
                    return Err((
                        StatusCode::NOT_ACCEPTABLE,
                        Json(json!({"error": "Game is not finished yet!"})),
                    )
                        .into_response());
                }

                let winner_bets = bets
                    .iter()
                    .filter(|bet| {
                        bet.game_id == game_id && bet.expected_results == req_body.results
                    })
                    .collect::<Vec<&db_models::Bet>>();

                winner_bets.iter().for_each(|bet| {
                    let winner_user = users.into_iter().find(|user| user.id == bet.user_id);

                    if let Some(winner_user) = winner_user {
                        winner_user.balance += bet.amount as u64;
                    }
                });

                game.results = Some(req_body.results);
            }
        }
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Game not found!"})),
            )
                .into_response())
        }
    }

    Ok(Json(json!({"results": "success"})))
}
