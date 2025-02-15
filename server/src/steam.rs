use std::{sync::mpsc::{channel, Sender}, time::Duration};

use steamworks::{Server, SteamAPIInitError, SteamId};

pub struct SteamServer {
    server: Server,
    session_end_tx: Sender<SteamId>,
}

impl SteamServer {
    pub fn init() -> Result<Self, SteamAPIInitError> {
        let (server, single) = steamworks::Server::init(
            "127.0.0.1".parse().unwrap(),
            10901,
            3333,
            steamworks::ServerMode::AuthenticationAndSecure,
            ""
        )?;

        let (session_end_tx, session_end_rx) = channel();
        {
            let server = server.clone();
            tokio::spawn(async move {
                log::info!("Starting steam poll loop");

                loop {
                    // End any queued session for ending
                    while let Ok(steam_id) = session_end_rx.try_recv() {
                        server.end_authentication_session(steam_id);
                    }

                    single.run_callbacks();
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            });
        }

        Ok(Self { server, session_end_tx })
    }

    pub fn start_session(&self, steam_id: u64, ticket: &[u8]) -> Result<SteamSession, steamworks::AuthSessionError> {
        let steam_id = SteamId::from_raw(steam_id);
        self.server.begin_authentication_session(steam_id, ticket)?;

        // TODO: listen for ValidateAuthTicket callbacks

        Ok(SteamSession {
            steam_id,
            session_end_tx: self.session_end_tx.clone(),
        })
    }
}

pub struct SteamSession {
    steam_id: SteamId,
    session_end_tx: Sender<SteamId>,
}

impl Drop for SteamSession {
    fn drop(&mut self) {
        log::info!("Request steam session end. steam_id = {}.", self.steam_id.raw());
        if let Err(e) = self.session_end_tx.send(self.steam_id) {
            log::error!("Could not send session end request to steam server. {e}.");
        }
    }
}
