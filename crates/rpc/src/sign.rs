use rand::prelude::*;
use thiserror::Error;

use waygate_connection::send_push;
use waygate_connection::ClientSession;
use waygate_pool::SignPoolEntry;
use waygate_pool::{PoolKey, SignPoolQuery, SIGN_POOL};

use crate::HandlerResult;

use waygate_message::*;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("Could not find specified sign")]
    SignMissing,
}

pub async fn handle_create_sign(
    session: ClientSession,
    request: RequestCreateSignParams,
) -> HandlerResult {
    let key = SIGN_POOL.insert(session.player_id, request.into())?;

    let identifier: ObjectIdentifier = (&key.1).into();
    session.game_session_mut().sign.push(key);

    Ok(ResponseParams::CreateSign(ResponseCreateSignParams {
        identifier,
    }))
}

pub async fn handle_get_sign_list(
    session: ClientSession,
    request: RequestGetSignListParams,
) -> HandlerResult {
    tracing::info!(
        "Host requested signs list. host = {}",
        session.player_id
    );

    let mut pool_matches = SIGN_POOL.match_entries::<SignPoolQuery>(&(&request).into());

    let already_received = pool_matches
        .iter()
        .filter_map(|e| {
            if request.known_signs.contains(&(&e.0).into()) {
                Some(e.0.clone())
            } else {
                None
            }
        })
        .collect::<Vec<PoolKey>>();

    pool_matches.retain(|e| !already_received.contains(&e.0));
    pool_matches.shuffle(&mut thread_rng());

    Ok(ResponseParams::GetSignList(ResponseGetSignListParams {
        known_signs: already_received.into_iter().map(|e| (&e).into()).collect(),

        entries: pool_matches.iter().map(|e| e.into()).collect(),
    }))
}

pub async fn handle_summon_sign(
    session: ClientSession,
    request: RequestSummonSignParams,
) -> HandlerResult {
    let player_id = session.player_id;
    tracing::info!("Host tapped a summon sign. host = {}", player_id);

    let poolkey: PoolKey = (&request.identifier).into();
    if !SIGN_POOL.has(&poolkey) {
        return Err(Box::new(SignError::SignMissing));
    }

    // Inform the summonee that they're being summoned
    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::SummonSign(SummonSignParams {
            summoning_player_id: player_id,
            steam_id: String::default(),
            summoned_player_id: poolkey.1,
            sign_identifier: (&poolkey).into(),
            join_data: request.data,
        }),
    });

    let result = send_push(poolkey.1, push_payload)
        .await
        .map(|_| ResponseParams::SummonSign)?;

    Ok(result)
}

pub async fn handle_remove_sign(
    session: ClientSession,
    request: RequestRemoveSignParams,
) -> HandlerResult {
    tracing::info!(
        "Player sent RemoveSign. player = {}",
        session.player_id
    );

    let mut game_session = session.game_session_mut();
    let index = game_session.sign.iter()
        .position(|s| s.1 .0 == request.sign_identifier.object_id)
        .ok_or(SignError::SignMissing)?;
    let _sign = game_session.sign.remove(index);

    Ok(ResponseParams::RemoveSign)
}

pub async fn handle_reject_sign(
    session: ClientSession,
    request: RequestRejectSignParams,
) -> HandlerResult {
    tracing::info!("Player sent RejectSign.");

    let summoned_player_id = session.player_id;

    // Inform the host that the summon is not coming
    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::RejectSign(RejectSignParams {
            sign_identifier: request.sign_identifier,
            summoned_player_id,
        }),
    });

    Ok(send_push(request.summoning_player_id, push_payload)
        .await
        .map(|_| ResponseParams::RejectSign)?)
}

pub async fn handle_update_sign(
    session: ClientSession,
    request: RequestUpdateSignParams,
) -> HandlerResult {
    tracing::info!(
        "Player sent UpdateSign. player = {}",
        session.player_id
    );

    let exists = SIGN_POOL.has(&(&request.identifier).into());

    if !exists {
        Err(Box::new(SignError::SignMissing))
    } else {
        Ok(ResponseParams::UpdateSign)
    }
}

pub async fn handle_create_match_area_sign(
    session: ClientSession,
    request: RequestCreateMatchAreaSignParams,
) -> HandlerResult {
    tracing::info!("CreateMatchAreaSign: {request:?}");

    let mut pool_entry: SignPoolEntry = request.into();
    pool_entry.external_id = session.external_id.clone();
    let key = SIGN_POOL.insert(session.player_id, pool_entry)?;

    let identifier: ObjectIdentifier = (&key.1).into();
    session.game_session_mut().sign.push(key);

    Ok(ResponseParams::CreateMatchAreaSign(
        ResponseCreateMatchAreaSignParams { identifier },
    ))
}

pub async fn handle_get_match_area_sign_list(
    request: RequestGetMatchAreaSignListParams,
) -> HandlerResult {
    tracing::info!("GetMatchAreaSignList: {request:?}");

    let mut pool_matches = SIGN_POOL.match_entries::<SignPoolQuery>(&(&request).into());

    tracing::info!("Found {} matches", pool_matches.len());

    let already_received = pool_matches
        .iter()
        .filter_map(|e| {
            if request.known_signs.contains(&(&e.0).into()) {
                Some(e.0.clone())
            } else {
                None
            }
        })
        .collect::<Vec<PoolKey>>();

    pool_matches.retain(|e| !already_received.contains(&e.0));

    Ok(ResponseParams::GetMatchAreaSignList(
        ResponseGetMatchAreaSignListParams {
            known_signs: already_received.into_iter().map(|e| (&e).into()).collect(),
            entries: pool_matches.iter().map(|e| e.into()).collect(),
        },
    ))
}
