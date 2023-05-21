// Here we have a bunch of dummy data for the debug mode so we can test and inspect
// out APIs.

use crate::db_models;
use std::time::Duration;

pub const USER_TOEKN1: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpZCI6MSwiZXhwIjoxNjg1MjYwOTIzMzY0fQ.D3RJlVWVH7JnQCuP29lge14baWWmXhtxFGp473k1P7HlbLRlyM1ld_17LZjeknkLM7AlCAaHIyBv_bb8zJE4UA";
pub const USER_TOEKN2: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpZCI6MiwiZXhwIjoxNjg1MjYwOTY3MTk3fQ.-rcbqEIquUCbvOUQ8rh8HsdlF7DKanD53bB6Io_-MVGAJsZqEs9khKltZgEgJOpxSFQko7z2dto1bBTlZZP-Mw";

pub(crate) struct DebugData {
    pub bets: Vec<db_models::Bet>,
    pub games: Vec<db_models::Game>,
    pub users: Vec<db_models::User>,
}
impl DebugData {
    pub fn initialize_data() -> Self {
        let users = vec![
            db_models::User {
                id: 1,
                balance: 100,
                total_winnings: 1,
                is_superuser: true,
                username: String::from("foo"),
                password: String::from("bar"),
                token: String::from(USER_TOEKN1),
            },
            db_models::User {
                id: 2,
                balance: 100 * 5,
                total_winnings: 5,
                is_superuser: false,
                username: String::from("foo_2"),
                password: String::from("bar_2"),
                token: String::from(USER_TOEKN2),
            },
        ];

        let games = vec![
            db_models::Game {
                id: 1,
                results: None,
                name: String::from("Volleyball World Cup"),
                start_time: db_models::get_past_iso(Duration::from_secs(86400 * 7)),
                end_time: db_models::get_past_iso(Duration::from_secs(86400 * 2)),
            },
            db_models::Game {
                id: 2,
                results: None,
                name: String::from("Football World Cup"),
                start_time: db_models::get_now_iso(),
                end_time: db_models::get_future_iso(Duration::from_secs(86400 * 2)),
            },
            db_models::Game {
                id: 2,
                results: Some(String::from("Germany")),
                name: String::from("Basketball League"),
                start_time: db_models::get_past_iso(Duration::from_secs(86400 * 7)),
                end_time: db_models::get_past_iso(Duration::from_secs(86400 * 2)),
            },
        ];

        let bets = vec![db_models::Bet {
            user_id: 1,
            game_id: 1,
            expected_results: String::from("Germany"),
            id: 456,
            amount: 100,
        }];

        Self { bets, games, users }
    }
}
