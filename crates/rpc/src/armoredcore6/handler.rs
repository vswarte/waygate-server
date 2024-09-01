use waygate_message::armoredcore6::request::RequestParams;
use waygate_connection::{ClientError, ClientSession};
use waygate_message::armoredcore6::ResponseParams;

use crate::HandlerResult;
use crate::armoredcore6::{announcement, character, matchingticket, session, ugc};

pub async fn handle_request(
    session: ClientSession,
    request: RequestParams,
) -> HandlerResult {
    tracing::debug!(
        "dispatch_request player = {}, type= {}",
        session.player_id,
        request.name(),
    );

    Ok(match request {
        RequestParams::DeleteSession => session::handle_delete_session(session).await?,

        // Client throws a steamworks error if this is returns an error but it
        // seems unused otherwise.
        RequestParams::RegisterUGC
            => ugc::handle_register_ugc().await?,

        RequestParams::UpdateLoginPlayerCharacter
            => character::handle_update_login_player_character().await?,


        RequestParams::GetAnnounceMessageList(_)
            => announcement::handle_get_announce_message_list().await?,

        RequestParams::PollMatchingTicket
            => matchingticket::handle_poll_matching_ticket().await?,
        RequestParams::CreateMatchingTicket(_)
            => matchingticket::handle_create_matching_ticket().await?,

        RequestParams::UploadFamilySharingInfo
            => ResponseParams::UploadFamilySharingInfo,

        _ => {
            return Err(Box::new(ClientError::NoHandler(request.name().to_string())))
        },
    })
}
