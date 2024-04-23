mod key;
mod crypto;
pub(crate) mod transport;
pub(crate) mod connection;

use std::io;
use std::net::SocketAddr;
use fnrpc::ResponseParams;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tungstenite::Message;
use futures_util::SinkExt;
use fnrpc::{PayloadType, RequestParams};

use crate::rpc;
use crate::push;
use crate::session;
use crate::steam::SteamSession;

pub const CLIENT_HELLO_TIMEOUT: u64 = 5;
pub const CLIENT_PUBLIC_KEY_TIMEOUT: u64 = 5;
pub const CLIENT_SESSION_TIMEOUT: u64 = 5;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Credential error")]
    Credentials,

    #[error("Transport error {0:?}")]
    Transport(#[from] transport::TransportError),

    #[error("Protocol error {0:?}")]
    Protocol(#[from] ProtocolError),

    #[error("Io error {0:?}")]
    Io(#[from] io::Error),

    #[error("Crypto error {0:?}")]
    Crypto(#[from] crypto::CryptoError),

    #[error("Write error {0:?}")]
    Wire(#[from] fnrpc::FNWireError),

    #[error("Could not get a handler for message")]
    NoHandler,

    #[error("Client closed connection")]
    ClosedConnection,
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Got non-binary message")]
    NonBinaryMessage,

    #[error("Timed out while waiting for HELLO")]
    TimeoutHello,

    #[error("Got malformed HELLO")]
    MalformedHello,

    #[error("Timed out while waiting for public key")]
    TimeoutPublicKey,

    #[error("Got malformed public key")]
    MalformedPublicKey,

    #[error("Timed out while waiting for session creation")]
    TimeoutSessionCreation,

    #[error("Expected session message")]
    ExpectedSessionMessage,

    #[error("Received wrong payload type")]
    WrongPayloadType,
}

pub fn new_client(
    steam_session: SteamSession,
    peer_address: SocketAddr,
    transport: transport::ClientTransport,
) -> Client<ClientStateConnected> {
    Client {
        steam_session,
        peer_address,
        state: ClientStateConnected { transport },
    }
}

pub trait ClientState {}

#[derive(Debug)]
pub struct Client<S: ClientState> {
    steam_session: SteamSession,
    peer_address: SocketAddr,
    state: S,
}

#[derive(Debug)]
pub struct ClientStateConnected {
    transport: transport::ClientTransport,
}

impl ClientState for ClientStateConnected {}

impl Client<ClientStateConnected> {
    pub async fn await_hello(mut self) -> Result<Client<ClientStateReceivedHello>, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(
                tokio::time::Duration::from_secs(CLIENT_HELLO_TIMEOUT)
            ) => {
                return Err(ClientError::Protocol(ProtocolError::TimeoutHello))
            },

            // Incoming message that we need to handle
            message = self.state.transport.receive_next() => {
                let hello = match message.map_err(ClientError::Transport)? {
                    Message::Binary(b) => b,
                    Message::Close(_) => return Err(ClientError::ClosedConnection),
                    _ => return Err(ClientError::Protocol(
                        ProtocolError::NonBinaryMessage
                    ))
                };

                if hello != vec![0x00, 0x00, 0x01, 0x00] {
                    return Err(ClientError::Protocol(
                        ProtocolError::MalformedHello
                    ));
                }
            }
        }

        Ok(Client {
            steam_session: self.steam_session,
            peer_address: self.peer_address,
            state: ClientStateReceivedHello {
                transport: self.state.transport
            },
        })
    }
}

#[derive(Debug)]
pub struct ClientStateReceivedHello {
    transport: transport::ClientTransport,
}

impl ClientState for ClientStateReceivedHello {}

impl Client<ClientStateReceivedHello> {
    pub async fn advertise_crypto_session(
        mut self
    ) -> Result<Client<ClientStateAwaitingPublicKey>, ClientError> {
        let crypto = crypto::new_client_crypto().generate_new();

        let payload = crypto.create_crypto_advertisement_buffer().await?;

        let encrypted = crypto.kx_encrypt(payload).await?;
        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(transport::TransportError::WriteFailed(e)))?;

        Ok(Client {
            steam_session: self.steam_session,
            peer_address: self.peer_address,
            state: ClientStateAwaitingPublicKey {
                transport: self.state.transport,
                crypto,
            }
        })
    }
}

#[derive(Debug)]
pub struct ClientStateAwaitingPublicKey {
    transport: transport::ClientTransport,
    crypto: crypto::ClientCrypto<crypto::ClientCryptoStateParametersGenerated>,
}

impl ClientState for ClientStateAwaitingPublicKey {}

impl Client<ClientStateAwaitingPublicKey> {
    pub async fn await_public_key(
        mut self
    ) -> Result<Client<ClientStateAwaitingSession>, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(
                tokio::time::Duration::from_secs(CLIENT_PUBLIC_KEY_TIMEOUT)
            ) => {
                Err(ClientError::Protocol(ProtocolError::TimeoutPublicKey))
            },

            message = self.state.transport.receive_next() => {
                let client_pk = match message.map_err(ClientError::Transport)? {
                    Message::Binary(b) => b,
                    _ => return Err(ClientError::Protocol(
                        ProtocolError::NonBinaryMessage
                    ))
                };

                let client_pk = self.state.crypto.kx_decrypt(client_pk.as_slice()).await?;
                if client_pk.len() != 32 {
                    return Err(ClientError::Protocol(
                        ProtocolError::MalformedPublicKey
                    ));
                }

                let client_pk = sodiumoxide::crypto::kx::x25519blake2b::PublicKey::from_slice(&client_pk)
                    .ok_or(ClientError::Crypto(crypto::CryptoError::PublicKeyCreation))?;

                let crypto = self.state.crypto.derive_session_keys(client_pk)?;

                Ok(Client {
                    steam_session: self.steam_session,
                    peer_address: self.peer_address,
                    state: ClientStateAwaitingSession {
                        transport: self.state.transport,
                        crypto,
                    }
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct ClientStateAwaitingSession {
    transport: transport::ClientTransport,
    crypto: crypto::ClientCrypto<crypto::ClientCryptoStateActiveSession>,
}

impl ClientState for ClientStateAwaitingSession {}

impl Client<ClientStateAwaitingSession> {
    pub async fn await_session_message(
        mut self
    ) -> Result<Client<ClientStateReceivedSessionDetails>, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(
                tokio::time::Duration::from_secs(CLIENT_SESSION_TIMEOUT)
            ) => {
                Err(ClientError::Protocol(ProtocolError::TimeoutSessionCreation))
            },

            message = self.state.transport.receive_next() => {
                let message = match message.map_err(ClientError::Transport)? {
                    Message::Binary(b) => b,
                    _ => return Err(ClientError::Protocol(
                        ProtocolError::NonBinaryMessage
                    ))
                };

                let message = self.state.crypto.session_decrypt(message.as_slice())
                    .await?;

                let (responder, request) = rpc::create_handling_context(message.as_slice())?;
                match &request {
                    RequestParams::CreateSession(_) => {},
                    RequestParams::RestoreSession(_) => {},
                    _ => return Err(ClientError::Protocol(
                        ProtocolError::ExpectedSessionMessage
                    )),
                };

                Ok(Client {
                    steam_session: self.steam_session,
                    peer_address: self.peer_address,
                    state: ClientStateReceivedSessionDetails {
                        transport: self.state.transport,
                        crypto: self.state.crypto,
                        responder,
                        request,
                    }
                })
            }
        }
    }
}

// TODO: this state can probably be merged with ClientCryptoStateAwaitingSession
#[derive(Debug)]
pub struct ClientStateReceivedSessionDetails {
    transport: transport::ClientTransport,
    crypto: crypto::ClientCrypto<crypto::ClientCryptoStateActiveSession>,
    responder: rpc::ResponseContext,
    request: RequestParams,
}

impl ClientState for ClientStateReceivedSessionDetails {}

impl Client<ClientStateReceivedSessionDetails> {
    pub async fn confirm_session(
        mut self
    ) -> Result<Client<ClientStateAuthenticated>, Box<dyn std::error::Error>> {

        let (session, response) = match self.state.request {
            RequestParams::CreateSession(p) =>
                rpc::session::handle_create_session(
                    self.steam_session.to_string(),
                    *p
                ).await?,

            RequestParams::RestoreSession(p) =>
                rpc::session::handle_restore_session(
                    self.steam_session.to_string(),
                    *p,
                ).await?,

            _=> unimplemented!(),
        };

        let mut bytes = self.state.responder.create_response_session(response)?;

        let encrypted = self.state.crypto.session_encrypt(&mut bytes)
            .await?;
        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(transport::TransportError::WriteFailed(e)))?;

        let (push_tx, push_rx) = tokio::sync::mpsc::channel(25);
        push::add_client_channel(session.player_id, push_tx).await;

        Ok(Client {
            steam_session: self.steam_session,
            peer_address: self.peer_address,
            state: ClientStateAuthenticated {
                transport: self.state.transport,
                crypto: self.state.crypto,
                session,
                push_rx,
            }
        })
    }
}

#[derive(Debug)]
pub struct ClientStateAuthenticated {
    transport: transport::ClientTransport,
    crypto: crypto::ClientCrypto<crypto::ClientCryptoStateActiveSession>,
    session: session::ClientSession,
    push_rx: push::ClientPushChannelRX,
}

impl ClientState for ClientStateAuthenticated {}

impl Client<ClientStateAuthenticated> {
    pub async fn serve(&mut self) -> Result<(), ClientError> {
        tokio::select! {
            message = self.state.transport.receive_next() => {
                let message = match message.map_err(ClientError::Transport)? {
                    tungstenite::Message::Binary(b) => b,
                    _ => return Err(ClientError::Protocol(
                        crate::client::ProtocolError::NonBinaryMessage
                    ))
                };

                self.handle_message(&message).await?;
            },

            Some(push) = self.state.push_rx.recv() => {
                self.send_push(push).await?
            },
        };

        Ok(())
    }

    pub async fn handle_message(
        &mut self,
        message: &[u8],
    ) -> Result<(), ClientError> {
        let decrypted = self.state.crypto.session_decrypt(message).await?;

        let payload_type = rpc::get_payload_type(decrypted.as_slice()).unwrap();

        match payload_type {
            PayloadType::Heartbeat
                => self.handle_heartbeat().await?,

            PayloadType::Request
                => self.handle_request(decrypted.as_slice()).await?,

            PayloadType::Push
                => log::debug!("Got push confirmation from client."),

            _ => log::warn!("Got unexpected payload type from client."),
        };

        Ok(())
    }

    async fn handle_heartbeat(&mut self) -> Result<(), ClientError> {
        let encrypted = self.state.crypto.session_encrypt(&mut vec![0x7, 0x64])
            .await?;

        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(transport::TransportError::WriteFailed(e)))?;

        Ok(())
    }

    async fn handle_request(
        &mut self,
        message: &[u8],
    ) -> Result<(), ClientError> {
        let (responder, request) = rpc::create_handling_context(message)?;
        log::info!(
            "Parsed message as type {} for session {}",
            request.name(),
            self.state.session.session_id,
        );

        #[cfg(feature = "dump")]
        {
            std::fs::write(
                format!(
                    "./dump/request-{}-{}-{}.bin",
                    self.state.session.session_id,
                    responder.sequence,
                    request.name(),
                ),
                message,
            )?;
        }

        let response: Result<ResponseParams, Box<dyn std::error::Error>> = match rpc::handler::spawn_handling_task(
            self.state.session.clone(),
            request,
        ) {
            Some(h) => h.await,
            None => {
                log::warn!("Request does not have a handler");
                Err(Box::new(ClientError::NoHandler))
            },
        };

        let mut bytes = responder.create_response(response)?;

        let encrypted = self.state.crypto.session_encrypt(&mut bytes).await?;
        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(transport::TransportError::WriteFailed(e)))?;

        Ok(())
    }

    pub async fn send_push(&mut self, mut bytes: Vec<u8>) -> Result<(), ClientError> {
        let encrypted = self.state.crypto.session_encrypt(&mut bytes).await?;
        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(transport::TransportError::WriteFailed(e)))?;

        Ok(())
    }
}
