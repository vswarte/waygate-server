use core::fmt;
use std::sync;

use steamworks::SteamAPIInitError;
use thiserror::Error;

static STEAM_SERVER: sync::OnceLock<steamworks::Server> = sync::OnceLock::new();

#[derive(Debug, Error)]
pub enum SteamError {
    #[error("Could not initialize steamworks server")]
    Initialize(#[from] steamworks::SteamAPIInitError),

    #[error("Steam ID was invalid")]
    InvalidSteamID,

    #[error("Steam session ticket was invalid")]
    InvalidTicket,

    #[error("Could not authenticate user")]
    SteamAuth(#[from] steamworks::AuthSessionError),
}

pub fn init() -> Result<(), SteamAPIInitError> {

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

impl fmt::Display for SteamSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.steamid32())
    }
}

pub fn begin_session(
    steam_id: &str,
    ticket: &str,
) -> Result<SteamSession, SteamError> {
    let steam_id = steam_id.parse::<u64>()
        .map(|id| steamworks::SteamId::from_raw(id))
        .map_err(|_| SteamError::InvalidSteamID)?;

    let ticket = hex_to_bytes(ticket)
        .ok_or(SteamError::InvalidTicket)?;

    log::info!("Starting steam session for {:?}", steam_id);

    STEAM_SERVER.get()
        .expect("Could not get steam API instace")
        .begin_authentication_session(steam_id, ticket.as_slice())?;

    Ok(SteamSession(steam_id))
}

impl Drop for SteamSession {
    fn drop(&mut self) {
        log::info!("Ending steam session for {:?}", self);

        STEAM_SERVER.get()
            .expect("Could not get steam API instace")
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
