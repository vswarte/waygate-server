use std::net;
use std::sync;
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
    let steam_id = sync::Arc::new(sync::Mutex::new(String::new()));
    let steam_session_ticket = sync::Arc::new(sync::Mutex::new(String::new()));

    let header_callback = {
        let steam_id = steam_id.clone();
        let steam_session_ticket = steam_session_ticket.clone();

        move |req: &handshake::server::Request, response: handshake::server::Response| {
            for (ref header, value) in req.headers() {
                // Get steam ID from header
                if header.as_str() == "x-steam-id" {
                    let value = String::from(value.to_str().unwrap());
                    let mut lock = steam_id.lock().unwrap();
                    *lock = value;
                }

                // Get steam session ticket from header
                if header.as_str() == "x-steam-session-ticket" {
                    let value = String::from(value.to_str().unwrap());
                    let mut lock = steam_session_ticket.lock().unwrap();
                    *lock = value;
                }
            }

            Ok(response)
        }
    };

    let transport = tokio_tungstenite::accept_hdr_async(stream, header_callback)
        .await.map_err(|_| ClientError::Transport(TransportError::AcceptFailed))?;

    let external_id = steam_id.clone().lock().unwrap().to_owned();
    let session_ticket = steam_session_ticket.clone().lock().unwrap().to_owned();

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
