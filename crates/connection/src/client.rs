use std::future::Future;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use base64::DecodeError;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use thiserror::Error;
use tokio::sync::mpsc::channel;
use tungstenite::Message;
use futures_util::SinkExt;
use waygate_message::{PayloadType, RequestParams, ResponseParams};
use waygate_wire::{deserialize, serialize, FNWireError};

use crate::{add_client_channel, handle_create_session, handle_restore_session, new_client_crypto, ClientPushChannelRX, ClientSession, ClientSessionContainer, CryptoError};
use crate::ClientCrypto;
use crate::ClientCryptoStateActiveSession;
use crate::ClientCryptoStateParametersGenerated;
use crate::ClientTransport;
use crate::TransportError;
use waygate_steam::SteamSession;

pub const CLIENT_HELLO_TIMEOUT: u64 = 5;
pub const CLIENT_PUBLIC_KEY_TIMEOUT: u64 = 5;
pub const CLIENT_SESSION_TIMEOUT: u64 = 5;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Credential error")]
    Credentials,

    #[error("Transport error {0:?}")]
    Transport(#[from] TransportError),

    #[error("Protocol error {0:?}")]
    Protocol(#[from] ProtocolError),

    #[error("Io error {0:?}")]
    Io(#[from] io::Error),

    #[error("Crypto error {0:?}")]
    Crypto(#[from] CryptoError),

    #[error("Write error {0:?}")]
    Wire(#[from] FNWireError),

    #[error("Could not get a handler for message. {0}")]
    NoHandler(String),

    #[error("Client closed connection")]
    ClosedConnection,

    #[error("Bootstrap keys decode failed. {0}")]
    Decode(#[from] DecodeError),

    #[error("Player has been banned.")]
    Banned,
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
    transport: ClientTransport,
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
    transport: ClientTransport,
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
    transport: ClientTransport,
}

impl ClientState for ClientStateReceivedHello {}

impl Client<ClientStateReceivedHello> {
    pub async fn advertise_crypto_session(
        mut self
    ) -> Result<Client<ClientStateAwaitingPublicKey>, ClientError> {
        let crypto = new_client_crypto().generate_new();

        let payload = crypto.create_crypto_advertisement_buffer().await?;

        let encrypted = crypto.kx_encrypt(payload).await?;
        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(TransportError::WriteFailed(e)))?;

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
    transport: ClientTransport,
    crypto: ClientCrypto<ClientCryptoStateParametersGenerated>,
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

                let crypto = self.state.crypto.derive_session_keys(
                    client_pk.as_slice().try_into().unwrap()
                )?;

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
    transport: ClientTransport,
    crypto: ClientCrypto<ClientCryptoStateActiveSession>,
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

                let (responder, request) = create_handling_context(message.as_slice())?;
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
    transport: ClientTransport,
    crypto: ClientCrypto<ClientCryptoStateActiveSession>,
    responder: ResponseContext,
    request: RequestParams,
}

impl ClientState for ClientStateReceivedSessionDetails {}

impl Client<ClientStateReceivedSessionDetails> {
    pub async fn confirm_session<T, Fut>(
        mut self,
        dispatch: T,
    ) -> Result<Client<ClientStateAuthenticated<T, Fut>>, Box<dyn std::error::Error>>
        where
            T: Fn(ClientSession, RequestParams) -> Fut,
            Fut: Future<Output = Result<ResponseParams, Box<dyn std::error::Error>>>
    {

        let (session, response) = match self.state.request {
            RequestParams::CreateSession(p) =>
                handle_create_session(
                    self.steam_session.to_string(),
                    *p
                ).await?,

            RequestParams::RestoreSession(p) =>
                handle_restore_session(
                    self.steam_session.to_string(),
                    *p,
                ).await?,

            _=> unimplemented!(),
        };

        let mut bytes = self.state.responder.create_response_session(response)?;

        let encrypted = self.state.crypto.session_encrypt(&mut bytes)
            .await?;
        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(TransportError::WriteFailed(e)))?;

        let (push_tx, push_rx) = channel(25);
        let player_id = session.lock_read().player_id;
        add_client_channel(player_id, push_tx).await;

        Ok(Client {
            steam_session: self.steam_session,
            peer_address: self.peer_address,
            state: ClientStateAuthenticated {
                transport: self.state.transport,
                crypto: self.state.crypto,
                session,
                push_rx,
                dispatch,
            }
        })
    }
}

#[derive(Debug)]
pub struct ClientStateAuthenticated<T, Fut>
where
    T: Fn(ClientSession, RequestParams) -> Fut,
    Fut: Future<Output = Result<ResponseParams, Box<dyn std::error::Error>>>
{
    transport: ClientTransport,
    crypto: ClientCrypto<ClientCryptoStateActiveSession>,
    session: ClientSession,
    push_rx: ClientPushChannelRX,
    dispatch: T,
}

impl<T, Fut> ClientState for ClientStateAuthenticated<T, Fut>
where
    T: Fn(ClientSession, RequestParams) -> Fut,
    Fut: Future<Output = Result<ResponseParams, Box<dyn std::error::Error>>>
{}

impl<T, Fut> Client<ClientStateAuthenticated<T, Fut>>
where
    T: Fn(ClientSession, RequestParams) -> Fut,
    Fut: Future<Output = Result<ResponseParams, Box<dyn std::error::Error>>>
{
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

        let payload_type = decrypted.as_slice().read_u8()?;

        match payload_type.into() {
            PayloadType::Heartbeat
                => self.handle_heartbeat().await?,

            PayloadType::Request
                => self.handle_request(decrypted.as_slice()).await?,

            PayloadType::Push
                => tracing::debug!("Got push confirmation from client."),

            _ => tracing::warn!("Got unexpected payload type from client."),
        };

        Ok(())
    }

    async fn handle_heartbeat(&mut self) -> Result<(), ClientError> {
        let encrypted = self.state.crypto.session_encrypt(&mut vec![0x7, 0x64])
            .await?;

        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(TransportError::WriteFailed(e)))?;

        Ok(())
    }

    async fn handle_request(
        &mut self,
        message: &[u8],
    ) -> Result<(), ClientError> {
        let (responder, request) = create_handling_context(message)?;

        let response = (self.state.dispatch)(
            self.state.session.clone(),
            request,
        ).await;

        let mut bytes = responder.create_response(response)?;
        let encrypted = self.state.crypto.session_encrypt(&mut bytes).await?;

        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(TransportError::WriteFailed(e)))?;

        Ok(())
    }

    pub async fn send_push(&mut self, mut bytes: Vec<u8>) -> Result<(), ClientError> {
        let encrypted = self.state.crypto.session_encrypt(&mut bytes).await?;
        self.state.transport.sink.send(Message::Binary(encrypted))
            .await.map_err(|e| ClientError::Transport(TransportError::WriteFailed(e)))?;

        Ok(())
    }
}

pub fn create_handling_context<R: Read>(mut r: R) -> Result<(ResponseContext, RequestParams), ClientError> {
    let payload_type: PayloadType = r.read_u8()?.into();

    // TODO: newtype to remove runtime checking
    if payload_type != PayloadType::Request {
        return Err(ClientError::Protocol(
            ProtocolError::WrongPayloadType,
        ));
    }

    let context = ResponseContext { sequence: r.read_u32::<LE>()? };

    let mut params_buffer = vec![];
    r.read_to_end(&mut params_buffer)?;

    let params = deserialize::<RequestParams>(&params_buffer)
        .map_err(ClientError::Wire)?;

    Ok((context, params))
}

#[derive(Debug)]
pub struct ResponseContext {
    pub sequence: u32,
}

impl ResponseContext {
    pub fn create_response_session(&self, params: ResponseParams) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![];

        buffer.write_u8(5)?; // Response payload type
        buffer.write_u32::<LE>(self.sequence)?;
        buffer.write_u8(1)?; // Status code

        let param_buffer = serialize(params)
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
        buffer.write_all(param_buffer.as_slice())?;

        Ok(buffer)
    }

    /// Formats a response to be send to the client.
    pub fn create_response(
        &self,
        response: Result<ResponseParams, Box<dyn std::error::Error>>
    ) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![];

        buffer.write_u8(5)?; // Response payload type
        buffer.write_u32::<LE>(self.sequence)?;

        match response {
            Err(e) => {
                tracing::error!("Could not handle request: {e:?}");

                buffer.write_u8(0)?; // Status code
                buffer.write_u32::<LE>(0)?; // Error code
            },
            Ok(params) => {
                buffer.write_u8(1)?; // Status code
                let param_buffer = serialize(params)
                    .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
                buffer.write_all(param_buffer.as_slice())?;
            },
        };

        Ok(buffer)
    }
}
