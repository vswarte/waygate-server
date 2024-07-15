use std::net;
use std::sync::Arc;
use std::sync::OnceLock;
use tungstenite::handshake;
use tokio::net as tokionet;

use crate::steam;
use crate::client::new_client;
use crate::client::ClientError;
use crate::client::transport::TransportError;

pub async fn handle(
    stream: tokionet::TcpStream,
    peer_address: net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    // Acquire the steam ID and session ticket from the opening request
    let steam_id: Arc<OnceLock<String>> = Default::default();
    let session_ticket: Arc<OnceLock<String>> = Default::default();

    let header_callback = {
        let steam_id = steam_id.clone();
        let session_ticket = session_ticket.clone();

        move |req: &handshake::server::Request, response: handshake::server::Response| {
            for (header, value) in req.headers() {
                if header.as_str() == "x-steam-id" {
                    steam_id.set(String::from(value.to_str().unwrap())).unwrap();
                }

                if header.as_str() == "x-steam-session-ticket" {
                    session_ticket.set(String::from(value.to_str().unwrap())).unwrap();
                }

                if header.as_str() == "x-waygate-client-version" {
                    log::info!("Waygate client version. {:?}", value.to_str());
                }
            }

            Ok(response)
        }
    };

    let transport = tokio_tungstenite::accept_hdr_async(stream, header_callback)
        .await.map_err(|_| ClientError::Transport(TransportError::AcceptFailed))?;

    let external_id = steam_id.get().unwrap();
    let session_ticket = session_ticket.get().unwrap();

    let steam_session = match steam::begin_session(
        external_id.as_str(),
        session_ticket.as_str()
    ) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("Got error while starting steam session: {:?}", e);
            return Err(Box::new(ClientError::Credentials));
        }
    };

    let mut client = new_client(
            steam_session,
            peer_address,
            transport.into(),
        )
        .await_hello().await?
        .advertise_crypto_session().await?
        .await_public_key().await?
        .await_session_message().await?
        .confirm_session().await?;
    
    loop {
        client.serve().await?;
    }
}
