use waygate_connection::ClientSession;

use waygate_message::armoredcore6::*;

use crate::HandlerResult;

pub async fn handle_delete_session(
    _session: ClientSession,
) -> HandlerResult {
    Ok(ResponseParams::DeleteSession)
}
