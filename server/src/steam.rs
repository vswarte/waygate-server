use std::sync;

use steamworks::SteamAPIInitError;
use thiserror::Error;

static STEAM_SERVER: sync::OnceLock<steamworks::Server> = sync::OnceLock::new();

#[derive(Debug, Error)]
pub enum SteamError {
    #[error("Could not initialize steamworks server")]
    Initialize(#[from] SteamAPIInitError),

    #[error("Steam ID was invalid")]
    InvalidSteamID,

    #[error("Steam session ticket was invalid")]
    InvalidTicket,

    #[error("Could not authenticate user")]
    SteamAuth(#[from] steamworks::AuthSessionError),
}

pub fn init() -> Result<(), SteamError> {
    let (server, _) = steamworks::Server::init(
        "127.0.0.1".parse().unwrap(),
        10901,
        3333,
        steamworks::ServerMode::NoAuthentication,
        ""
    )?;

    // steamworks::Server doesn't implement std::fmt::Debug...
    match STEAM_SERVER.set(server) {
        Ok(_) => {},
        Err(_) => panic!("Could not initialize STEAM_SERVER static"),
    }

    log::info!("Initialized steamworks");

    Ok(())
}

#[derive(Debug)]
pub struct SteamSession(steamworks::SteamId);

impl std::fmt::Display for SteamSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0.raw())
    }
}

pub fn begin_session(
    steam_id: &str,
    ticket: &str,
) -> Result<SteamSession, SteamError> {
    let steam_id = steam_id.parse::<u64>()
        .map(steamworks::SteamId::from_raw)
        .map_err(|_| SteamError::InvalidSteamID)?;

    let ticket = hex_to_bytes(ticket)
        .ok_or(SteamError::InvalidTicket)?;

    log::debug!("Starting steam session for {:?}", steam_id);

    let result = STEAM_SERVER.get()
        .expect("Could not get steam API instace")
        .begin_authentication_session(steam_id, ticket.as_slice());

    if let Err(error) = result {
        match error {
            steamworks::AuthSessionError::DuplicateRequest => {
                log::warn!("Got duplicate request, attempting to clear the previous one");
                STEAM_SERVER.get()
                    .expect("Could not get steam API instace")
                    .end_authentication_session(steam_id);

                STEAM_SERVER.get()
                    .expect("Could not get steam API instace")
                    .begin_authentication_session(steam_id, ticket.as_slice())?;

                log::warn!("Reattempting the auth");
            },
            _ => return Err(SteamError::SteamAuth(error)),
        }
    }

    Ok(SteamSession(steam_id))
}

impl Drop for SteamSession {
    fn drop(&mut self) {
        log::debug!("Ending steam session for {:?}", self);

        STEAM_SERVER.get()
            .expect("Could not get steam API instance")
            .end_authentication_session(self.0);
    }
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2)
                .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}
