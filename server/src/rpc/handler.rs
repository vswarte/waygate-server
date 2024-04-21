use std::pin::Pin;

use futures::Future;
use fnrpc::RequestParams;

use crate::{rpc::*, session::ClientSession};

pub type HandlerTask = Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

pub fn spawn_handling_task(
    session: ClientSession,
    request: RequestParams,
) -> Option<HandlerTask> {
    Some(match request {
        RequestParams::DeleteSession
            => Box::pin(session::handle_delete_session(session)),

        // Client throws a steamworks error if this is returns an error
        RequestParams::RegisterUGC
            => Box::pin(ugc::handle_register_ugc()),

        RequestParams::UpdateLoginPlayerCharacter
            => Box::pin(character::handle_update_login_player_character()),

        RequestParams::UpdatePlayerStatus(p)
            => Box::pin(player::handle_update_player_status(session, p)),

        RequestParams::GetAnnounceMessageList(_)
            => Box::pin(announcement::handle_get_announce_message_list()),

        RequestParams::CreateMatchingTicket(p)
            => Box::pin(matchingticket::handle_create_matching_ticket(session, p)),

        RequestParams::CreateBloodstain(p)
            => Box::pin(bloodstain::handle_create_bloodstain(session, p)),
        RequestParams::GetBloodstainList(p)
            => Box::pin(bloodstain::handle_get_bloodstain_list(p)),
        RequestParams::GetDeadingGhost(p)
            => Box::pin(bloodstain::handle_get_deading_ghost(p)),

        RequestParams::CreateGhostData(p)
            => Box::pin(ghostdata::handle_create_ghostdata(session, p)),
        RequestParams::GetGhostDataList(p)
            => Box::pin(ghostdata::handle_get_ghostdata_list(p)),

        RequestParams::CreateBloodMessage(p)
            => Box::pin(bloodmessage::handle_create_blood_message(session, p)),
        RequestParams::GetBloodMessageList(p)
            => Box::pin(bloodmessage::handle_get_blood_message_list(p)),
        RequestParams::EvaluateBloodMessage(p)
            => Box::pin(bloodmessage::handle_evaluate_blood_message(p)),
        RequestParams::ReentryBloodMessage(p)
            => Box::pin(bloodmessage::handle_reentry_blood_message(p)),
        RequestParams::RemoveBloodMessage(p)
            => Box::pin(bloodmessage::handle_remove_blood_message(session, p)),

        RequestParams::CreateSign(p)
            => Box::pin(sign::handle_create_sign(session, p)),
        RequestParams::GetSignList(p)
            => Box::pin(sign::handle_get_sign_list(session, p)),
        RequestParams::SummonSign(p)
            => Box::pin(sign::handle_summon_sign(session, p)),
        RequestParams::RejectSign(_)
            => Box::pin(sign::handle_reject_sign(session)),
        RequestParams::RemoveSign(p)
            => Box::pin(sign::handle_remove_sign(session, p)),
        RequestParams::UpdateSign(p)
            => Box::pin(sign::handle_update_sign(session,p)),

        RequestParams::SearchQuickMatch(p)
            => Box::pin(quickmatch::handle_search_quick_match(p)),
        RequestParams::RegisterQuickMatch(p)
            => Box::pin(quickmatch::handle_register_quick_match(session, p)),
        RequestParams::UnregisterQuickMatch
            => Box::pin(quickmatch::handle_unregister_quick_match(session)),
        RequestParams::UpdateQuickMatch
            => Box::pin(quickmatch::handle_update_quick_match()),
        RequestParams::JoinQuickMatch(p)
            => Box::pin(quickmatch::handle_join_quick_match(session, p)),
        RequestParams::AcceptQuickMatch(p)
            => Box::pin(quickmatch::handle_accept_quick_match(session, p)),

        RequestParams::GetBreakInTargetList(p)
            => Box::pin(breakin::handle_get_break_in_target_list(p)),
        RequestParams::BreakInTarget(p)
            => Box::pin(breakin::handle_break_in_target(session, p)),
        RequestParams::AllowBreakInTarget(p)
            => Box::pin(breakin::handle_allow_break_in_target(session, p)), 

        RequestParams::GetItemLog(p)
            => Box::pin(player::handle_get_item_log(session, p)), 
        RequestParams::UseItemLog(p)
            => Box::pin(player::handle_use_item_log(session, p)), 
        RequestParams::KillEnemyLog(p)
            => Box::pin(player::handle_kill_enemy_log(session, p)), 

        RequestParams::GrGetPlayerEquipments(p)
            => Box::pin(player_equipments::handle_gr_get_player_equipments(p)), 

        _ => return None,
    })
}
