use message::{
    builder::MessageBuilder,
    eldenring::{
        AcceptQuickMatchParams, JoinParams, JoinPayload, JoinQuickMatchParams, ObjectIdentifier,
        PushParams, RequestAcceptQuickMatchParams, RequestCreateBattleSessionParams,
        RequestJoinQuickMatchParams, RequestRegisterQuickMatchParams,
        RequestSearchQuickMatchParams, RequestUnregisterQuickMatchParams,
        RequestUpdateQuickMatchParams, ResponseAcceptQuickMatchParams,
        ResponseCreateBattleSessionParams, ResponseJoinQuickMatchParams,
        ResponseRegisterQuickMatchParams, ResponseSearchQuickMatchParams,
        ResponseSearchQuickMatchParamsEntry, ResponseUnregisterQuickMatchParams,
        ResponseUpdateQuickMatchParams,
    },
};
use rand::prelude::*;
use std::time::Duration;
use thiserror::Error;

use crate::{
    handler::HandleRequest,
    services::eldenring::quickmatch::{
        QuickMatchJoinAttempt, QuickMatchPoolEntry, QuickMatchPoolKey, QuickMatchPoolQuery,
        QUICKMATCH_JOIN_ATTEMPTS,
    },
};

use super::DefaultClientHandler;

#[derive(Debug, Error)]
enum Error {
    #[error("QuickMatch lobby could not be found.")]
    QuickMatchNotFound,
    #[error("QuickMatch join attempt could not be found.")]
    QuickMatchJoinAttemptNotFound,
}

const ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);

impl HandleRequest<Box<RequestSearchQuickMatchParams>, ResponseSearchQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestSearchQuickMatchParams>,
    ) -> Result<ResponseSearchQuickMatchParams, Box<dyn std::error::Error>> {
        let mut pool_matches = self.services.pool_quickmatch.matches(&QuickMatchPoolQuery {
            player_id: self.session.player_id,
            arena_id: request.arena_id,
            character_level: request.matching_parameters.character_level as u32,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            password: request.matching_parameters.password.clone(),
            quickmatch_settings: request.quickmatch_settings,
        });

        pool_matches.shuffle(&mut rand::rng());

        Ok(ResponseSearchQuickMatchParams {
            matches: pool_matches
                .into_iter()
                .map(|e| ResponseSearchQuickMatchParamsEntry {
                    host_player_id: e.1.host_player_id,
                    host_external_id: e.1.host_external_id,
                    arena_id: e.1.arena_id,
                })
                .collect(),
            unk1: 0x0,
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
        log::info!("Register {request:#?}");
        let token = self.services.pool_quickmatch.insert(
            self.session.player_id,
            QuickMatchPoolEntry {
                host_player_id: self.session.player_id,
                host_external_id: self.session.external_id.clone(),
                character_level: request.matching_parameters.character_level as u32,
                weapon_level: request.matching_parameters.max_reinforce as u32,
                arena_id: request.arena_id,
                password: request.matching_parameters.password.clone(),
                quickmatch_settings: request.quickmatch_settings,
                host_tx: self.push_tx.clone(),
            },
        );

        self.quickmatch_token = Some(token);

        Ok(ResponseRegisterQuickMatchParams {})
    }
}

impl HandleRequest<Box<RequestUnregisterQuickMatchParams>, ResponseUnregisterQuickMatchParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestUnregisterQuickMatchParams>,
    ) -> Result<ResponseUnregisterQuickMatchParams, Box<dyn std::error::Error>> {
        let _ = self.quickmatch_token.take();

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
        let Some(token) = self.quickmatch_token.as_ref() else {
            return Err(Box::new(Error::QuickMatchNotFound));
        };

        self.services.pool_quickmatch.merge(&token.1, |e| {
            e.quickmatch_settings = request.quickmatch_settings;
            e.arena_id = request.arena_id;
        })?;

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
        // Retrieve pool entry for quickmatch lobby if it's still there.
        let pool_key = QuickMatchPoolKey(request.host_player_id);
        let Some(entry) = self.services.pool_quickmatch.get(&pool_key) else {
            return Err(Box::new(Error::QuickMatchNotFound));
        };

        // Save our connection attempt for cancellation if required.
        let joining_player_id = self.session.player_id;
        QUICKMATCH_JOIN_ATTEMPTS.insert(
            (pool_key.clone(), joining_player_id),
            QuickMatchJoinAttempt {
                quickmatch_settings: entry.quickmatch_settings,
                joining_player_tx: self.push_tx.clone(),
            },
        );

        {
            let pool_key = pool_key.clone();
            tokio::spawn(async move {
                tokio::time::sleep(ATTEMPT_CLEANUP_TIMEOUT).await;
                let _ = QUICKMATCH_JOIN_ATTEMPTS.remove(&(pool_key, joining_player_id));
            });
        }

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
                        unk3: 0,
                        password: request.password.clone(),
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
        let pool_key = QuickMatchPoolKey(self.session.player_id);

        let attempt = QUICKMATCH_JOIN_ATTEMPTS
            .remove(&(pool_key, request.joining_player_id))
            .ok_or(Error::QuickMatchJoinAttemptNotFound)?;

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

impl HandleRequest<Box<RequestCreateBattleSessionParams>, ResponseCreateBattleSessionParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestCreateBattleSessionParams>,
    ) -> Result<ResponseCreateBattleSessionParams, Box<dyn std::error::Error>> {
        Ok(ResponseCreateBattleSessionParams {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 0,
            unk5: 0,
            unk6: 0,
        })
    }
}
