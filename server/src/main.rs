use std::{
    error::Error,
    net::SocketAddr,
    sync::{mpsc::channel, Arc, OnceLock},
};

use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use handler::{eldenring::DefaultClientHandler, RequestHandler};
use message::{builder::MessageBuilder, reader::MessageReader, MessageType};
use protocol::ClientProtocol;
use services::eldenring::GameServices;
use sqlx::{Pool, Postgres};
use steam::SteamServer;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::tungstenite::Message;

mod handler;
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
    log4rs::init_file("config/logging.yaml", Default::default()).unwrap();
    log::info!("Bootstrapping with config {config:#?}");

    let config = Arc::new(config);

    let database = Pool::<Postgres>::connect(&config.database).await?;
    sqlx::migrate!("./migrations").run(&database).await?;
    log::info!("Initialized database");

    let services = Arc::new(GameServices::new(database.clone())?);

    serve_websockets(config.clone(), database.clone(), services).await?;

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

        tokio::spawn(async move {
            log::info!("Started serving client. remote = {peer_address}.");

            match serve_client(stream, peer_address, config, database, services).await {
                Ok(_) => {
                    log::info!("Client closed connection. remote = {peer_address}.");
                }
                Err(e) => log::error!(
                    "Caught error while serving client. remote = {peer_address}. e = {e:?}."
                ),
            };
        });

        log::info!("Incoming connection from: {peer_address}");
    }

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
                    external_id
                        .set(String::from(value.to_str().unwrap()))
                        .unwrap();
                }

                if header.as_str() == "x-steam-session-ticket" {
                    session_ticket
                        .set(String::from(value.to_str().unwrap()))
                        .unwrap();
                }

                if header.as_str() == "x-waygate-client-version" {
                    waygate_version
                        .set(String::from(value.to_str().unwrap()))
                        .unwrap();
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
    let _waygate_version = waygate_version.get().unwrap();

    let parsed_external_id = external_id.parse::<u64>()?;

    // Handle protocol stuff first like exchanging keys and shit
    let mut protocol = ClientProtocol::new(
        database.clone(),
        format!("{:x?}", parsed_external_id),
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
    let mut handler = DefaultClientHandler::new(
        services.as_ref(),
        push_tx,
        protocol.session_details().unwrap(),
    );

    while let Some(event) = stream.next().await {
        match event {
            Ok(Message::Close(_)) => {
                log::debug!("Close message. remote = {peer_address}.");
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
                                handler.session.external_id,
                                sequence,
                                label,
                            );
                            std::fs::write(file_name, decrypted).unwrap();
                        }

                        // Dispatch request to handler to get reponse.
                        let response = match handler.dispatch_request(deserialized).await {
                            Ok(Some(response)) => builder.body(response).build()?,
                            Err(e) => {
                                log::error!("Error while processing request. remote = {peer_address}. error = {e}");
                                builder.error(0).build()?
                            }
                            Ok(None) => builder.error(0).build()?,
                        };

                        // Throw response back at client.
                        let encrypted = protocol.encrypt_message(&response)?;
                        sink.send(Message::Binary(encrypted.into())).await?;
                    }

                    // Clients send a push message type to confirm that they've received some
                    // server-originating push message.
                    MessageType::Push => {
                        log::info!("Push confirmation from {}", handler.session.peer_address);
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
            _ => {
                log::debug!("Unknown websocket message type {event:#?}");
            }
        }
    }

    Err(Box::new(ClientServeError::StreamClosed))
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
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
