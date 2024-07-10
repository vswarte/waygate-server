use std::io;
use std::env;
use tokio::net as tokionet;

mod database;
mod rpc;
mod push;
mod pool;
mod steam;
mod client;
mod session;
mod config;

#[tokio::main]
async fn main () -> Result<(), io::Error> {
    dotenvy::dotenv().expect("Could not init env vars");
    env_logger::init();
    config::init().expect("Could not load config");

    database::init().await.expect("Could not initialize database");
    steam::init().expect("Could not initialize steam");
    pool::init_pools().expect("Could not initialize pools");

    let bind = env::args()
        .nth(1)
        .unwrap_or("0.0.0.0:10901".to_string());

    listener(bind).await.expect("Could not bind to socket");

    Ok(())
}

async fn listener(bind: String) -> Result<(), io::Error> {
    let listener = tokionet::TcpListener::bind(&bind).await?;
    while let Ok((stream, peer_address)) = listener.accept().await {
        tokio::spawn(async move {
            match client::connection::handle(stream, peer_address).await {
                Ok(_) => log::info!(
                    "Peer disconnected. peer_addres = {}",
                    peer_address.to_string(),
                ),
                Err(e) => log::error!(
                    "Peer connection crashed. peer_address = {}, e = {:?}",
                    peer_address.to_string(),
                    e,
                ),
            }
        });
    }

    Ok(())
}
