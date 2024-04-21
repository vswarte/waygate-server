use std::time;
use rand::prelude::*;
use sqlx::Row;

use crate::database::{self, DatabaseError};

// Sessions are valid for an hour
pub const SESSION_VALIDITY: u64 = 60 * 60;
pub const SESSION_COOKIE_SIZE: usize = 32;

#[derive(Clone, Debug)]
pub struct ClientSession {
    pub cookie: String,
    pub player_id: i32,
    pub session_id: i32,
    pub valid_from: u64,
    pub valid_until: u64,
}

/// Creates a new session for a given external_id
pub async fn new_client_session(external_id: &str) -> Result<ClientSession, DatabaseError> {
    let player_id = acquire_player_id(external_id).await?;
    let cookie = generate_session_cookie();

    let mut connection = database::acquire().await?;
    let session_id = sqlx::query("INSERT INTO sessions (player_id, cookie) VALUES ($1, $2) RETURNING session_id")
        .bind(player_id)
        .bind(cookie)
        .fetch_one(&mut *connection)
        .await?
        .get("session_id");

    let valid_for = time::Duration::from_secs(SESSION_VALIDITY);
    let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    let valid_from = (now - valid_for).as_secs();
    let valid_until = (now + valid_for).as_secs();

    let cookie = encode_session_cookie(cookie);

    Ok(ClientSession {
        player_id,
        session_id,
        cookie,
        valid_from,
        valid_until,
    })
}

#[derive(sqlx::FromRow)]
struct Player {
    player_id: i32,
}

/// Tries to fetch our player ID by the external_id, creates the record and
/// returns ID of newly created row if none exists with that external ID.
async fn acquire_player_id(external_id: &str) -> Result<i32, DatabaseError> {
    let mut connection = database::acquire().await?;
    let result = sqlx::query_as::<_, Player>("SELECT player_id FROM players WHERE external_id = $1",)
        .bind(external_id)
        .fetch_optional(&mut *connection)
        .await?;

    match result {
        Some(player) => Ok(player.player_id),
        None => {
            let inserted = sqlx::query("INSERT INTO players (external_id) VALUES ($1) RETURNING player_id")
                .bind(external_id)
                .fetch_one(&mut *connection)
                .await?;

            Ok(inserted.get("player_id"))
        }
    }
}

fn generate_session_cookie() -> [u8; SESSION_COOKIE_SIZE] {
    thread_rng().gen::<[u8; 32]>()
}

fn encode_session_cookie(cookie: [u8; SESSION_COOKIE_SIZE]) -> String {
    format!("{:02x?}", cookie)
        .replace("[", "")
        .replace("]", "")
        .replace(" ", "")
        .replace("0x", "")
        .replace(",", "")
}
