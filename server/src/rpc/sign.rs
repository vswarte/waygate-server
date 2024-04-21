use fnrpc::ResponseParams;
use fnrpc::push::JoinPayload;
use fnrpc::push::PushJoinParams;
use fnrpc::push::PushParams;
use fnrpc::shared::ObjectIdentifier;
use fnrpc::sign::*;
use rand::Rng;
use thiserror::Error;

use crate::pool::key::PoolKey;
use crate::pool::matching::area::MatchingArea;
use crate::pool::pool::MatchResult;
use crate::pool::sign::SignHostMatcher;
use crate::pool::sign::SignPoolEntry;
use crate::pool::sign::SignPoolQuery;
use crate::pool::sign_pool;
use crate::rpc;
use crate::push;
use crate::session::ClientSession;

pub async fn handle_create_sign(
    session: ClientSession,
    request: RequestCreateSignParams,
) -> rpc::HandlerResult {
    let key = crate::pool::sign_pool_mut()
        .insert(session.player_id, (&request).into())?;

    log::info!(
        "Player sent CreateSign. player = {}. area = {}. play_region = {}",
        session.player_id,
        request.area.area,
        request.area.play_region,
    );

    Ok(ResponseParams::CreateSign(
        ResponseCreateSignParams {
            identifier: (&key).into()
        }
    ))
}

pub async fn handle_get_sign_list(
    session: ClientSession,
    request: RequestGetSignListParams,
) -> rpc::HandlerResult {
    log::info!("Player sent GetSignList. player = {}", session.player_id);

    let query = (&request).into();
    let mut pool_matches = sign_pool()
        .match_entries::<SignPoolQuery, SignHostMatcher>(&query);

    let already_received = pool_matches.iter()
        .filter_map(|e| if request.known_signs.contains(&(&e.0).into()) {
            Some(e.0.clone())
        } else {
            None
        })
        .collect::<Vec<PoolKey>>();

    pool_matches.retain(|e| !already_received.contains(&e.0));

    Ok(ResponseParams::GetSignList(
        ResponseGetSignListParams {
            known_signs: already_received.into_iter()
                .map(|e| (&e).into())
                .collect(),

            entries: pool_matches.iter()
                .map(|e| e.into())
                .collect()
        }
    ))
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
    log::info!("Player sent SummonSign. player = {}", session.player_id);

    let poolkey: PoolKey = (&request.identifier).into();
    if !sign_pool().has(&poolkey) {
        return Err(Box::new(SummonError::SignMissing));
    }

    let push_payload = PushParams::Join(PushJoinParams {
        identifier: ObjectIdentifier {
            object_id: rand::thread_rng().gen::<i32>(),
            secondary_id: rand::thread_rng().gen::<i32>(),
        },
        join_payload: JoinPayload::JoiningPlayer(fnrpc::push::PushJoiningPlayerParams {
            summoning_player_id: session.player_id,
            steam_id: String::default(),
            summoned_player_id: poolkey.1 as i32,
            sign_identifier: (&poolkey).into(),
            join_data: request.data,
        })
    });

    Ok(
        push::send_push(poolkey.1 as i32, push_payload)
            .await
            .map(|_| ResponseParams::SummonSign)?
    )
}

pub async fn handle_remove_sign(
    session: ClientSession,
    request: RequestRemoveSignParams,
) -> rpc::HandlerResult {
    log::info!("Player sent RemoveSign. player = {}", session.player_id);

    crate::pool::sign_pool_mut()
        .remove(&(&request.sign_identifier).into())?;

    Ok(ResponseParams::RemoveSign)
}

pub async fn handle_reject_sign(
    session: ClientSession,
) -> rpc::HandlerResult {
    log::info!("Player sent RejectSign. player = {}", session.player_id);

    // TODO: notify summoner of rejected summon
    Ok(ResponseParams::RejectSign)
}

pub async fn handle_update_sign(
    session: ClientSession,
    request: RequestUpdateSignParams,
) -> rpc::HandlerResult {
    log::info!("Player sent UpdateSign. player = {}", session.player_id);

    let exists = crate::pool::sign_pool_mut()
        .has(&(&request.identifier).into());

    if !exists {
        Err(Box::new(SummonError::SignMissing))
    } else {
        Ok(ResponseParams::UpdateSign)
    }
}

impl From<&RequestCreateSignParams> for SignPoolEntry {
    fn from(value: &RequestCreateSignParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level,
            weapon_level: value.matching_parameters.max_reinforce,
            area: MatchingArea::new(
                value.area.area as u32,
                value.area.play_region as u32,
            ),
            password: value.matching_parameters.password.clone(),
            group_passwords: value.group_passwords.clone(),
            data: value.data.clone(),
        }
    }
}

impl Into<ResponseGetSignListParamsEntry> for &MatchResult<SignPoolEntry> {
    fn into(self) -> ResponseGetSignListParamsEntry {
        ResponseGetSignListParamsEntry {
            player_id: self.0.1 as i32,
            identifier: (&self.0).into(),
            area: (&self.1.area).into(),
            data: self.1.data.clone(),
            steam_id: String::default(),
            unk_string: String::default(),
            group_passwords: self.1.group_passwords.clone(),
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

