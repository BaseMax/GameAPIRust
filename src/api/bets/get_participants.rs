use crate::{auth::AuthPayload, db_models};
use axum::{extract::Path, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Participant {
    pub id: u32,
    pub username: String,
    pub total_winnings: u32,
}

pub(crate) async fn get_participants(
    _auth_payload: AuthPayload,
    Path(game_id): Path<u32>,
) -> impl IntoResponse {
    let bets = db_models::Bet::get_bets();
    let users = db_models::User::get_users();

    let game_participants = bets
        .into_iter()
        .filter(|bet| bet.game_id == game_id)
        .map(|bet| {
            if bet.game_id == game_id {
                let user = users.iter().find(|user| user.id == bet.user_id);
                if let Some(user) = user {
                    return Some(Participant {
                        id: user.id,
                        username: user.username.clone(),
                        total_winnings: user.total_winnings,
                    });
                }
            }
            None
        })
        .flatten()
        .collect::<Vec<Participant>>();

    Json(game_participants)
}
