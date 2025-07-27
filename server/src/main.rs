use std::{
    error::Error,
    net::SocketAddr,
    sync::{mpsc::channel, Arc, OnceLock},
    time::UNIX_EPOCH,
};

use actix_web::{web, App, HttpServer};
use api::{
    auth::CheckKey,
    ban::{delete_ban, get_ban, get_ban_by_id, post_ban},
    health::healthcheck,
    notification::announcement,
    AppState,
};
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use handler::{
    eldenring::{ActiveHandler, BannedClientHandler, DefaultClientHandler},
    RequestHandler,
};
use message::{builder::MessageBuilder, reader::MessageReader, MessageType};
use protocol::ClientProtocol;
use services::eldenring::GameServices;
use sqlx::{Pool, Postgres};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::tungstenite::Message;

use crate::logging::LogContext;

mod api;
mod bans;
mod handler;
mod logging;
mod notification;
mod protocol;
mod services;
mod steam;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    /// Bind address for the game-facing server.
    #[arg(long, env("WAYGATE_BIND"))]
    bind: String,

    /// Bind address for the HTTP JSON API.
    #[arg(long, env("WAYGATE_API_BIND"))]
    api_bind: String,

    /// Auth for the HTTP JSON API. Passed alongside requests as the
    /// X-Auth-Token HTTP header.
    #[arg(long, env("WAYGATE_API_KEY"))]
    api_key: String,

    /// Database URL pointing to the postgresql instance.
    #[arg(long, env("WAYGATE_DATABASE"))]
    database: String,

    /// Key used by server to encrypt KX messages going to client.
    /// This key should be kept secret.
    #[arg(long, env("WAYGATE_CLIENT_PUBLIC_KEY"))]
    client_public_key: String,

    /// Key used by server to decrypt incoming KX messages from the client.
    /// This key should be kept secret.
    #[arg(long, env("WAYGATE_SERVER_SECRET_KEY"))]
    server_secret_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    log4rs::init_file("config/logging.yml", Default::default()).unwrap();
    log::info!("Bootstrapping with config {config:#?}");

    let config = Arc::new(config);

    let database = Pool::<Postgres>::connect(&config.database).await?;
    sqlx::migrate!("./migrations").run(&database).await?;
    log::info!("Initialized database");

    let services = Arc::new(GameServices::new(database.clone())?);

    tokio::select! {
        _ = serve_websockets(config.clone(), database.clone(), services.clone()) => {
            log::info!("Websocket server stopped listening");
        },
        _ = serve_api(config.clone(), database.clone(), services.clone()) => {
            log::info!("API server stopped listening");
        },
    };

    Ok(())
}

/// Serve the websocket server the game uses for its matchmaking protocol.
async fn serve_websockets(
    config: Arc<Config>,
    database: Pool<Postgres>,
    services: Arc<GameServices>,
) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(&config.as_ref().bind).await?;

    while let Ok((stream, peer_address)) = listener.accept().await {
        let config = config.clone();
        let database = database.clone();
        let services = services.clone();

        tokio::spawn(LogContext::with(async move {
            LogContext::insert("peer_address", peer_address.to_string());

            log::info!(
                context:serde = LogContext::current();
                "Started serving client. remote = {peer_address}."
            );

            match serve_client(stream, peer_address, config, database, services).await {
                Ok(_) => {
                    log::info!(
                        context:serde = LogContext::current();
                        "Client closed connection."
                    )
                }
                Err(e) => log::error!(
                    context:serde = LogContext::current(),
                    error:? = e;
                    "Caught error while serving client."
                ),
            };
        }));

        log::info!("Incoming connection from: {peer_address}");
    }

    Ok(())
}

/// Serve API for control over the instance.
async fn serve_api(
    config: Arc<Config>,
    database: Pool<Postgres>,
    services: Arc<GameServices>,
) -> Result<(), Box<dyn Error>> {
    {
        let config = config.clone();
        let state = web::Data::new(AppState { database, services });

        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .wrap(CheckKey::new(&config.api_key))
                .service(healthcheck)
                .service(get_ban)
                .service(post_ban)
                .service(delete_ban)
                .service(get_ban_by_id)
                .service(announcement)
        })
    }
    .bind(&config.api_bind)?
    .run()
    .await?;

    Ok(())
}

#[derive(Error, Debug)]
enum ClientServeError {
    #[error("Client stream closed while reading.")]
    StreamClosed,
    #[error("Client violated init protocol.")]
    ProtocolViolation,
    #[error("Client didn't send valid session ticket.")]
    InvalidSessionTicket,
}

/// Serves a single game client.
async fn serve_client(
    tcp_stream: TcpStream,
    peer_address: SocketAddr,
    config: Arc<Config>,
    database: Pool<Postgres>,
    services: Arc<GameServices>,
) -> Result<(), Box<dyn Error>> {
    // Sample steam ID, steam session ticket and waygate version off of the HTTP header.
    let external_id = Arc::from(OnceLock::new());
    let session_ticket = Arc::from(OnceLock::new());
    let waygate_version = Arc::from(OnceLock::new());
    let header_callback = {
        let external_id = external_id.clone();
        let session_ticket = session_ticket.clone();
        let waygate_version = waygate_version.clone();

        move |req: &Request, response: Response| {
            for (header, value) in req.headers() {
                if header.as_str() == "x-steam-id" {
                    let value = value
                        .to_str()
                        .expect("Upgrade request missing X-STEAM-ID header");
                    external_id.set(value.to_string()).unwrap();
                }

                if header.as_str() == "x-steam-session-ticket" {
                    let value = value
                        .to_str()
                        .expect("Upgrade request missing X-STEAM-SESSION-TICKET header");
                    session_ticket.set(value.to_string()).unwrap();
                }

                if header.as_str() == "x-waygate-client-version" {
                    let value = value
                        .to_str()
                        .expect("Upgrade request missing X-WAYGATE-CLIENT-VERSION header");
                    waygate_version.set(value.to_string()).unwrap();
                }
            }

            Ok(response)
        }
    };

    // Upgrade HTTP request to websockets
    let (mut sink, mut stream) = tokio_tungstenite::accept_hdr_async(tcp_stream, header_callback)
        .await?
        .split();

    let external_id = external_id.get().unwrap();
    let session_ticket = session_ticket.get().unwrap();
    let waygate_version = waygate_version.get().unwrap();

    LogContext::insert("external_id", external_id);
    LogContext::insert("waygate_version", waygate_version);

    let parsed_external_id = external_id.parse::<u64>()?;

    let is_banned = match services
        .bans
        .get_ban(&format!("{parsed_external_id}"))
        .await?
    {
        Some(ban_record) => {
            let banned_at = UNIX_EPOCH
                .checked_add(std::time::Duration::from_secs(ban_record.banned_at as u64))
                .ok_or("Time overflow when calculating ban timestamp")?;
            // reject all connections for 120 seconds from the time of the ban to prevent reconnection
            if banned_at.elapsed().unwrap_or_default() < std::time::Duration::from_secs(120) {
                log::info!(
                    context:serde = LogContext::current();
                    "Banned client reconnection attempt, disconnecting..."
                );
                return Ok(());
            }
            true
        }
        None => false,
    };

    // Handle protocol stuff first like exchanging keys and shit
    let mut protocol = ClientProtocol::new(
        database.clone(),
        format!("{parsed_external_id:x?}"),
        peer_address.to_string(),
        config.as_ref().try_into()?,
    )?;

    // Cycle over stream playing the messaging against our protocol statemachine
    while let Some(event) = stream.next().await {
        let Ok(Message::Binary(data)) = event else {
            return Err(Box::new(ClientServeError::ProtocolViolation));
        };

        if let Some(buffer) = protocol.transition(&data).await? {
            sink.send(Message::Binary(buffer.into())).await?;
        }

        if protocol.finalized() {
            break;
        }
    }

    let parsed_session_ticket =
        hex_to_bytes(session_ticket).ok_or(ClientServeError::InvalidSessionTicket)?;
    let _steam_session = services
        .steam
        .start_session(parsed_external_id, &parsed_session_ticket)?;

    // Start serving the, at this point, fully authenticated client.
    let (push_tx, push_rx) = channel::<Vec<u8>>();
    let mut handler = if is_banned {
        ActiveHandler::Banned(BannedClientHandler::default())
    } else {
        ActiveHandler::Default(DefaultClientHandler::new(
            services.as_ref(),
            push_tx,
            protocol.session_details().unwrap(),
        ))
    };

    while let Some(event) = stream.next().await {
        if let ActiveHandler::Default(_) = handler {
            if (services
                .bans
                .get_ban(&format!("{parsed_external_id}"))
                .await?)
                .is_some()
            {
                log::info!(
                    context:serde = LogContext::current();
                    "Client was banned while connected, disconnecting..."
                );
                return Ok(());
            }
        }
        match event {
            Ok(Message::Close(_)) => {
                log::debug!(
                    context:serde = LogContext::current();
                    "Close message."
                );
                return Ok(());
            }
            Ok(Message::Binary(data)) => {
                // Shove any outbound push messages down the sink.
                while let Ok(push_message) = push_rx.try_recv() {
                    let encrypted = protocol.encrypt_message(&push_message)?;
                    sink.send(Message::Binary(encrypted.into())).await?;
                }

                // Decrypt received buffer against session parameters for plaintext message.
                let decrypted = protocol.decrypt_message(data.as_ref())?;

                // Handle the contents of the message we've received.
                let reader = MessageReader::new(&decrypted);
                match reader.message_type()? {
                    MessageType::Request => {
                        let sequence = reader.sequence()?.unwrap();
                        let builder = MessageBuilder::response().sequence(sequence);
                        let deserialized = &reader.deserialize_request()?;

                        // Dump packets if required
                        #[cfg(feature = "packet-dump")]
                        {
                            let label = deserialized.name();
                            let file_name = format!(
                                "dump/{}_{}_{}_{}.mmbin",
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("Time went backwards")
                                    .as_millis(),
                                external_id,
                                sequence,
                                label,
                            );
                            std::fs::write(file_name, decrypted).unwrap();
                        }

                        // Dispatch request to handler for specific message type.
                        let response = match match &mut handler {
                            ActiveHandler::Default(h) => h.dispatch_request(deserialized).await,
                            ActiveHandler::Banned(h) => h.dispatch_request(deserialized).await,
                        } {
                            Ok(Some(response)) => builder.body(response).build()?,
                            Err(e) => {
                                log::error!(
                                    context:serde = LogContext::current(),
                                    error:? = e;
                                    "Error while processing request"
                                );
                                builder.error(0).build()?
                            }
                            Ok(None) => builder.error(0).build()?,
                        };

                        // Send response back to client.
                        let encrypted = protocol.encrypt_message(&response)?;
                        sink.send(Message::Binary(encrypted.into())).await?;
                    }

                    // Clients send a push message type to confirm that they've received some
                    // server-originating push message.
                    MessageType::Push => {
                        log::debug!(
                            context:serde = LogContext::current();
                            "Handled push confirmation"
                        );
                    }

                    // Clients periodically send these. Client will disconnect if we dont
                    // send one for too long.
                    MessageType::Heartbeat => {
                        let encrypted = protocol.encrypt_message(MessageBuilder::heartbeat())?;
                        sink.send(Message::Binary(encrypted.into())).await?;
                    }

                    _ => {}
                }
            }
            _ => log::warn!(
                context:serde = LogContext::current();
                "Unknown websocket message type {event:#?}"
            ),
        }
    }

    Err(Box::new(ClientServeError::StreamClosed))
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len().is_multiple_of(2) {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}
