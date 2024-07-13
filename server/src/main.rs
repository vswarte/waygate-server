use std::io;
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
    log4rs::init_file("logging.toml", Default::default()).unwrap();

    let config = config::get();
    log::debug!("Loaded config: {:#?}", config);

    database::init(config.database_url.as_str())
        .await.expect("Could not initialize database");

    steam::init().expect("Could not initialize steam");
    pool::init_pools().expect("Could not initialize pools");

    listener(config.bind.to_owned())
        .await.expect("Could not bind to socket");

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
