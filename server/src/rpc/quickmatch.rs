use fnrpc::push::PushPlayerJoiningQuickMatchParams;
use fnrpc::push::PushJoiningQuickMatchParams;
use fnrpc::push::PushParams;
use fnrpc::push::PushJoinParams;
use fnrpc::shared;
use fnrpc::quickmatch::*;
use fnrpc::ResponseParams;
use fnrpc::push::JoinPayload;
use rand::Rng;

use crate::rpc;
use crate::push;
use crate::session::ClientSession;

use super::encode_external_id;

pub async fn handle_search_quick_match(
    params: RequestSearchQuickMatchParams,
) -> rpc::HandlerResult {
    todo!();
    // Ok(ResponseParams::SearchQuickMatch(ResponseSearchQuickMatchParams {
    //     items: repository::quickmatch::quickmatch_search(params.arena_id, params.quickmatch_settings)
    //         .iter()
    //         .map(|m| ResponseSearchQuickMatchParamsEntry {
    //             host_player_id: m.host_id,
    //             arena_id: m.arena_id,
    //             host_steam_id: "".to_string(),
    //         })
    //     .collect(),
    //     unk1: 0x0,
    // }))
}

pub async fn handle_register_quick_match(
    session: ClientSession,
    params: RequestRegisterQuickMatchParams,
) -> rpc::HandlerResult {
    todo!();
    // let _quickmatch_id = repository::quickmatch::register_quickmatch(
    //     &params,
    //     &session.player_id,
    // );

    Ok(ResponseParams::RegisterQuickMatch)
}

pub async fn handle_unregister_quick_match(
    session: ClientSession,
) -> rpc::HandlerResult {
    todo!();
    // repository::quickmatch::unregister_quickmatches_for_player(&session.player_id)
    //     .map_err(|_| 0x0 as u32)?;

    Ok(ResponseParams::UnregisterQuickMatch)
}

pub async fn handle_update_quick_match() -> rpc::HandlerResult {
    // TODO: ???
    Ok(ResponseParams::UpdateQuickMatch)
}

pub async fn handle_join_quick_match(
    session: ClientSession,
    request: RequestJoinQuickMatchParams,
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
    session: ClientSession,
    request: RequestAcceptQuickMatchParams,
) -> rpc::HandlerResult {
    todo!();
    // let quickmatch = repository::quickmatch::get_quick_match(session.player_id)
    //     .ok_or(0 as u32)?;
    //
    // let push_payload = PushParams::Join(PushJoinParams {
    //     identifier: shared::ObjectIdentifier {
    //         object_id: rand::thread_rng().gen::<i32>(),
    //         secondary_id: rand::thread_rng().gen::<i32>(),
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
