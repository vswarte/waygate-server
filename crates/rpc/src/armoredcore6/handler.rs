use waygate_message::armoredcore6::request::RequestParams;
use waygate_connection::{ClientError, ClientSession};
use waygate_message::armoredcore6::ResponseParams;

use crate::HandlerResult;
use crate::armoredcore6::{announcement, character, matchingticket, player, room, session, ugc};

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
        RequestParams::DeleteSession
            => session::handle_delete_session(session).await?,

        RequestParams::RegisterUGC(p)
            => ugc::handle_register_ugc(p).await?,

        RequestParams::GetUGCStatus(p)
            => ugc::handle_get_ugc_status(p).await?,

        RequestParams::UpdateLoginPlayerCharacter
            => character::handle_update_login_player_character().await?,

        RequestParams::GetAnnounceMessageList(_)
            => announcement::handle_get_announce_message_list().await?,

        RequestParams::PollMatchingTicket
            => matchingticket::handle_poll_matching_ticket().await?,

        RequestParams::UploadGuardITCode
            => ResponseParams::UploadGuardITCode,

        RequestParams::UploadFamilySharingInfo
            => ResponseParams::UploadFamilySharingInfo,

        RequestParams::CreateRoom(p)
            => room::handle_create_room(session, p).await?,

        RequestParams::UpdateRoom(p)
            => room::handle_update_room(session, p).await?,

        RequestParams::DeleteRoom(p)
            => room::handle_delete_room(session, p).await?,

        RequestParams::GetRoomList(p)
            => room::handle_get_room_list(p).await?,

        RequestParams::GenerateRoomBattleID(p)
            => room::handle_generate_room_battle_id(p).await?,

        RequestParams::GetRankingOrder(p)
            => player::handle_get_ranking_order(p).await?,

        RequestParams::GetRatingStatus
            => ugc::handle_get_rating_status().await?,

        _ => {
            return Err(Box::new(ClientError::NoHandler(request.name().to_string())))
        },
    })
}
