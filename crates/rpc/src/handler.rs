use crate::*;

use waygate_message::*;
use waygate_connection::{ClientSession, ClientSessionContainer};

pub async fn handle_request(
    session: ClientSession,
    request: RequestParams,
) -> HandlerResult {
    tracing::debug!(
        "dispatch_request player = {}, type= {}",
        session.lock_read().player_id,
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

        RequestParams::UpdatePlayerStatus(p)
            => player::handle_update_player_status(session, *p).await?,

        RequestParams::GetAnnounceMessageList(_)
            => announcement::handle_get_announce_message_list().await?,

        RequestParams::PollMatchingTicket
            => matchingticket::handle_poll_matching_ticket().await?,
        RequestParams::CreateMatchingTicket(_)
            => matchingticket::handle_create_matching_ticket().await?,

        RequestParams::CreateBloodstain(p)
            => bloodstain::handle_create_bloodstain(session, *p).await?,
        RequestParams::GetBloodstainList(p)
            => bloodstain::handle_get_bloodstain_list(*p).await?,
        RequestParams::GetDeadingGhost(p)
            => bloodstain::handle_get_deading_ghost(*p).await?,

        RequestParams::CreateGhostData(p)
            => ghostdata::handle_create_ghostdata(session, *p).await?,
        RequestParams::GetGhostDataList(p)
            => ghostdata::handle_get_ghostdata_list(*p).await?,

        RequestParams::CreateBloodMessage(p)
            => bloodmessage::handle_create_blood_message(session, *p).await?,
        RequestParams::GetBloodMessageList(p)
            => bloodmessage::handle_get_blood_message_list(*p).await?,
        RequestParams::EvaluateBloodMessage(p)
            => bloodmessage::handle_evaluate_blood_message(*p).await?,
        RequestParams::ReentryBloodMessage(p)
            => bloodmessage::handle_reentry_blood_message(*p).await?,
        RequestParams::RemoveBloodMessage(p)
            => bloodmessage::handle_remove_blood_message(session, *p).await?,

        RequestParams::CreateSign(p)
            => sign::handle_create_sign(session, *p).await?,
        RequestParams::GetSignList(p)
            => sign::handle_get_sign_list(session, *p).await?,
        RequestParams::SummonSign(p)
            => sign::handle_summon_sign(session, *p).await?,
        RequestParams::RejectSign(p)
            => sign::handle_reject_sign(session, *p).await?,
        RequestParams::RemoveSign(p)
            => sign::handle_remove_sign(session, *p).await?,
        RequestParams::UpdateSign(p)
            => sign::handle_update_sign(session,*p).await?,
        RequestParams::GetMatchAreaSignList(p)
            => sign::handle_get_match_area_sign_list(*p).await?,
        RequestParams::CreateMatchAreaSign(p)
            => sign::handle_create_match_area_sign(session, *p).await?,

        RequestParams::SearchQuickMatch(p)
            => quickmatch::handle_search_quick_match(*p).await?,
        RequestParams::RegisterQuickMatch(p)
            => quickmatch::handle_register_quick_match(session, *p).await?,
        RequestParams::UnregisterQuickMatch
            => quickmatch::handle_unregister_quick_match(session).await?,
        RequestParams::UpdateQuickMatch
            => quickmatch::handle_update_quick_match(session).await?,
        RequestParams::JoinQuickMatch(p)
            => quickmatch::handle_join_quick_match(session, *p).await?,
        RequestParams::AcceptQuickMatch(p)
            => quickmatch::handle_accept_quick_match(session, *p).await?,
        RequestParams::QuickMatchResultLog
            => quickmatch::handle_quick_match_result_log(session).await?,
        RequestParams::SendQuickMatchStart
            => quickmatch::handle_send_quick_match_start(session).await?,
        RequestParams::SendQuickMatchResult
            => quickmatch::handle_send_quick_match_result(session).await?,

        RequestParams::GetBreakInTargetList(p)
            => breakin::handle_get_break_in_target_list(*p).await?,
        RequestParams::BreakInTarget(p)
            => breakin::handle_break_in_target(session, *p).await?,
        RequestParams::AllowBreakInTarget(p)
            => breakin::handle_allow_break_in_target(session, *p).await?, 
        RequestParams::RejectBreakInTarget(p)
            => breakin::handle_reject_break_in_target(session, *p).await?, 

        RequestParams::GrUploadPlayerEquipments(p)
            => player_equipments::handle_gr_upload_player_equipments(session, *p).await?, 
        RequestParams::GrGetPlayerEquipments(p)
            => player_equipments::handle_gr_get_player_equipments(*p).await?, 

        _ => {
            return Err(Box::new(ClientError::NoHandler(request.name().to_string())))
        },
    })
}
