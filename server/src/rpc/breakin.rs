use fnrpc::push::AllowBreakInTargetParams;
use fnrpc::push::BreakInTargetParams;
use fnrpc::push::JoinPayload;
use fnrpc::push::RejectBreakInTargetParams;
use rand::prelude::*;

use fnrpc::breakin::*;
use fnrpc::push::JoinParams;
use fnrpc::push::PushParams;
use fnrpc::shared::ObjectIdentifier;
use fnrpc::ResponseParams;

use crate::pool::breakin::BreakInPoolEntry;
use crate::pool::breakin::BreakInPoolQuery;
use crate::pool::breakin_pool;
use crate::pool::MatchResult;
use crate::push;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

impl From<RequestGetBreakInTargetListParams> for BreakInPoolQuery {
    fn from(value: RequestGetBreakInTargetListParams) -> Self {
        BreakInPoolQuery {
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
        }
    }
}

pub async fn handle_get_break_in_target_list(
    params: RequestGetBreakInTargetListParams,
) -> rpc::HandlerResult {
    let play_region = params.play_region;
    let entries = breakin_pool()?
        .match_entries::<BreakInPoolQuery>(&params.into())
        .iter()
        .map(|m| m.into())
        .collect::<Vec<_>>();

    Ok(ResponseParams::GetBreakInTargetList(
        ResponseGetBreakInTargetListParams {
            play_region,
            entries,
        },
    ))
}

pub async fn handle_break_in_target(
    session: ClientSession,
    request: RequestBreakInTargetParams,
) -> rpc::HandlerResult {
    let (player_id, external_id) = {
        let lock = session.lock_read();
        (lock.player_id, lock.external_id.clone())
    };

    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::BreakInTarget(BreakInTargetParams {
            invader_player_id: player_id,
            invader_steam_id: external_id,
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
    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::AllowBreakInTarget(AllowBreakInTargetParams {
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

pub async fn handle_reject_break_in_target(
    session: ClientSession,
    request: RequestRejectBreakInTargetParams,
) -> rpc::HandlerResult {
    let (player_id, steam_id) = {
        let lock = session.lock_read();
        (lock.player_id, lock.external_id.clone())
    };

    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::RejectBreakInTarget(RejectBreakInTargetParams {
            host_player_id: player_id,
            unk1: -90,
            host_steam_id: steam_id,
            unk2: 0,
        }),
    });

    Ok(
        push::send_push(request.invading_player_id, push_payload)
            .await
            .map(|_| ResponseParams::RejectBreakInTarget)?
    )
}

impl From<&MatchResult<BreakInPoolEntry>> for ResponseGetBreakInTargetListParamsEntry {
    fn from(val: &MatchResult<BreakInPoolEntry>) -> Self {
        ResponseGetBreakInTargetListParamsEntry {
            player_id: val.0.1,
            steam_id: val.1.steam_id.clone(),
        }
    }
}
