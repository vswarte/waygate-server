use rand::prelude::*;

use waygate_connection::send_push;
use waygate_connection::ClientSession;
use waygate_message::*;
use waygate_pool::breakin::BreakInPoolQuery;
use waygate_pool::PoolError;
use waygate_pool::BREAKIN_POOL;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::HandlerResult;

/// Sent by invaders to retrieve a list of invadeable hosts.
pub async fn handle_get_break_in_target_list(
    params: RequestGetBreakInTargetListParams,
) -> HandlerResult {
    let play_region = params.play_region;
    let mut entries = BREAKIN_POOL
        .match_entries::<BreakInPoolQuery>(&params.into())
        .iter()
        .map(|m| m.into())
        .collect::<Vec<_>>();
    entries.shuffle(&mut thread_rng());

    Ok(ResponseParams::GetBreakInTargetList(
        ResponseGetBreakInTargetListParams {
            play_region,
            entries,
        },
    ))
}

/// Sent by invaders when they have selected a host from the list returned by
/// `handle_get_break_in_target_list`. This causes the server to send a
/// `BreakInTarget` push message to the selected hosted with the invaders details.
pub async fn handle_break_in_target(
    session: ClientSession,
    request: RequestBreakInTargetParams,
) -> HandlerResult {
    let target = BREAKIN_POOL
        .by_topic_id(request.player_id)
        .ok_or(PoolError::NotFound)?;
    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::BreakInTarget(BreakInTargetParams {
            invader_player_id: session.player_id,
            invader_steam_id: session.external_id.clone(),
            unk1: 0x0,
            unk2: 0x0,
            play_region: target.play_region,
        }),
    });

    Ok(send_push(request.player_id, push_payload)
        .await
        .map(|_| ResponseParams::BreakInTarget)?)
}

/// Sent by the host after receiving a `BreakInTarget` push to share the
/// lobby data such that the invader can join.
pub async fn handle_allow_break_in_target(
    _session: ClientSession,
    request: RequestAllowBreakInTargetParams,
) -> HandlerResult {
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

    Ok(send_push(request.player_id, push_payload)
        .await
        .map(|_| ResponseParams::AllowBreakInTarget)?)
}

/// Sent by the host to reject a `BreakInTarget` push. This might happen
/// if the host has died since the invader selected the host or the lobby is
/// already at capacity.
pub async fn handle_reject_break_in_target(
    session: ClientSession,
    request: RequestRejectBreakInTargetParams,
) -> HandlerResult {
    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::RejectBreakInTarget(RejectBreakInTargetParams {
            host_player_id: session.player_id,
            unk1: -90,
            host_steam_id: session.external_id.clone(),
            unk2: 0,
        }),
    });

    Ok(send_push(request.invading_player_id, push_payload)
        .await
        .map(|_| ResponseParams::RejectBreakInTarget)?)
}
