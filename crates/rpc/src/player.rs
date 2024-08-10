use waygate_connection::{ClientSession, ClientSessionContainer};
use waygate_message::*;

use crate::HandlerResult;

pub async fn handle_update_player_status(
    session: ClientSession,
    request: RequestUpdatePlayerStatusParams
) -> HandlerResult {
    {
        let mut session = session.lock_write();
        // YOLO it for now, client will reject unwanted invasions anyways
        session.invadeable = request.character.online_activity == 0x1;

        session.matching = Some((&request).into());

        session.update_invadeability()?;
    }

    Ok(ResponseParams::UpdatePlayerStatus)
}
