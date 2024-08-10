use std::io;
use tokio::net as tokionet;

mod rpc;
mod push;
mod steam;
mod client;
mod session;
mod config;
mod api;

#[tokio::main]
async fn main () -> Result<(), io::Error> {
    log4rs::init_file("logging.toml", Default::default()).unwrap();

    let config = config::get();
    log::debug!("Loaded config: {:#?}", config);

    waygate_database::init(config.database_url.as_str())
        .await.expect("Could not initialize database");

    steam::init().expect("Could not initialize steam");

    tokio::select! {
        _ = api::start_api() => {
            log::warn!("Shut down API");
        },
        _= listener(config.bind.to_owned()) => {
            log::warn!("Shut down game RPC");
        },
    };

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
