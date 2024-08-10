use std::sync::{Arc, OnceLock};
use std::net::SocketAddr;
use std::io;
use std::error::Error;

use clap::Parser;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::handshake::server::{Request, Response};
use waygate_api::serve_api;
use waygate_config::init_config;
use waygate_connection::{new_client, ClientError, TransportError};
use waygate_database::init_database;
use waygate_steam::{begin_session, init_steamworks};
use waygate_rpc::handle_request;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    log4rs::init_file("logging.toml", Default::default()).unwrap();

    init_config().expect("Could not initialize config");

    init_database(args.database.as_str())
        .await
        .expect("Could not initialize database");

    init_steamworks().expect("Could not initialize steam");

    tokio::select! {
        _ = serve_api(
            args.api_bind.as_str(),
            args.api_key.as_str(),
        ) => {
            tracing::warn!("Shut down API");
        },
        _= serve_game(args.bind.as_str()) => {
            tracing::warn!("Shut down game RPC");
        },
    };

    Ok(())
}

async fn serve_game(bind: &str) -> Result<(), io::Error> {
    let listener = TcpListener::bind(&bind).await?;
    while let Ok((stream, peer_address)) = listener.accept().await {
        tokio::spawn(async move {
            match handle_connection(stream, peer_address).await {
                Ok(_) => tracing::info!(
                    "Peer disconnected. peer_addres = {}",
                    peer_address.to_string(),
                ),
                Err(e) => tracing::error!(
                    "Peer connection crashed. peer_address = {}, e = {:?}",
                    peer_address.to_string(),
                    e,
                ),
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    peer_address: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    // Acquire the steam ID and session ticket from the opening request
    let steam_id: Arc<OnceLock<String>> = Default::default();
    let session_ticket: Arc<OnceLock<String>> = Default::default();

    let header_callback = {
        let steam_id = steam_id.clone();
        let session_ticket = session_ticket.clone();

        move |req: &Request, response: Response| {
            for (header, value) in req.headers() {
                if header.as_str() == "x-steam-id" {
                    steam_id.set(String::from(value.to_str().unwrap())).unwrap();
                }

                if header.as_str() == "x-steam-session-ticket" {
                    session_ticket
                        .set(String::from(value.to_str().unwrap()))
                        .unwrap();
                }

                if header.as_str() == "x-waygate-client-version" {
                    tracing::info!("Waygate client version. {:?}", value.to_str());
                }
            }

            Ok(response)
        }
    };

    let transport = tokio_tungstenite::accept_hdr_async(stream, header_callback)
        .await
        .map_err(|_| ClientError::Transport(TransportError::AcceptFailed))?;

    let external_id = steam_id.get().unwrap();
    let session_ticket = session_ticket.get().unwrap();

    let steam_session = match begin_session(external_id.as_str(), session_ticket.as_str()) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!("Got error while starting steam session: {:?}", e);
            return Err(Box::new(ClientError::Credentials));
        }
    };

    let mut client = new_client(steam_session, peer_address, transport.into())
        .await_hello()
        .await?
        .advertise_crypto_session()
        .await?
        .await_public_key()
        .await?
        .await_session_message()
        .await?
        .confirm_session(handle_request)
        // .confirm_session(handle_banned)
        .await?;

    loop {
        client.serve().await?;
    }
}
