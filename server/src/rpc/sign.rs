use rand::prelude::*;
use thiserror::Error;

use crate::pool::MatchResult;
use crate::pool::sign::SignPoolEntry;
use crate::pool::sign::SignPoolQuery;
use crate::pool::sign_pool;
use crate::pool::PoolKey;
use crate::push;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

use super::message::*;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("Could not find specified sign")]
    SignMissing,
}

pub async fn handle_create_sign(
    session: ClientSession,
    request: RequestCreateSignParams,
) -> rpc::HandlerResult {
    let mut session = session.lock_write();

    let key = crate::pool::sign_pool()
        .insert(session.player_id, request.into())?;

    let identifier: ObjectIdentifier = (&key.1).into();
    session.sign.push(key);

    Ok(ResponseParams::CreateSign(ResponseCreateSignParams {
        identifier,
    }))
}

pub async fn handle_get_sign_list(
    session: ClientSession,
    request: RequestGetSignListParams,
) -> rpc::HandlerResult {
    log::info!("Host requested signs list. host = {}", session.lock_read().player_id);

    let mut pool_matches = sign_pool()
        .match_entries::<SignPoolQuery>(&(&request).into());

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

    Ok(ResponseParams::GetSignList(ResponseGetSignListParams {
        known_signs: already_received.into_iter()
            .map(|e| (&e).into())
            .collect(),

        entries: pool_matches.iter()
            .map(|e| e.into())
            .collect(),
    }))
}


// Host tapped the sign
pub async fn handle_summon_sign(
    session: ClientSession,
    request: RequestSummonSignParams,
) -> rpc::HandlerResult {
    let player_id = session.lock_read().player_id;
    log::info!("Host tapped a summon sign. host = {}", player_id);

    let poolkey: PoolKey = (&request.identifier).into();
    if !sign_pool().has(&poolkey) {
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

    let result = push::send_push(poolkey.1, push_payload)
        .await
        .map(|_| ResponseParams::SummonSign)?;

    Ok(result)
}

pub async fn handle_remove_sign(
    session: ClientSession,
    request: RequestRemoveSignParams,
) -> rpc::HandlerResult {
    log::info!("Player sent RemoveSign. player = {}", session.lock_read().player_id);

    let mut session = session.lock_write();
    let index = session.sign.iter()
        .position(|s| s.1.0 == request.sign_identifier.object_id)
        .ok_or(SignError::SignMissing)?;
    let _sign = session.sign.remove(index);

    Ok(ResponseParams::RemoveSign)
}

pub async fn handle_reject_sign(
    session: ClientSession,
    request: RequestRejectSignParams,
) -> rpc::HandlerResult {
    log::info!("Player sent RejectSign.");

    let summoned_player_id = session.lock_read().player_id;

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

    Ok(
        push::send_push(request.summoning_player_id, push_payload)
        .await
        .map(|_| ResponseParams::RejectSign)?
    )
}

pub async fn handle_update_sign(
    session: ClientSession,
    request: RequestUpdateSignParams,
) -> rpc::HandlerResult {
    log::info!("Player sent UpdateSign. player = {}", session.lock_read().player_id);

    let exists = crate::pool::sign_pool()
        .has(&(&request.identifier).into());

    if !exists {
        Err(Box::new(SignError::SignMissing))
    } else {
        Ok(ResponseParams::UpdateSign)
    }
}

impl From<RequestCreateSignParams> for SignPoolEntry {
    fn from(val: RequestCreateSignParams) -> Self {
        SignPoolEntry {
            external_id: String::new(),
            character_level: val.matching_parameters.soul_level as u32,
            weapon_level: val.matching_parameters.max_reinforce as u32,
            area: (&val.area).into(),
            password: val.matching_parameters.password.clone(),
            group_passwords: val.group_passwords.clone(),
            data: val.data,
        }
    }
}

impl From<RequestCreateMatchAreaSignParams> for SignPoolEntry {
    fn from(val: RequestCreateMatchAreaSignParams) -> Self {
        SignPoolEntry {
            external_id: String::new(),
            character_level: val.matching_parameters.soul_level as u32,
            weapon_level: val.matching_parameters.max_reinforce as u32,
            area: (&val.area).into(),
            password: val.matching_parameters.password.clone(),
            group_passwords: val.group_passwords.clone(),
            data: val.data,
        }
    }
}

impl From<&MatchResult<SignPoolEntry>> for ResponseGetSignListParamsEntry {
    fn from(val: &MatchResult<SignPoolEntry>) -> Self {
        ResponseGetSignListParamsEntry {
            player_id: val.0.1,
            identifier: (&val.0).into(),
            area: (&val.1.area).into(),
            data: val.1.data.clone(),
            steam_id: val.1.external_id.clone(),
            unk_string: String::default(),
            group_passwords: val.1.group_passwords.clone(),
        }
    }
}

impl From<&RequestGetSignListParams> for SignPoolQuery {
    fn from(value: &RequestGetSignListParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
            areas: value.search_areas.iter().map(|e| e.into()).collect(),
            password: value.matching_parameters.password.clone(),
        }
    }
}

impl From<&RequestGetMatchAreaSignListParams> for SignPoolQuery {
    fn from(value: &RequestGetMatchAreaSignListParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
            areas: vec![(&value.area).into()],
            password: value.matching_parameters.password.clone(),
        }
    }
}

impl From<&MatchResult<SignPoolEntry>> for ResponseGetMatchAreaSignListParamsEntry {
    fn from(val: &MatchResult<SignPoolEntry>) -> Self {
        ResponseGetMatchAreaSignListParamsEntry {
            player_id: val.0.1,
            identifier: (&val.0).into(),
            data: val.1.data.clone(),
            steam_id: val.1.external_id.clone(),
            area: (&val.1.area).into(),
            unk1: 0,
            unk_string: String::default(),
            group_passwords: val.1.group_passwords.clone(),
        }
    }
}

pub async fn handle_create_match_area_sign(
    session: ClientSession,
    request: RequestCreateMatchAreaSignParams,
) -> rpc::HandlerResult {
    log::info!("CreateMatchAreaSign: {request:?}");

    let mut session = session.lock_write();

    let key = crate::pool::sign_pool()
        .insert(session.player_id, request.into())?;

    let identifier: ObjectIdentifier = (&key.1).into();
    session.sign.push(key);

    Ok(ResponseParams::CreateMatchAreaSign(ResponseCreateMatchAreaSignParams {
        identifier,
    }))
}

pub async fn handle_get_match_area_sign_list(
    request: RequestGetMatchAreaSignListParams,
) -> rpc::HandlerResult {
    log::info!("GetMatchAreaSignList: {request:?}");

    let mut pool_matches = sign_pool()
        .match_entries::<SignPoolQuery>(&(&request).into());

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

    Ok(ResponseParams::GetMatchAreaSignList(ResponseGetMatchAreaSignListParams {
        known_signs: already_received.into_iter()
            .map(|e| (&e).into())
            .collect(),

        entries: pool_matches.iter()
            .map(|e| e.into())
            .collect(),
    }))
}
