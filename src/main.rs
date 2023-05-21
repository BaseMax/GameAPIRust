use axum::{
    routing::{delete, get, post, put},
    Router, Server,
};
use std::env;

mod api;
mod db;

// Re-exporting these so our APIs can access them.
pub(crate) mod auth;
pub(crate) use db::models as db_models;

#[tokio::main]
async fn main() {
    let listener = env::var("ADDR").unwrap_or(String::from("0.0.0.0:3000"));

    let jwt_secret = env::var("JWT_SECRET");
    // Use random key for debug if the env doesn't exist
    #[cfg(debug_assertions)]
    let jwt_secret = jwt_secret.unwrap_or(String::from("123456789"));
    #[cfg(not(debug_assertions))]
    let jwt_secret = jwt_secret.expect("`JWT_SECRET` env must be set");

    let app = Router::new()
        .route("/api/authenticate", post(api::authenticate))
        .route("/api/bets", get(api::get_bets))
        .route("/api/bets", post(api::post_bets))
        .route("/api/bets/:bet_id", delete(api::delete_bet))
        .route("/api/bets/:bet_id", put(api::put_bet))
        .route("/api/games", get(api::get_games))
        .route("/api/games/:game_id/bets", get(api::get_bets_on_game))
        .route(
            "/api/games/:game_id/participants",
            get(api::get_participants),
        )
        .route("/api/games/:game_id/results", get(api::get_game_results))
        .route("/api/games/:game_id/results", post(api::post_game_results))
        .route("/api/leaderboard", get(api::get_leaderboard))
        .route("/api/profile", delete(api::delete_profile))
        .route("/api/profile", get(api::get_profile))
        .route("/api/profile", put(api::put_profile))
        .route("/api/users", post(api::post_user))
        .route("/api/users/:user_id", get(api::get_user));

    auth::init_auth_keys(jwt_secret);
    db_models::init();

    println!("Listening on `{}`.", listener);
    Server::bind(&listener.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    db_models::destroy();
}
