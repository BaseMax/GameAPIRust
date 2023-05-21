// Our Table models represented as structs.

use super::core;
use crate::auth::{AuthError, AuthPayload, JWT_ENCODING_KEY};
use jsonwebtoken::{self, Algorithm, Header as JwtHeader};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(debug_assertions)]
use super::debug_data::DebugData;

// Dead code is allowed here. These functions may or may not be used in release mode.
#[allow(dead_code)]
pub fn get_now_iso() -> u128 {
    (SystemTime::now())
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
#[allow(dead_code)]
pub fn get_future_iso(future: Duration) -> u128 {
    (SystemTime::now() + future)
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
#[allow(dead_code)]
pub fn get_past_iso(future: Duration) -> u128 {
    (SystemTime::now() - future)
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct User {
    pub id: u32,
    pub password: String,
    pub token: String,
    pub balance: u64,
    pub is_superuser: bool,
    pub total_winnings: u32,
    pub username: String,
}
impl User {
    pub fn get_users() -> &'static Vec<User> {
        core::get_table("users").unwrap()
    }
    pub fn get_users_mut() -> &'static mut Vec<User> {
        core::get_table_mut("users").unwrap()
    }

    fn new_expire_date() -> u128 {
        get_future_iso(Duration::from_secs(86400 * 7))
    }
    pub fn new_token(id: u32) -> Result<String, AuthError> {
        Ok(unsafe {
            jsonwebtoken::encode(
                &JwtHeader::new(Algorithm::HS512),
                &AuthPayload {
                    id,
                    exp: Self::new_expire_date(),
                },
                JWT_ENCODING_KEY.as_ref().unwrap(),
            )
            .map_err(|_| AuthError::TokenCreation)?
        })
    }

    pub fn refresh_token(&mut self) -> Result<(), AuthError> {
        self.token = Self::new_token(self.id)?;

        Ok(())
    }

    pub fn new(username: String, password: String, id: u32) -> Result<Self, AuthError> {
        Ok(Self {
            id,
            is_superuser: false,
            balance: 0,
            total_winnings: 0,
            password,
            token: Self::new_token(id)?,
            username,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Game {
    pub end_time: u128,
    pub id: u32,
    pub name: String,
    pub results: Option<String>,
    pub start_time: u128,
}
impl Game {
    pub fn get_games() -> &'static Vec<Game> {
        core::get_table("games").unwrap()
    }
    #[allow(dead_code)]
    pub fn get_games_mut() -> &'static mut Vec<Game> {
        core::get_table_mut("games").unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Bet {
    pub amount: u32,
    pub game_id: u32,
    pub expected_results: String,
    pub id: u32,
    pub user_id: u32,
}
impl Bet {
    pub fn get_bets() -> &'static Vec<Bet> {
        core::get_table("bets").unwrap()
    }
    pub fn get_bets_mut() -> &'static mut Vec<Bet> {
        core::get_table_mut("bets").unwrap()
    }
}

// Allocate tables
pub(crate) fn init() {
    #[cfg(debug_assertions)]
    let DebugData { users, games, bets } = DebugData::initialize_data();
    #[cfg(not(debug_assertions))]
    let (users, games, bets) = (
        Vec::new() as Vec<User>,
        Vec::new() as Vec<Game>,
        Vec::new() as Vec<Bet>,
    );

    core::create_table("bets", bets);
    core::create_table("games", games);
    core::create_table("users", users);
}

// Gracefully destroy all tables
pub(crate) fn destroy() {
    core::destroy_table::<Vec<Bet>>("bets");
    core::destroy_table::<Vec<Game>>("games");
    core::destroy_table::<Vec<User>>("users");
}
