use crate::{auth::AuthPayload, db_models};
use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct GetBetsRes<'a> {
    pub bets: Vec<&'a db_models::Bet>,
}

pub(crate) async fn get_bets(auth_payload: AuthPayload) -> impl IntoResponse {
    let bets = db_models::Bet::get_bets();
    let user_bets = bets
        .into_iter()
        .filter(|bet| bet.user_id == auth_payload.id)
        .collect::<Vec<&db_models::Bet>>();

    Json(GetBetsRes { bets: user_bets })
}
