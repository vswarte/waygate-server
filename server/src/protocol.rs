mod crypto;

use std::time::{self};

use base64::{prelude::BASE64_STANDARD, Engine};
use crypto::ClientProtocolCrypto;

use message::{
    builder::MessageBuilder,
    eldenring::{
        RequestParams, ResponseCreateSessionParams, ResponseParams, ResponseRestoreSessionParams,
        SessionData,
    },
    reader::MessageReader,
    MessageType,
};
use rand::prelude::*;
use sqlx::{Pool, Postgres, Row};

use crate::Config;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Received invalid hello message.")]
    InvalidHello,
    #[error("Crypto protocol error. {0}")]
    Crypto(#[from] crypto::Error),
    #[error("Protocol violation. {0}")]
    ProtocolViolation(&'static str),
    #[error("Incorrect message type.")]
    MessageType,
    #[error("Could not acquire player record.")]
    AcquirePlayer,
    #[error("Could not acquire session record.")]
    AcquireSession,
    #[error("Failed building message.")]
    MessageBuilder,
    #[error("Failed encrypting message.")]
    Encrypt,
}

pub struct ClientProtocolConfig {
    kx_server_secret_key: Vec<u8>,
    kx_client_public_key: Vec<u8>,
}

impl TryFrom<&Config> for ClientProtocolConfig {
    type Error = base64::DecodeError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        Ok(ClientProtocolConfig {
            kx_server_secret_key: BASE64_STANDARD.decode(&value.server_secret_key)?,
            kx_client_public_key: BASE64_STANDARD.decode(&value.client_public_key)?,
        })
    }
}

/// Handles the setup phase of the client connection as well as the key exchange.
pub struct ClientProtocol {
    database: Pool<Postgres>,
    external_id: String,
    peer_address: String,
    crypto: ClientProtocolCrypto,
    state: ClientProtocolState,
    session: Option<ClientSession>,
}

impl ClientProtocol {
    pub fn new(
        database: Pool<Postgres>,
        external_id: String,
        peer_address: String,
        config: ClientProtocolConfig,
    ) -> Result<Self, Error> {
        Ok(Self {
            database,
            external_id,
            peer_address,
            crypto: ClientProtocolCrypto::generate(&config)?,
            state: ClientProtocolState::AwaitingHello,
            session: None,
        })
    }

    /// Takes in client-supplied buffer and produces a response or returns an error in case of
    /// protocol violations.
    pub async fn transition(&mut self, data: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        match self.state {
            ClientProtocolState::AwaitingHello => {
                self.state = ClientProtocolState::AwaitingPublicKey;
                if data != [0x00, 0x00, 0x01, 0x00] {
                    return Err(Error::InvalidHello);
                }

                Ok(Some(self.crypto.create_session_advertisement()?))
            }
            ClientProtocolState::AwaitingPublicKey => {
                self.crypto.post_client_keys(data)?;
                self.state = ClientProtocolState::AwaitingCredentials;

                Ok(None)
            }
            ClientProtocolState::AwaitingCredentials => {
                let decrypted = self.crypto.decrypt(data)?;
                let reader = MessageReader::new(decrypted.as_ref());

                if !matches!(
                    reader.message_type().map_err(|_| Error::MessageType)?,
                    MessageType::Request
                ) {
                    return Err(Error::ProtocolViolation(
                        "First message was not a session request (incorrect message type)",
                    ));
                }

                // TODO: fetch from DB
                let player_id = self
                    .acquire_player_id(&self.external_id)
                    .await
                    .map_err(|_| Error::AcquirePlayer)?;
                let cookie = generate_session_cookie();

                let now = time::SystemTime::now()
                    .duration_since(time::UNIX_EPOCH)
                    .unwrap();
                let valid_for = time::Duration::from_secs(60 * 60 * 12);
                let valid_from = (now - valid_for).as_secs() as i64;
                let valid_until = (now + valid_for).as_secs() as i64;

                let response = match reader.deserialize_request().map_err(|_| {
                    Error::ProtocolViolation("Deserialization of session request failed")
                })? {
                    RequestParams::CreateSession(_) => {
                        let session_id = self
                            .create_session(player_id, &cookie, valid_until)
                            .await
                            .map_err(|_| Error::AcquireSession)?;

                        self.session = Some(ClientSession {
                            player_id,
                            session_id,
                            external_id: self.external_id.clone(),
                            peer_address: self.peer_address.clone(),
                        });

                        ResponseParams::CreateSession(ResponseCreateSessionParams {
                            player_id,
                            external_id: self.external_id.clone(),
                            ip_address: self.peer_address.clone(),
                            session_data: SessionData {
                                identifier: session_id,
                                valid_from,
                                valid_until,
                                cookie,
                            },
                            redirect_url: String::new(),
                        })
                    }
                    RequestParams::RestoreSession(request) => {
                        let session = self
                            .retrieve_session(
                                request.session_data.identifier,
                                &request.session_data.cookie,
                            )
                            .await
                            .map_err(|_| Error::AcquireSession)?;

                        self.session = Some(ClientSession {
                            player_id,
                            session_id: session.session_id,
                            external_id: self.external_id.clone(),
                            peer_address: self.peer_address.clone(),
                        });

                        ResponseParams::RestoreSession(ResponseRestoreSessionParams {
                            session_data: SessionData {
                                identifier: session.session_id,
                                valid_from,
                                valid_until,
                                cookie,
                            },
                            redirect_url: String::new(),
                        })
                    }
                    _ => {
                        return Err(Error::ProtocolViolation(
                            "First request was not CreateSesion or RestoreSession",
                        ))
                    }
                };

                let mut response = MessageBuilder::response()
                    .sequence(
                        reader
                            .sequence()
                            .map_err(|_| {
                                Error::ProtocolViolation("No sequence on session creation message.")
                            })?
                            .unwrap(),
                    )
                    .body(response)
                    .build()
                    .map_err(|_| Error::MessageBuilder)?;

                let encrypted = self
                    .crypto
                    .encrypt(response.as_mut_slice())
                    .map_err(|_| Error::Encrypt)?;

                self.state = ClientProtocolState::Finalized;

                Ok(Some(encrypted))
            }
            _ => unimplemented!(),
        }
    }

    /// Decrypts a received message.
    pub fn decrypt_message(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(self.crypto.decrypt(data)?)
    }

    /// Encrypts an outbound message.
    pub fn encrypt_message(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(self.crypto.encrypt(data)?)
    }

    /// Are we done negotiating low-level connection details?
    pub fn finalized(&self) -> bool {
        matches!(self.state, ClientProtocolState::Finalized)
    }

    pub fn session_details(&self) -> Option<ClientSession> {
        self.session.clone()
    }

    async fn acquire_player_id(&self, external_id: &str) -> Result<i32, sqlx::Error> {
        let result = sqlx::query_as::<_, PlayerRecord>(
            "SELECT player_id FROM players WHERE external_id = $1",
        )
        .bind(external_id)
        .fetch_optional(&self.database)
        .await?;

        match result {
            Some(player) => Ok(player.player_id),
            None => {
                let inserted = sqlx::query(
                    "INSERT INTO players (external_id) VALUES ($1) RETURNING player_id",
                )
                .bind(external_id)
                .fetch_one(&self.database)
                .await?;

                Ok(inserted.get("player_id"))
            }
        }
    }

    async fn create_session(
        &self,
        player_id: i32,
        cookie: &str,
        valid_until: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(
            sqlx::query("INSERT INTO sessions (player_id, cookie, valid_until) VALUES ($1, $2, $3) RETURNING session_id")
                .bind(player_id)
                .bind(cookie)
                .bind(valid_until)
                .fetch_one(&self.database)
                .await?
                .get("session_id")
        )
    }

    async fn retrieve_session(
        &self,
        session_id: i64,
        cookie: &str,
    ) -> Result<SessionRecord, Box<dyn std::error::Error>> {
        Ok(sqlx::query_as::<_, SessionRecord>(
            "SELECT * FROM sessions WHERE session_id = $1 -- AND cookie = $2",
        )
        .bind(session_id)
        // .bind(cookie)
        .fetch_one(&self.database)
        .await?)
    }
}

#[derive(Clone)]
pub struct ClientSession {
    pub player_id: i32,
    pub session_id: i64,
    pub external_id: String,
    pub peer_address: String,
}

enum ClientProtocolState {
    /// Awaiting [0x00, 0x00, 0x01, 0x00] message from client.
    /// In response we send a crypto session advertisement.
    AwaitingHello,
    /// Awaiting public key from the client.
    AwaitingPublicKey,
    /// Awaiting credentials to create a session.
    AwaitingCredentials,
    /// We've setup the necessary crypto stuff as well as a game session.
    Finalized,
}

// Secure enough
fn generate_session_cookie() -> String {
    let cookie = rand::rng().random::<[u8; 32]>();
    encode_session_cookie(&cookie)
}

fn encode_session_cookie(cookie: &[u8]) -> String {
    format!("{:02x?}", cookie)
        .replace(['[', ']', ' ', ','], "")
        .replace("0x", "")
}

#[derive(sqlx::FromRow)]
struct PlayerRecord {
    player_id: i32,
}

#[derive(sqlx::FromRow, Debug)]
struct SessionRecord {
    session_id: i64,
}
