use waygate_connection::ClientSession;
use waygate_message::*;

use crate::HandlerResult;

pub async fn handle_update_player_status(
    session: ClientSession,
    request: RequestUpdatePlayerStatusParams
) -> HandlerResult {
    {
        let mut game_session = session.game_session_mut();
        // YOLO it for now, client will reject unwanted invasions anyways
        game_session.invadeable = request.character.online_activity == 0x1;
        game_session.matching = Some((&request).into());
    }

    session.update_invadeability()?;

    Ok(ResponseParams::UpdatePlayerStatus)
}
