use fnrpc::quickmatch::*;
use fnrpc::ResponseParams;

use crate::pool::quickmatch_pool;
use crate::pool::quickmatch::QuickmatchPoolEntry;
use crate::pool::quickmatch::QuickmatchPoolQuery;
use crate::pool::MatchResult;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

pub async fn handle_search_quick_match(
    request: RequestSearchQuickMatchParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::SearchQuickMatch(ResponseSearchQuickMatchParams {
        matches: quickmatch_pool()?
            .match_entries::<QuickmatchPoolQuery>(&request.into())
            .iter()
            .map(|m| m.into())
            .collect(),
        unk1: 0x0,
    }))
}

pub async fn handle_register_quick_match(
    session: ClientSession,
    request: RequestRegisterQuickMatchParams,
) -> rpc::HandlerResult {
    let _key = quickmatch_pool()?
        .insert(session.lock_read().player_id, request.into())?;

    Ok(ResponseParams::RegisterQuickMatch)
}

pub async fn handle_unregister_quick_match(
    _session: ClientSession,
) -> rpc::HandlerResult {
    todo!();
    // repository::quickmatch::unregister_quickmatches_for_player(&session.player_id)
    //     .map_err(|_| 0x0 as u32)?;
    //
    // Ok(ResponseParams::UnregisterQuickMatch)
}

pub async fn handle_update_quick_match() -> rpc::HandlerResult {
    // TODO: Set new params if received, error if otherwise as this'll prompt a 
    // recreate.
    Ok(ResponseParams::UpdateQuickMatch)
}

pub async fn handle_join_quick_match(
    _session: ClientSession,
    _request: RequestJoinQuickMatchParams,
) -> rpc::HandlerResult {
    todo!();
    // let quickmatch = repository::quickmatch::get_quick_match(request.host_player_id)
    //     .ok_or(0 as u32)?;

    // let push_payload = PushParams::Join(PushJoinParams {
    //     identifier: shared::ObjectIdentifier {
    //         object_id: rand::thread_rng().gen::<i32>(),
    //         secondary_id: rand::thread_rng().gen::<i32>(),
    //     },
    //     join_payload: JoinPayload::PlayerJoiningQuickMatch(PushPlayerJoiningQuickMatchParams {
    //         quickmatch_settings: quickmatch.quickmatch_settings,
    //         joining_player_id: session.player_id,
    //         joining_player_steam_id: encode_external_id(&session.external_id),
    //         unk2: 0x3,
    //         arena_id: quickmatch.arena_id,
    //         unk3: 0x0,
    //         password: request.password.clone(),
    //     })
    // });
    //
    // match push::send_push(quickmatch.host_id, push_payload).await {
    //     Err(_) => Err(0 as u32),
    //     Ok(_) => Ok(ResponseParams::JoinQuickMatch)
    // }
}

pub async fn handle_accept_quick_match(
    _session: ClientSession,
    _request: RequestAcceptQuickMatchParams,
) -> rpc::HandlerResult {
    todo!();
    // let quickmatch = repository::quickmatch::get_quick_match(session.player_id)
    //     .ok_or(0 as u32)?;
    //
    // let push_payload = PushParams::Join(PushJoinParams {
    //     identifier: shared::ObjectIdentifier {
    //         object_id: rand::thread_rng().gen::<i32>(),
    //         seconda rand::thread_rng().gen::<i32>(),
    //     },
    //     join_payload: JoinPayload::JoiningQuickMatch(PushJoiningQuickMatchParams {
    //         quickmatch_settings: quickmatch.quickmatch_settings,
    //         host_player_id: quickmatch.host_id,
    //         host_steam_id: encode_external_id(&session.external_id),
    //         join_data: request.join_data,
    //     })
    // });
    //
    // // TODO: check if player requested a join
    // match push::send_push(request.joining_player_id, push_payload).await {
    //     Err(_) => Err(0 as u32),
    //     Ok(_) => Ok(ResponseParams::AcceptQuickMatch)
    // }
}

impl From<RequestRegisterQuickMatchParams> for QuickmatchPoolEntry {
    fn from(val: RequestRegisterQuickMatchParams) -> Self {
        QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: val.matching_parameters.soul_level,
            weapon_level: val.matching_parameters.max_reinforce,
            arena_id: val.arena_id,
            password: val.matching_parameters.password.clone(),
            settings: val.quickmatch_settings,
        }

    }
}

impl From<&MatchResult<QuickmatchPoolEntry>> for ResponseSearchQuickMatchParamsEntry {
    fn from(val: &MatchResult<QuickmatchPoolEntry>) -> Self {
        ResponseSearchQuickMatchParamsEntry {
            host_player_id: val.0.1,
            host_steam_id: val.1.external_id.clone(),
            arena_id: val.1.arena_id,
        }
    }
}

impl From<RequestSearchQuickMatchParams> for QuickmatchPoolQuery {
    fn from(value: RequestSearchQuickMatchParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level,
            weapon_level: value.matching_parameters.max_reinforce,
            password: value.matching_parameters.password.clone(),
            arena_id: value.arena_id,
            settings: value.quickmatch_settings,
        }
    }
}
