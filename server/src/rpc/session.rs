use std::error::Error;

use fnrpc::session::*;
use fnrpc::ResponseParams;

use crate::rpc;
use crate::session;

// TODO: actually handle the request in some capacity
pub async fn handle_create_session(
    external_id: &str,
    _params: RequestCreateSessionParams,
) -> Result<(session::ClientSession, ResponseParams), Box<dyn Error>> {
    let session = session::new_client_session(external_id).await?;

    let player_id = session.player_id;
    let session_id = session.session_id;
    let valid_from = session.valid_from;
    let valid_until = session.valid_until;
    let cookie = session.cookie.clone();

    Ok((session, ResponseParams::CreateSession(ResponseCreateSessionParams {
        player_id,
        steam_id: external_id.to_string(),
        ip_address: String::from(""),
        unk: 0x0,
        session_id,
        valid_from,
        valid_until,
        cookie,
        unk_string: String::from(""),
    })))
}

pub async fn handle_restore_session(
    external_id: &str,
    params: RequestRestoreSessionParams,
) -> Result<(session::ClientSession, ResponseParams), Box<dyn Error>> {
    log::info!(
        "Player sent RestoreSession. external_id = {}. params = {:#?}.",
        external_id,
        params,
    );

    let session = session::new_client_session(external_id).await?;

    let session_id = session.session_id;
    let valid_from = session.valid_from;
    let valid_until = session.valid_until;
    let cookie = session.cookie.clone();

    Ok((session, ResponseParams::RestoreSession(ResponseRestoreSessionParams {
        unk: 0x0,
        session_id,
        valid_from,
        valid_until,
        cookie,
        unk_string: String::from(""),
    })))
}

pub async fn handle_delete_session(
    session: session::ClientSession,
) -> rpc::HandlerResult {
    log::info!("Player sent DeleteSession. player = {}", session.player_id);

    // TODO: do actual session cleanup stuff
    Ok(ResponseParams::DeleteSession)
}
