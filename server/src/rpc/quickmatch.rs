use fnrpc::push::AcceptQuickMatchParams;
use fnrpc::push::JoinParams;
use fnrpc::push::JoinPayload;
use fnrpc::push::PushParams;
use fnrpc::quickmatch::*;
use fnrpc::shared;
use fnrpc::ResponseParams;
use rand::Rng;

use crate::pool;
use crate::pool::quickmatch_pool;
use crate::pool::quickmatch::QuickmatchPoolEntry;
use crate::pool::quickmatch::QuickmatchPoolQuery;
use crate::pool::MatchResult;
use crate::pool::PoolError;
use crate::push;
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
    let mut session = session.lock_write();

    let entry: QuickmatchPoolEntry = QuickmatchPoolEntry::from_request(
        request,
        session.external_id.clone(),
    );

    log::info!("QuickmatchPoolEntry: {:#?}", entry);
    let key = quickmatch_pool()?
        .insert(session.player_id, entry)?;

    session.quickmatch = Some(key);
    Ok(ResponseParams::RegisterQuickMatch)
}

pub async fn handle_unregister_quick_match(
    session: ClientSession,
) -> rpc::HandlerResult {
    if let Some(key) = session.lock_write().quickmatch.take() {
        quickmatch_pool()?.remove(&key)?;
    }

    Ok(ResponseParams::UnregisterQuickMatch)
}

pub async fn handle_update_quick_match(
    session: ClientSession,
) -> rpc::HandlerResult {
    // Current we just check if the quickmatch is known in-memory at all.
    let _existing = session.lock_write().quickmatch.as_ref()
        .ok_or(PoolError::NotFound)?;

    Ok(ResponseParams::UpdateQuickMatch)
}

pub async fn handle_join_quick_match(
    session: ClientSession,
    request: RequestJoinQuickMatchParams,
) -> rpc::HandlerResult {
    let quickmatch = pool::quickmatch_pool()?
        .by_topic_id(request.host_player_id)
        .ok_or(std::io::Error::from(std::io::ErrorKind::Other))?;

    let (joining_player_id, joining_player_steam_id) = {
        let session = session.lock_read();

        (session.player_id, session.external_id.clone())
    };

    let push_payload = PushParams::Join(JoinParams {
        identifier: shared::ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::JoinQuickMatch(fnrpc::push::JoinQuickMatchParams {
            quickmatch_settings: quickmatch.settings,
            joining_player_id,
            joining_player_steam_id,
            unk2: 0x3,
            arena_id: quickmatch.arena_id,
            unk3: 0x0,
            password: request.password.clone(),
        })
    });

    push::send_push(request.host_player_id, push_payload).await?;
    Ok(ResponseParams::JoinQuickMatch)
}

pub async fn handle_accept_quick_match(
    session: ClientSession,
    request: RequestAcceptQuickMatchParams,
) -> rpc::HandlerResult {
    let (host_player_id, host_steam_id) = {
        let session = session.lock_read();
        (session.player_id, session.external_id.clone())
    };

    let quickmatch = pool::quickmatch_pool()?
        .by_topic_id(host_player_id)
        .ok_or(std::io::Error::from(std::io::ErrorKind::Other))?;

    let push_payload = PushParams::Join(JoinParams {
        identifier: shared::ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::AcceptQuickMatch(AcceptQuickMatchParams {
            quickmatch_settings: quickmatch.settings,
            host_player_id,
            host_steam_id,
            join_data: request.join_data,
        })
    });

    push::send_push(request.joining_player_id, push_payload).await?;
    Ok(ResponseParams::JoinQuickMatch)
}

impl QuickmatchPoolEntry {
    fn from_request(val: RequestRegisterQuickMatchParams, external_id: String) -> Self {
        QuickmatchPoolEntry {
            external_id,
            character_level: val.matching_parameters.soul_level as u32,
            weapon_level: val.matching_parameters.max_reinforce as u32,
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
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
            password: value.matching_parameters.password.clone(),
            arena_id: value.arena_id,
            settings: value.quickmatch_settings,
        }
    }
}
