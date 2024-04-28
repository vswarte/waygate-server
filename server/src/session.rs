use std::{error::Error, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}, time};
use fnrpc::player::RequestUpdatePlayerStatusParams;
use rand::prelude::*;
use sqlx::Row;

use crate::{database::{self, DatabaseError}, pool::{self, breakin::BreakInPoolEntry, key::PoolKey}};

// Sessions are valid for an hour
pub const SESSION_VALIDITY: u64 = 60 * 60;

#[derive(Clone, Debug)]
pub struct ClientSessionInner {
    pub cookie: String,
    pub external_id: String,
    pub player_id: i32,
    pub session_id: i32,
    pub valid_from: i64,
    pub valid_until: i64,

    pub invadeable: bool,
    pub sign: Option<PoolKey>,

    pub matching: Option<CharacterMatchingData>,
    pub breakin: Option<PoolKey>,
}

impl ClientSessionInner {
    pub fn update_invadeability(&mut self) -> Result<(), Box<dyn Error>> {
        if self.invadeable && self.breakin.is_none() {
            let matching = self.matching.as_ref().unwrap();

            // Add token to fucking pool help me
            let key = pool::breakin()?
                .insert(self.player_id, BreakInPoolEntry {
                    character_level: matching.level,
                    weapon_level: matching.max_reinforce_level,
                    steam_id: self.external_id.clone(),
                })?;

            self.breakin = Some(key);
            log::info!("Added to breakin pool");
        } else if !self.invadeable && self.breakin.is_some() {
            let key = self.breakin.take().unwrap();

            pool::breakin()?.remove(&key)?;
            log::info!("Removed from breakin pool");
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CharacterMatchingData {
    pub level: u32,
    pub max_reinforce_level: u32,
}

impl From<&RequestUpdatePlayerStatusParams> for CharacterMatchingData {
    fn from(value: &RequestUpdatePlayerStatusParams) -> Self {
        Self {
            level: value.character.level,
            max_reinforce_level: value.character.max_reinforce_level,
        }
    }
}

pub type ClientSession = Arc<RwLock<ClientSessionInner>>;

pub trait ClientSessionContainer {
    fn lock_read(&self) -> RwLockReadGuard<'_, ClientSessionInner>;
    fn lock_write(&self) -> RwLockWriteGuard<'_, ClientSessionInner>;
}

impl ClientSessionContainer for ClientSession {
    fn lock_read(&self) -> RwLockReadGuard<'_, ClientSessionInner> {
        match self.read() {
            Ok(s) => s,
            Err(e) => {
                self.clear_poison();
                e.into_inner()
            },
        }
    }

    fn lock_write(&self) -> RwLockWriteGuard<'_, ClientSessionInner> {
        match self.write() {
            Ok(s) => s,
            Err(e) => {
                self.clear_poison();
                e.into_inner()
            },
        }
    }
}

/// Creates a new session for a given external_id
pub async fn new_client_session(external_id: String) -> Result<ClientSessionInner, DatabaseError> {
    let player_id = acquire_player_id(&external_id).await?;
    let cookie = generate_session_cookie();

    let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    let valid_for = time::Duration::from_secs(SESSION_VALIDITY);
    let valid_from = (now - valid_for).as_secs() as i64;
    let valid_until = (now + valid_for).as_secs() as i64;

    let mut connection = database::acquire().await?;
    let session_id = sqlx::query("INSERT INTO sessions (player_id, cookie, valid_until) VALUES ($1, $2, $3) RETURNING session_id")
        .bind(player_id)
        .bind(&cookie)
        .bind(valid_until)
        .fetch_one(&mut *connection)
        .await?
        .get("session_id");

    Ok(ClientSessionInner {
        cookie,
        external_id,
        player_id,
        session_id,
        valid_from,
        valid_until,

        invadeable: false,
        sign: None,
        matching: None,
        breakin: None,
    })
}

/// Creates a client session from an already existing session 
pub async fn get_client_session(external_id: String, session_id: i32, cookie: &str) -> Result<ClientSessionInner, DatabaseError> {
    let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();

    let mut connection = database::acquire().await?;
    // let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE session_id = $1 AND cookie = $2 AND valid_until > $3")
    let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE session_id = $1 AND cookie = $2")
        .bind(session_id)
        .bind(cookie)
        // .bind(now.as_secs() as i64)
        .fetch_one(&mut *connection)
        .await?;

    let valid_for = time::Duration::from_secs(SESSION_VALIDITY);
    let valid_from = (now - valid_for).as_secs() as i64;

    Ok(ClientSessionInner {
        cookie: session.cookie,
        external_id,
        player_id: session.player_id,
        session_id: session.session_id,
        valid_from,
        valid_until: session.valid_until,

        invadeable: false,
        sign: None,
        matching: None,
        breakin: None,
    })
}

#[derive(sqlx::FromRow)]
struct Player {
    player_id: i32,
}

#[derive(sqlx::FromRow, Debug)]
struct Session {
    session_id: i32,
    player_id: i32,
    valid_until: i64,
    cookie: String,
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

fn generate_session_cookie() -> String {
    encode_session_cookie(&thread_rng().gen::<[u8; 32]>())
}

fn encode_session_cookie(cookie: &[u8]) -> String {
    format!("{:02x?}", cookie)
        .replace(['[', ']', ' ', ','], "")
        .replace("0x", "")
}
