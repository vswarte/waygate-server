use fnrpc::push::JoinParams;
use rand::prelude::*;
use fnrpc::push::JoinPayload;
use fnrpc::push::PushParams;
use fnrpc::shared::ObjectIdentifier;
use fnrpc::sign::*;
use fnrpc::ResponseParams;
use thiserror::Error;

use crate::pool::key::PoolKey;
use crate::pool::matching::area::MatchingArea;
use crate::pool::MatchResult;
use crate::pool::sign::SignPoolEntry;
use crate::pool::sign::SignPoolQuery;
use crate::pool::sign_pool;
use crate::push;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

pub async fn handle_create_sign(
    session: ClientSession,
    request: RequestCreateSignParams,
) -> rpc::HandlerResult {
    let session = session.lock_read();

    log::info!(
        "Player put down their sign. player = {}. area = {}. play_region = {}",
        session.player_id,
        request.area.area,
        request.area.play_region,
    );

    let key = crate::pool::sign_pool()?
        .insert(session.player_id, request.into())?;

    Ok(ResponseParams::CreateSign(ResponseCreateSignParams {
        identifier: (&key).into(),
    }))
}

pub async fn handle_get_sign_list(
    session: ClientSession,
    request: RequestGetSignListParams,
) -> rpc::HandlerResult {
    log::info!("Host requested signs list. host = {}", session.lock_read().player_id);

    let mut pool_matches = sign_pool()?
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

#[derive(Debug, Error)]
pub enum SummonError {
    #[error("Could not find sign")]
    SignMissing,
}

pub async fn handle_summon_sign(
    session: ClientSession,
    request: RequestSummonSignParams,
) -> rpc::HandlerResult {
    let player_id = session.lock_read().player_id;
    log::info!("Host tapped a summon sign. host = {}", player_id);

    let poolkey: PoolKey = (&request.identifier).into();
    if !sign_pool()?.has(&poolkey) {
        return Err(Box::new(SummonError::SignMissing));
    }

    // Inform the summonee that they're being summoned
    let push_payload = PushParams::Join(JoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::SummonSign(fnrpc::push::SummonSignParams {
            summoning_player_id: player_id,
            steam_id: String::default(),
            summoned_player_id: poolkey.1,
            sign_identifier: (&poolkey).into(),
            join_data: request.data,
        }),
    });

    Ok(
        push::send_push(poolkey.1, push_payload)
            .await
            .map(|_| ResponseParams::SummonSign)?
    )
}

pub async fn handle_remove_sign(
    session: ClientSession,
    request: RequestRemoveSignParams,
) -> rpc::HandlerResult {
    log::info!("Player sent RemoveSign. player = {}", session.lock_read().player_id);

    crate::pool::sign_pool()?
        .remove(&(&request.sign_identifier).into())?;

    Ok(ResponseParams::RemoveSign)
}

pub async fn handle_reject_sign(
    session: ClientSession,
    request: RequestRejectSignParams,
) -> rpc::HandlerResult {
    log::info!(
        "Player sent RejectSign. player = {}. request = {:#?}",
        session.lock_read().player_id,
        request
    );

    // TODO: notify summoner of rejected summon
    Ok(ResponseParams::RejectSign)
}

pub async fn handle_update_sign(
    session: ClientSession,
    request: RequestUpdateSignParams,
) -> rpc::HandlerResult {
    log::info!("Player sent UpdateSign. player = {}", session.lock_read().player_id);

    let exists = crate::pool::sign_pool()?
        .has(&(&request.identifier).into());

    if !exists {
        Err(Box::new(SummonError::SignMissing))
    } else {
        Ok(ResponseParams::UpdateSign)
    }
}

impl From<RequestCreateSignParams> for SignPoolEntry {
    fn from(val: RequestCreateSignParams) -> Self {
        SignPoolEntry {
            external_id: String::new(),
            character_level: val.matching_parameters.soul_level,
            weapon_level: val.matching_parameters.max_reinforce,
            area: MatchingArea::new(val.area.area as u32, val.area.play_region as u32),
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
            steam_id: String::default(),
            unk_string: String::default(),
            group_passwords: val.1.group_passwords.clone(),
        }
    }
}

impl From<&RequestGetSignListParams> for SignPoolQuery {
    fn from(value: &RequestGetSignListParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level,
            weapon_level: value.matching_parameters.max_reinforce,
            areas: value.search_areas.iter().map(|e| e.into()).collect(),
            password: value.matching_parameters.password.clone(),
        }
    }
}
