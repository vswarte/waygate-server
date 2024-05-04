use fnrpc::RequestParams;

use crate::{client::ClientError, rpc::*, session::ClientSession};

pub async fn dispatch_request(
    session: ClientSession,
    request: RequestParams,
) -> HandlerResult {
    log::debug!("Player sent request. request_type = {}", request.name());

    Ok(match request {
        RequestParams::DeleteSession => session::handle_delete_session(session).await?,

        // Client throws a steamworks error if this is returns an error
        RequestParams::RegisterUGC
            => ugc::handle_register_ugc().await?,

        RequestParams::UpdateLoginPlayerCharacter
            => character::handle_update_login_player_character().await?,

        RequestParams::UpdatePlayerStatus(p)
            => player::handle_update_player_status(session, *p).await?,

        RequestParams::GetAnnounceMessageList(_)
            => announcement::handle_get_announce_message_list().await?,

        RequestParams::CreateMatchingTicket(p)
            => matchingticket::handle_create_matching_ticket(session, *p).await?,

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

        RequestParams::GetBreakInTargetList(p)
            => breakin::handle_get_break_in_target_list(*p).await?,
        RequestParams::BreakInTarget(p)
            => breakin::handle_break_in_target(session, *p).await?,
        RequestParams::AllowBreakInTarget(p)
            => breakin::handle_allow_break_in_target(session, *p).await?, 
        RequestParams::RejectBreakInTarget(p)
            => breakin::handle_reject_break_in_target(session, *p).await?, 

        RequestParams::GetItemLog(p)
            => player::handle_get_item_log(session, *p).await?, 
        RequestParams::UseItemLog(p)
            => player::handle_use_item_log(session, *p).await?, 
        RequestParams::KillEnemyLog(p)
            => player::handle_kill_enemy_log(session, *p).await?, 

        RequestParams::JoinMultiplay(p)
            => player::handle_join_multiplay(session, *p).await?, 

        RequestParams::GrGetPlayerEquipments(p)
            => player_equipments::handle_gr_get_player_equipments(*p).await?, 

        _ => return Err(Box::new(ClientError::NoHandler)),
    })
}
