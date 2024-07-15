use crate::pool::breakin::BreakInPoolEntry;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

use super::message::*;

pub async fn handle_update_player_status(
    session: ClientSession,
    request: RequestUpdatePlayerStatusParams
) -> rpc::HandlerResult {
    {
        let mut session = session.lock_write();
        // YOLO it for now, client will reject unwanted invasions anyways
        session.invadeable = request.character.online_activity == 0x1;
        session.matching = Some((&request).into());

        session.update_invadeability()?;
    }

    Ok(ResponseParams::UpdatePlayerStatus)
}

impl From<(String, &RequestUpdatePlayerStatusParams)> for BreakInPoolEntry {
    fn from(value: (String, &RequestUpdatePlayerStatusParams)) -> Self {
        Self {
            character_level: value.1.character.level,
            weapon_level: value.1.character.max_reinforce_level,
            steam_id: value.0.to_string(),
        }
    }
}
