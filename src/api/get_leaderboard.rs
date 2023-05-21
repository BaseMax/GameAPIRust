use crate::{auth::AuthPayload, db_models};
use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct LeaderBoardUser {
    pub user_id: u32,
    pub username: String,
    pub total_winnings: u32,
}
#[derive(Serialize)]
pub(crate) struct GetLeaderBoardRes {
    pub leaderboard: Vec<LeaderBoardUser>,
}

pub(crate) async fn get_leaderboard(_auth_payload: AuthPayload) -> impl IntoResponse {
    let mut users = db_models::User::get_users().clone();
    users.sort_by_key(|user| user.total_winnings);
    users.reverse();

    let mut leaderboard = Vec::with_capacity(10);
    for user in users.iter() {
        leaderboard.push(LeaderBoardUser {
            user_id: user.id,
            username: user.username.clone(),
            total_winnings: user.total_winnings,
        });

        if leaderboard.len() >= 10 {
            break;
        }
    }

    Json(GetLeaderBoardRes { leaderboard })
}
