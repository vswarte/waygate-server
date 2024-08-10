use rand::Rng;
use waygate_connection::send_push;
use waygate_connection::ClientSession;
use waygate_connection::ClientSessionContainer;
use waygate_message::*;
use waygate_pool::{QUICKMATCH_POOL, QuickmatchPoolQuery, QuickmatchPoolEntry, PoolError};

use crate::HandlerResult;

pub async fn handle_search_quick_match(
    request: RequestSearchQuickMatchParams,
) -> HandlerResult {
    Ok(ResponseParams::SearchQuickMatch(ResponseSearchQuickMatchParams {
        matches: QUICKMATCH_POOL
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
) -> HandlerResult {
    tracing::info!("RequestRegisterQuickMatchParams: {:#?}", request);

    let mut session = session.lock_write();

    let entry: QuickmatchPoolEntry = QuickmatchPoolEntry::from_request(
        request,
        session.external_id.clone(),
    );

    tracing::info!("QuickmatchPoolEntry: {:#?}", entry);
    let key = QUICKMATCH_POOL.insert(session.player_id, entry)?;

    session.quickmatch = Some(key);
    Ok(ResponseParams::RegisterQuickMatch)
}

pub async fn handle_unregister_quick_match(
    session: ClientSession,
) -> HandlerResult {
    let _ = session.lock_write().quickmatch.take();

    Ok(ResponseParams::UnregisterQuickMatch)
}

pub async fn handle_update_quick_match(
    session: ClientSession,
) -> HandlerResult {
    // Current we just check if the quickmatch is known in-memory at all.
    let _existing = session.lock_write().quickmatch.as_ref()
        .ok_or(PoolError::NotFound)?;

    Ok(ResponseParams::UpdateQuickMatch)
}

pub async fn handle_join_quick_match(
    session: ClientSession,
    request: RequestJoinQuickMatchParams,
) -> HandlerResult {
    tracing::info!("RequestJoinQuickMatchParams: {:#?}", request);

    let quickmatch = QUICKMATCH_POOL
        .by_topic_id(request.host_player_id)
        .ok_or(std::io::Error::from(std::io::ErrorKind::Other))?;

    let (joining_player_id, joining_player_steam_id) = {
        let session = session.lock_read();

        (session.player_id, session.external_id.clone())
    };

    tracing::debug!("Joining player ID: {joining_player_id:?}");

    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::JoinQuickMatch(JoinQuickMatchParams {
            quickmatch_settings: quickmatch.settings,
            joining_player_id,
            joining_player_steam_id,
            unk2: 0x3,
            arena_id: quickmatch.arena_id,
            unk3: 0x0,
            password: request.password.clone(),
        })
    });

    send_push(request.host_player_id, push_payload).await?;
    Ok(ResponseParams::JoinQuickMatch)
}

pub async fn handle_accept_quick_match(
    session: ClientSession,
    request: RequestAcceptQuickMatchParams,
) -> HandlerResult {
    tracing::debug!("RequestAcceptQuickMatchParams: {:#?}", request);

    let (host_player_id, host_steam_id) = {
        let session = session.lock_read();
        (session.player_id, session.external_id.clone())
    };

    let quickmatch = QUICKMATCH_POOL
        .by_topic_id(host_player_id)
        .ok_or(std::io::Error::from(std::io::ErrorKind::Other))?;

    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
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

    send_push(request.joining_player_id, push_payload).await?;
    Ok(ResponseParams::JoinQuickMatch)
}

pub async fn handle_quick_match_result_log(
    _session: ClientSession,
) -> HandlerResult {
    tracing::debug!("Got quickmatch result log");

    Ok(ResponseParams::QuickMatchResultLog)
}

pub async fn handle_send_quick_match_start(
    _session: ClientSession,
) -> HandlerResult {
    tracing::debug!("Got quickmatch start log");

    Ok(ResponseParams::SendQuickMatchStart)
}

pub async fn handle_send_quick_match_result(
    _session: ClientSession,
) -> HandlerResult {
    tracing::debug!("Got quickmatch result log");

    Ok(ResponseParams::SendQuickMatchResult)
}

