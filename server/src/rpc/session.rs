use std::error::Error;

use fnrpc::session::*;
use fnrpc::shared::ObjectIdentifier;
use fnrpc::ResponseParams;

use crate::database;
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
    })))
}

pub async fn handle_restore_session(
    external_id: &str,
    params: RequestRestoreSessionParams,
) -> Result<(session::ClientSession, ResponseParams), Box<dyn Error>> {
    log::info!(
        "Player sent RestoreSession. external_id = {}.",
        external_id,
    );

    let session = session::get_client_session(
        params.session_data.identifier.object_id,
        &params.session_data.cookie,
    ).await?;

    let session_id = session.session_id;
    let valid_from = session.valid_from;
    let valid_until = session.valid_until;
    let cookie = session.cookie.clone();

    Ok((session, ResponseParams::RestoreSession(ResponseRestoreSessionParams {
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
    })))
}

pub async fn handle_delete_session(
    session: session::ClientSession,
) -> rpc::HandlerResult {
    log::info!("Player sent DeleteSession. player = {}", session.player_id);

    let mut connection = database::acquire().await?;
    sqlx::query("DELETE FROM bloodmessages WHERE player_id = $1")
        .bind(session.player_id)
        .fetch_all(&mut *connection)
        .await?;

    Ok(ResponseParams::DeleteSession)
}
