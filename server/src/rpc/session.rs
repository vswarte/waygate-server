use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

use crate::rpc;
use crate::session;
use crate::session::ClientSession;

use waygate_message::*;

// TODO: actually handle the request in some capacity
pub async fn handle_create_session(
    external_id: String,
    _params: RequestCreateSessionParams,
) -> Result<(ClientSession, ResponseParams), Box<dyn Error>> {
    let session = session::new_client_session(external_id.clone()).await?;

    let player_id = session.player_id;
    let session_id = session.session_id;
    let valid_from = session.valid_from;
    let valid_until = session.valid_until;
    let cookie = session.cookie.clone();

    Ok((
        Arc::new(RwLock::new(session)),
        ResponseParams::CreateSession(ResponseCreateSessionParams {
            player_id,
            steam_id: external_id,
            ip_address: String::from(""),
            session_data: SessionData {
                identifier: ObjectIdentifier {
                    object_id: session_id,
                    secondary_id: 0x0,
                },
                valid_from,
                valid_until,
                cookie,
            },
            redirect_url: String::from(""),
        })
    ))
}

pub async fn handle_restore_session(
    external_id: String,
    params: RequestRestoreSessionParams,
) -> Result<(ClientSession, ResponseParams), Box<dyn Error>> {
    log::info!(
        "Player sent RestoreSession. external_id = {}.",
        external_id,
    );

    let session = session::get_client_session(
        external_id,
        params.session_data.identifier.object_id,
        &params.session_data.cookie,
    ).await?;

    let session_id = session.session_id;
    let valid_from = session.valid_from;
    let valid_until = session.valid_until;
    let cookie = session.cookie.clone();

    Ok((
        Arc::new(RwLock::new(session)),
        ResponseParams::RestoreSession(ResponseRestoreSessionParams {
            session_data: SessionData {
                identifier: ObjectIdentifier {
                    object_id: session_id,
                    secondary_id: 0x0,
                },
                valid_from,
                valid_until,
                cookie,
            },
            unk_string: String::from(""),
        })
    ))
}

pub async fn handle_delete_session(
    _session: ClientSession,
) -> rpc::HandlerResult {
    Ok(ResponseParams::DeleteSession)
}
