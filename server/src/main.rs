#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::{
    error::Error,
    net::SocketAddr,
    sync::{mpsc::channel, Arc},
    time::{Duration, Instant, UNIX_EPOCH},
};

use actix_web::{web, App, HttpServer};
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use message::{
    builder::MessageBuilder, eldenring::ServerError, reader::MessageReader, MessageType,
};
use server::api::{
    auth::CheckKey,
    ban::{delete_ban, get_ban, get_ban_by_id, post_ban},
    health::healthcheck,
    notification::announcement,
    AppState,
};
use server::handler::{
    eldenring::{ActiveHandler, BannedClientHandler, DefaultClientHandler},
    RequestHandler,
};
use server::logging::{self, LogContext};
use server::protocol::ClientProtocol;
use server::services::eldenring::GameServices;
use sqlx::{Pool, Postgres};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::{
    error::ProtocolError,
    handshake::server::{Request, Response},
    Error as WsError, Message,
};

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
    dotenvy::dotenv().ok();
    let config = Config::parse();
    log4rs::init_file("config/logging.yml", Default::default()).unwrap();
    log::info!("Bootstrapping with config {config:#?}");

    let config = Arc::new(config);

    let database = Pool::<Postgres>::connect(&config.database).await?;
    sqlx::migrate!("./migrations").run(&database).await?;
    log::info!("Initialized database");

    let services = Arc::new(GameServices::new(database.clone())?);

    tokio::spawn(sweep_expired_task(services.clone()));

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

async fn sweep_expired_task(services: Arc<GameServices>) {
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(10));
    loop {
        ticker.tick().await;
        services.sweep_expired();
    }
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
                .wrap(logging::LogContextMiddleware)
                .service(
                    web::scope("")
                        .wrap(CheckKey::new(&config.api_key))
                        .service(healthcheck)
                        .service(get_ban)
                        .service(post_ban)
                        .service(delete_ban)
                        .service(get_ban_by_id)
                        .service(announcement),
                )
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
}

#[derive(Default)]
struct ClientHeaders {
    steam_id: Option<String>,
    waygate_version: Option<String>,
}

/// Serves a single game client.
async fn serve_client(
    tcp_stream: TcpStream,
    peer_address: SocketAddr,
    config: Arc<Config>,
    database: Pool<Postgres>,
    services: Arc<GameServices>,
) -> Result<(), Box<dyn Error>> {
    let mut client_headers = ClientHeaders::default();

    #[allow(clippy::result_large_err)]
    let header_callback = |req: &Request,
                           response: Response|
     -> Result<
        Response,
        tokio_tungstenite::tungstenite::http::Response<Option<String>>,
    > {
        let headers = req.headers();

        client_headers.steam_id = headers
            .get("x-steam-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        client_headers.waygate_version = headers
            .get("x-waygate-client-version")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        if let Some(v) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
            if let Some(first) = v.split(',').next() {
                LogContext::insert("peer_address", first.trim().to_string());
            }
        }
        if let Some(v) = headers.get("cf-ipcountry").and_then(|v| v.to_str().ok()) {
            LogContext::insert("ip_country", v.to_string());
        }

        if client_headers.steam_id.is_none() || client_headers.waygate_version.is_none() {
            let err_response = tokio_tungstenite::tungstenite::http::Response::builder()
                .status(tokio_tungstenite::tungstenite::http::StatusCode::BAD_REQUEST)
                .body(Some("Missing required headers".to_string()))
                .unwrap();

            return Err(err_response);
        }

        Ok(response)
    };

    LogContext::insert("peer_address", peer_address.to_string());

    let (mut sink, mut stream) = tokio_tungstenite::accept_hdr_async(tcp_stream, header_callback)
        .await?
        .split();

    log::info!(
        context:serde = LogContext::current();
        "Started serving client."
    );

    let steam_id = client_headers.steam_id.unwrap();
    let waygate_version = client_headers.waygate_version.unwrap();
    LogContext::insert("external_id", &steam_id);
    LogContext::insert("waygate_version", &waygate_version);

    let parsed_steam_id = steam_id.parse::<u64>()?;
    let external_id = format!("{parsed_steam_id:x?}");

    if let Some(ban) = services.bans.get_ban(&external_id).await? {
        LogContext::insert("banned_at", ban.banned_at.to_string());
        if UNIX_EPOCH
            .checked_add(std::time::Duration::from_secs(ban.banned_at as u64))
            .ok_or("Time overflow when calculating ban timestamp")?
            .elapsed()
            .unwrap_or_default()
            < std::time::Duration::from_secs(120)
        {
            log::info!(
                context:serde = LogContext::current();
                "Banned client reconnection attempt, disconnecting..."
            );
            return Ok(());
        }
    }

    // Handle protocol stuff first like exchanging keys and shit
    let mut protocol = ClientProtocol::new(
        database.clone(),
        external_id,
        parsed_steam_id,
        peer_address.to_string(),
        (
            config.server_secret_key.as_str(),
            config.client_public_key.as_str(),
        )
            .try_into()?,
        services.clone(),
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

    let is_banned = protocol.is_banned();

    // Start serving the, at this point, fully authenticated client.
    let (push_tx, push_rx) = channel::<Vec<u8>>();
    let mut handler = if is_banned {
        ActiveHandler::Banned(BannedClientHandler::default())
    } else {
        ActiveHandler::Default(Box::new(DefaultClientHandler::new(
            services.as_ref(),
            push_tx,
            protocol.session_details().unwrap(),
        )))
    };

    const BAN_CHECK_INTERVAL: Duration = Duration::from_secs(15);
    let mut last_ban_check = Instant::now();

    serve_client_loop(
        &mut stream,
        &mut sink,
        &mut protocol,
        &mut handler,
        &services,
        &mut last_ban_check,
        BAN_CHECK_INTERVAL,
        parsed_steam_id,
        push_rx,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn serve_client_loop(
    stream: &mut (impl futures_util::Stream<Item = Result<Message, WsError>> + Unpin),
    sink: &mut (impl futures_util::Sink<Message, Error = WsError> + Unpin),
    protocol: &mut ClientProtocol,
    handler: &mut ActiveHandler<'_>,
    services: &Arc<GameServices>,
    last_ban_check: &mut std::time::Instant,
    ban_check_interval: std::time::Duration,
    steam_id: u64,
    push_rx: std::sync::mpsc::Receiver<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let mut kick_pending = false;

    while let Some(event) = stream.next().await {
        if matches!(handler, ActiveHandler::Default(_))
            && last_ban_check.elapsed() >= ban_check_interval
        {
            *last_ban_check = std::time::Instant::now();
            if services.bans.is_banned(steam_id).await? {
                log::info!(
                    context:serde = LogContext::current();
                    "Client was banned while connected, upgrading to banned handler..."
                );
                *handler = ActiveHandler::Banned(BannedClientHandler::default());
                kick_pending = true;
            }
        }
        match event {
            Ok(Message::Close(_))
            | Err(WsError::Protocol(ProtocolError::ResetWithoutClosingHandshake)) => {
                log::debug!(
                    context:serde = LogContext::current();
                    "Close message."
                );
                return Ok(());
            }
            Err(e) => {
                log::error!(
                    context:serde = LogContext::current(),
                    error:? = e;
                    "Error reading from client websocket."
                );
                return Err(Box::new(e));
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

                        if kick_pending {
                            let response =
                                builder.error(ServerError::LoginRequired as u32).build()?;
                            let encrypted = protocol.encrypt_message(&response)?;
                            sink.send(Message::Binary(encrypted.into())).await?;
                            kick_pending = false;
                            continue;
                        }

                        let message_variant = reader
                            .peek_variant()?
                            .map_or("unknown".to_string(), |v| v.to_string());

                        LogContext::insert("message_variant", &message_variant);

                        // Dump packets if required
                        #[cfg(feature = "packet-dump")]
                        {
                            let label = reader
                                .peek_variant()?
                                .map_or("unknown".to_string(), |v| v.to_string());
                            let file_name = format!(
                                "dump/{}_{}_{}_{}.mmbin",
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("Time went backwards")
                                    .as_millis(),
                                steam_id,
                                sequence,
                                label,
                            );
                            std::fs::write(file_name, &decrypted).unwrap();
                        }

                        let deserialized = &reader.deserialize_request()?;

                        // Dispatch request to handler for specific message type.
                        let response = match match handler {
                            ActiveHandler::Default(h) => h.dispatch_request(deserialized).await,
                            ActiveHandler::Banned(h) => h.dispatch_request(deserialized).await,
                        } {
                            Ok(Some(response)) => builder.body(response).build()?,
                            Err(e) => {
                                let code = e.to_wire_code();
                                log::error!(
                                    context:serde = LogContext::current(),
                                    error:? = e;
                                    "Error while processing request"
                                );
                                builder.error(code).build()?
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
