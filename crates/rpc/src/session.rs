use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

use waygate_connection::ClientSession;

use waygate_message::*;

use crate::HandlerResult;

pub async fn handle_delete_session(
    _session: ClientSession,
) -> HandlerResult {
    Ok(ResponseParams::DeleteSession)
}
