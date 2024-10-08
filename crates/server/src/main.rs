use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, OnceLock};

use clap::Parser;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::handshake::server::{Request, Response};
use waygate_api::serve_api;
use waygate_config::init_config;
use waygate_connection::{init_crypto, new_client, ClientError, TransportError};
use waygate_database::init_database;
use waygate_rpc::handle_request;
use waygate_session::is_banned;
use waygate_steam::{begin_session, init_steamworks};

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
async fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    log4rs::init_file("logging.toml", Default::default()).unwrap();

    init_config().expect("Could not initialize config");

    init_crypto(
        args.client_public_key.as_str(),
        args.server_secret_key.as_str(),
    )
    .expect("Could not initialize crypto");

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
                    "Client disconnected. peer_addres = {}",
                    peer_address.to_string(),
                ),
                Err(e) => tracing::error!(
                    "Client connection was closed. peer_address = {}, e = {:?}",
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

    let external_id = steam_id.get().ok_or(ClientError::Credentials)?;
    let session_ticket = session_ticket.get().ok_or(ClientError::Credentials)?;

    let steam_session = match begin_session(external_id.as_str(), session_ticket.as_str()) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!("Got error while starting steam session: {:?}", e);
            return Err(Box::new(ClientError::Credentials));
        }
    };

    tracing::info!("Authenticated steam session. external_id = {external_id}");

    // Check if the external ID is on a banlist *after* authenticating the app
    // ticket. If the player is banned we gracefully close the connection.
    if is_banned(external_id).await? {
        tracing::info!("Player is banned. Closing connection. {external_id}");
        return Ok(());
    }

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
