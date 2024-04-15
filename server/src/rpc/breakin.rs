use fnrpc::breakin::*;
use fnrpc::push::JoinPayload;
use fnrpc::push::PushInvaderJoiningParams;
use fnrpc::push::PushJoinParams;
use fnrpc::push::PushJoiningAsInvaderParams;
use fnrpc::push::PushParams;
use fnrpc::shared::ObjectIdentifier;
use fnrpc::ResponseParams;
use rand::Rng;

use crate::pool::breakin::InvasionHostMatcher;
use crate::pool::breakin::InvasionPoolEntry;
use crate::pool::breakin::InvasionPoolQuery;
use crate::pool::invasion_pool;
use crate::pool::pool::MatchResult;
use crate::push;
use crate::rpc;
use crate::rpc::encode_external_id;
use crate::session::ClientSession;

impl From<RequestGetBreakInTargetListParams> for InvasionPoolQuery {
    fn from(value: RequestGetBreakInTargetListParams) -> Self {
        InvasionPoolQuery {
            character_level: value.matching_parameters.soul_level,
            weapon_level: value.matching_parameters.max_reinforce,
        }
    }
}

pub async fn handle_get_break_in_target_list(
    params: RequestGetBreakInTargetListParams,
) -> rpc::HandlerResult {
    log::debug!("RequestGetBreakInTargetListParams: {:?}", params);

    let entries = invasion_pool()
        .match_entries::<_, InvasionHostMatcher>(&params.into())
        .iter()
        .map(|m| m.into())
        .collect();

    Ok(ResponseParams::GetBreakInTargetList(
        ResponseGetBreakInTargetListParams { unk1: 0x0, entries },
    ))
}

pub async fn handle_break_in_target(
    session: ClientSession,
    request: RequestBreakInTargetParams,
) -> rpc::HandlerResult {
    let push_payload = PushParams::Join(PushJoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::InvaderJoining(PushInvaderJoiningParams {
            invader_player_id: session.player_id,
            invader_steam_id: encode_external_id(&session.external_id),
            unk1: 0x0,
            unk2: 0x0,
            play_region: 6100000,
        }),
    });

    Ok(
        push::send_push(request.player_id, push_payload)
            .await
            .map(|_| ResponseParams::BreakInTarget)?
    )
}

pub async fn handle_allow_break_in_target(
    _session: ClientSession,
    request: RequestAllowBreakInTargetParams,
) -> rpc::HandlerResult {
    let push_payload = PushParams::Join(PushJoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::JoiningAsInvader(PushJoiningAsInvaderParams {
            host_player_id: request.player_id,
            join_data: request.join_data,
            unk1: 0x0,
        }),
    });

    Ok(
        push::send_push(request.player_id, push_payload)
            .await
            .map(|_| ResponseParams::AllowBreakInTarget)?
    )
}

impl Into<ResponseGetBreakInTargetListParamsEntry> for &MatchResult<InvasionPoolEntry> {
    fn into(self) -> ResponseGetBreakInTargetListParamsEntry {
        ResponseGetBreakInTargetListParamsEntry {
            player_id: self.0.1 as i32,
            steam_id: self.1.steam_id.clone(),
        }
    }
}

