use std::{sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}, time};
use rand::prelude::*;
use sqlx::Row;
use thiserror::Error;

use waygate_message::{ObjectIdentifier, RequestCreateSessionParams, RequestRestoreSessionParams, RequestUpdatePlayerStatusParams, ResponseCreateSessionParams, ResponseParams, ResponseRestoreSessionParams, SessionData};
use waygate_database::{database_connection, DatabaseError};
use waygate_pool::{BREAKIN_POOL, SignPoolEntry, BreakInPoolEntry, QuickmatchPoolEntry, PoolError, PoolKeyGuard};

// Sessions are valid for an hour
pub const SESSION_VALIDITY: u64 = 60 * 60 * 7;

// TODO: these errors meaningless without some more context
#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Missing CharacterData")]
    MissingCharacterData,

    #[error("Pool error {0:?}")]
    Pool(#[from] PoolError),

    #[error("Missing breakin entry")]
    MissingBreakinEntry,

    #[error("Database error {0:?}")]
    Database(#[from] DatabaseError),
}

#[derive(Debug)]
pub struct ClientSessionInner {
    pub cookie: String,
    pub external_id: String,
    pub player_id: i32,
    pub session_id: i32,
    pub valid_from: i64,
    pub valid_until: i64,

    pub invadeable: bool,
    pub matching: Option<CharacterMatchingData>,

    pub sign: Vec<PoolKeyGuard<SignPoolEntry>>,
    pub quickmatch: Option<PoolKeyGuard<QuickmatchPoolEntry>>,
    pub breakin: Option<PoolKeyGuard<BreakInPoolEntry>>,
}

impl ClientSessionInner {
    pub fn update_invadeability(&mut self) -> Result<(), SessionError> {
        if self.invadeable && self.breakin.is_none() {
            let matching = self.matching.as_ref()
                .ok_or(SessionError::MissingCharacterData)?;

            let key = BREAKIN_POOL
                .insert(self.player_id, BreakInPoolEntry {
                    character_level: matching.level,
                    weapon_level: matching.max_reinforce_level,
                    steam_id: self.external_id.clone(),
                })?;

            self.breakin = Some(key);
            tracing::debug!("Added player to breakin pool. player_id = {}", self.player_id);
        } else if !self.invadeable && self.breakin.is_some() {
            let _ = self.breakin.take()
                .ok_or(SessionError::MissingBreakinEntry)?;

            tracing::debug!("Removed player from breakin pool. player_id = {}", self.player_id);
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
pub async fn new_client_session(external_id: String) -> Result<ClientSessionInner, SessionError> {
    let player_id = acquire_player_id(&external_id).await?;
    let cookie = generate_session_cookie();

    let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    let valid_for = time::Duration::from_secs(SESSION_VALIDITY);
    let valid_from = (now - valid_for).as_secs() as i64;
    let valid_until = (now + valid_for).as_secs() as i64;

    let mut connection = database_connection().await?;
    let session_id = sqlx::query("INSERT INTO sessions (player_id, cookie, valid_until) VALUES ($1, $2, $3) RETURNING session_id")
        .bind(player_id)
        .bind(&cookie)
        .bind(valid_until)
        .fetch_one(&mut *connection)
        .await
        .map_err(DatabaseError::from)?
        .get("session_id");

    Ok(ClientSessionInner {
        cookie,
        external_id,
        player_id,
        session_id,
        valid_from,
        valid_until,

        sign: vec![],
        quickmatch: None,

        invadeable: false,
        matching: None,
        breakin: None,
    })
}

/// Creates a client session from an already existing session 
pub async fn get_client_session(external_id: String, session_id: i32, cookie: &str) -> Result<ClientSessionInner, SessionError> {
    let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();

    let mut connection = database_connection().await?;
    // let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE session_id = $1 AND cookie = $2 AND valid_until > $3")
    let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE session_id = $1 AND cookie = $2")
        .bind(session_id)
        .bind(cookie)
        // .bind(now.as_secs() as i64)
        .fetch_one(&mut *connection)
        .await
        .map_err(DatabaseError::from)?;

    let valid_for = time::Duration::from_secs(SESSION_VALIDITY);
    let valid_from = (now - valid_for).as_secs() as i64;

    Ok(ClientSessionInner {
        cookie: session.cookie,
        external_id,
        player_id: session.player_id,
        session_id: session.session_id,
        valid_from,
        valid_until: session.valid_until,

        sign: vec![],
        quickmatch: None,

        invadeable: false,
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
    let mut connection = database_connection().await?;
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

pub async fn handle_create_session(
    external_id: String,
    _params: RequestCreateSessionParams,
) -> Result<(ClientSession, ResponseParams), Box<dyn std::error::Error>> {
    tracing::info!(
        "Player sent CreateSession. external_id = {}.",
        external_id,
    );

    let session = new_client_session(external_id.clone()).await?;

    let player_id = session.player_id;
    let session_id = session.session_id;
    let valid_from = session.valid_from;
    let valid_until = session.valid_until;
    let cookie = session.cookie.clone();

    Ok((
        Arc::new(RwLock::new(session)),
        ResponseParams::CreateSession(ResponseCreateSessionParams {
            player_id,
            steam_id: external_id,
            ip_address: String::from(""),
            session_data: SessionData {
                identifier: ObjectIdentifier {
                    object_id: session_id,
                    secondary_id: 0x0,
                },
                valid_from,
                valid_until,
                cookie,
            },
            redirect_url: String::from(""),
        })
    ))
}

pub async fn handle_restore_session(
    external_id: String,
    params: RequestRestoreSessionParams,
) -> Result<(ClientSession, ResponseParams), Box<dyn std::error::Error>> {
    tracing::info!(
        "Player sent RestoreSession. external_id = {}.",
        external_id,
    );

    let session = get_client_session(
        external_id,
        params.session_data.identifier.object_id,
        &params.session_data.cookie,
    ).await?;

    let session_id = session.session_id;
    let valid_from = session.valid_from;
    let valid_until = session.valid_until;
    let cookie = session.cookie.clone();

    Ok((
        Arc::new(RwLock::new(session)),
        ResponseParams::RestoreSession(ResponseRestoreSessionParams {
            session_data: SessionData {
                identifier: ObjectIdentifier {
                    object_id: session_id,
                    secondary_id: 0x0,
                },
                valid_from,
                valid_until,
                cookie,
            },
            unk_string: String::from(""),
        })
    ))
}
