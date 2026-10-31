use message::{
    builder::MessageBuilder,
    eldenring::{
        AcceptQuickMatchParams, JoinParams, JoinPayload, JoinQuickMatchParams, ObjectIdentifier,
        PushParams, RejectQuickMatchParams, RequestAcceptQuickMatchParams,
        RequestJoinQuickMatchParams, RequestRegisterQuickMatchParams,
        RequestRejectQuickMatchParams, RequestSearchQuickMatchParams,
        RequestSendQuickMatchResultParams, RequestSendQuickMatchStartParams,
        RequestUnregisterQuickMatchParams, RequestUpdateQuickMatchParams,
        ResponseAcceptQuickMatchParams, ResponseJoinQuickMatchParams,
        ResponseRegisterQuickMatchParams, ResponseRejectQuickMatchParams,
        ResponseSearchQuickMatchParams, ResponseSearchQuickMatchParamsEntry,
        ResponseSendQuickMatchResultParams, ResponseSendQuickMatchStartParams,
        ResponseUnregisterQuickMatchParams, ResponseUpdateQuickMatchParams, ServerError,
    },
};
use rand::prelude::*;

use crate::{
    handler::HandleRequest,
    services::eldenring::quickmatch::{
        QuickMatchJoinAttemptKey, QuickMatchPoolEntry, QuickMatchPoolQuery,
    },
};

use super::DefaultClientHandler;

const QUICKMATCH_SEARCH_DEFAULT_LIMIT: usize = 32;
const QUICKMATCH_SEARCH_MAX_LIMIT: usize = 64;

impl HandleRequest<Box<RequestSearchQuickMatchParams>, ResponseSearchQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestSearchQuickMatchParams>,
    ) -> Result<ResponseSearchQuickMatchParams, Box<dyn std::error::Error>> {
        let limit = match request.search_limit {
            0 => QUICKMATCH_SEARCH_DEFAULT_LIMIT,
            limit => (limit as usize).min(QUICKMATCH_SEARCH_MAX_LIMIT),
        };
        let query = QuickMatchPoolQuery {
            player_id: self.session.player_id,
            arenas: request.arenas.clone(),
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            password: request.matching_parameters.password.0.clone(),
            quickmatch_settings: request.quickmatch_settings,
        };

        let result = self.services.quickmatch.search(&query, limit).await;

        Ok(ResponseSearchQuickMatchParams {
            matches: result
                .matches
                .into_iter()
                .map(|e| ResponseSearchQuickMatchParamsEntry {
                    host_player_id: e.host_player_id,
                    host_external_id: e.host_external_id,
                    arena_id: e.arena_id,
                })
                .collect(),
            target_team: result.target_team,
            is_allied_password_opponent_search: result.is_allied_password_opponent_search,
        })
    }
}

impl HandleRequest<Box<RequestRegisterQuickMatchParams>, ResponseRegisterQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestRegisterQuickMatchParams>,
    ) -> Result<ResponseRegisterQuickMatchParams, Box<dyn std::error::Error>> {
        let entry = QuickMatchPoolEntry {
            host_player_id: self.session.player_id,
            host_external_id: self.session.external_id.clone(),
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            arena_id: request.arena_id,
            password: request.matching_parameters.password.clone().into(),
            quickmatch_settings: request.quickmatch_settings,
            host_tx: self.push_tx.clone(),
            updated_at: tokio::time::Instant::now(),
        };

        let token = self
            .services
            .quickmatch
            .register(entry, request.is_allied_password);
        if request.is_allied_password {
            self.quickmatch_allied_pw_token = Some(token);
        } else {
            self.quickmatch_token = Some(token);
        }

        Ok(ResponseRegisterQuickMatchParams {})
    }
}

impl HandleRequest<Box<RequestUnregisterQuickMatchParams>, ResponseUnregisterQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestUnregisterQuickMatchParams>,
    ) -> Result<ResponseUnregisterQuickMatchParams, Box<dyn std::error::Error>> {
        match request.is_allied_password {
            true => self.quickmatch_allied_pw_token = None,
            false => self.quickmatch_token = None,
        }

        Ok(ResponseUnregisterQuickMatchParams {})
    }
}

impl HandleRequest<Box<RequestUpdateQuickMatchParams>, ResponseUpdateQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestUpdateQuickMatchParams>,
    ) -> Result<ResponseUpdateQuickMatchParams, Box<dyn std::error::Error>> {
        if self.quickmatch_token.is_none() && self.quickmatch_allied_pw_token.is_none() {
            return Err(Box::new(ServerError::NotFound));
        }

        let merger = |e: &mut QuickMatchPoolEntry| {
            e.quickmatch_settings = request.quickmatch_settings;
            e.arena_id = request.arena_id;
        };

        if let Some(token) = &self.quickmatch_token {
            self.services.quickmatch.merge(token.key(), false, merger)?;
        }
        if let Some(token) = &self.quickmatch_allied_pw_token {
            self.services.quickmatch.merge(token.key(), true, merger)?;
        }

        Ok(ResponseUpdateQuickMatchParams {})
    }
}

impl HandleRequest<Box<RequestJoinQuickMatchParams>, ResponseJoinQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestJoinQuickMatchParams>,
    ) -> Result<ResponseJoinQuickMatchParams, Box<dyn std::error::Error>> {
        let entry = self
            .services
            .quickmatch
            .lobby_for_join(request.host_player_id, request.team)
            .ok_or(ServerError::NotFound)?;

        entry.host_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::JoinQuickMatch(JoinQuickMatchParams {
                        quickmatch_settings: entry.quickmatch_settings,
                        joining_player_id: self.session.player_id,
                        joining_player_external_id: self.session.external_id.clone(),
                        unk2: 0,
                        arena_id: request.arena_id,
                        team: request.team,
                        password: request.password.clone().into(),
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseJoinQuickMatchParams {})
    }
}

impl HandleRequest<Box<RequestAcceptQuickMatchParams>, ResponseAcceptQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestAcceptQuickMatchParams>,
    ) -> Result<ResponseAcceptQuickMatchParams, Box<dyn std::error::Error>> {
        let attempt = self
            .services
            .quickmatch
            .take_join_attempt(&QuickMatchJoinAttemptKey {
                host_player_id: self.session.player_id,
                joining_player_id: request.joining_player_id,
            })
            .ok_or(ServerError::NotFound)?;

        attempt.joining_player_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::AcceptQuickMatch(AcceptQuickMatchParams {
                        quickmatch_settings: attempt.quickmatch_settings,
                        host_player_id: self.session.player_id,
                        host_external_id: self.session.external_id.clone(),
                        join_data: request.join_data.clone(),
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseAcceptQuickMatchParams {})
    }
}

impl HandleRequest<Box<RequestRejectQuickMatchParams>, ResponseRejectQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestRejectQuickMatchParams>,
    ) -> Result<ResponseRejectQuickMatchParams, Box<dyn std::error::Error>> {
        let attempt = self
            .services
            .quickmatch
            .take_join_attempt(&QuickMatchJoinAttemptKey {
                host_player_id: self.session.player_id,
                joining_player_id: request.joiner_player_id as i32,
            })
            .ok_or(ServerError::NotFound)?;

        let _ = attempt.joining_player_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::RejectQuickMatch(RejectQuickMatchParams {
                        reject_reason: request.reject_reason,
                        host_player_id: self.session.player_id,
                        quickmatch_settings: request.current_settings,
                    }),
                }))
                .build()?,
        );

        Ok(ResponseRejectQuickMatchParams {})
    }
}

impl HandleRequest<Box<RequestSendQuickMatchStartParams>, ResponseSendQuickMatchStartParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestSendQuickMatchStartParams>,
    ) -> Result<ResponseSendQuickMatchStartParams, Box<dyn std::error::Error>> {
        Ok(ResponseSendQuickMatchStartParams {})
    }
}

impl HandleRequest<Box<RequestSendQuickMatchResultParams>, ResponseSendQuickMatchResultParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestSendQuickMatchResultParams>,
    ) -> Result<ResponseSendQuickMatchResultParams, Box<dyn std::error::Error>> {
        Ok(ResponseSendQuickMatchResultParams {
            united_combat_rank: 0,
            united_combat_points: 0,
            duel_rank: 0,
            duel_points: 0,
            spirit_ashes_rank: 0,
            spirit_ashes_points: 0,
        })
    }
}
