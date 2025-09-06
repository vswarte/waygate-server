use message::{
    builder::MessageBuilder,
    eldenring::{
        AllowBreakInTargetParams, BreakInTargetParams, JoinParams, JoinPayload, ObjectIdentifier,
        PushParams, RejectBreakInTargetParams, RequestAllowBreakInTargetParams,
        RequestBreakInTargetParams, RequestGetBreakInTargetListParams,
        RequestRejectBreakInTargetParams, ResponseAllowBreakInTargetParams,
        ResponseBreakInTargetParams, ResponseGetBreakInTargetListParams,
        ResponseGetBreakInTargetListParamsEntry, ResponseRejectBreakInTargetParams,
    },
};
use rand::prelude::*;
use thiserror::Error;

use crate::{
    handler::HandleRequest,
    services::eldenring::breakin::{
        BreakInAttempt, BreakInPoolKey, BreakInPoolQuery, BREAKIN_ATTEMPTS,
        BREAKIN_ATTEMPT_CLEANUP_TIMEOUT,
    },
};

use super::DefaultClientHandler;

#[derive(Debug, Error)]
enum Error {
    #[error("BreakIn target could not be found.")]
    BreakInTargetNotFound,
    #[error("BreakIn attempt could not be found.")]
    BreakInAttemptNotFound,
}

impl HandleRequest<Box<RequestGetBreakInTargetListParams>, ResponseGetBreakInTargetListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetBreakInTargetListParams>,
    ) -> Result<ResponseGetBreakInTargetListParams, Box<dyn std::error::Error>> {
        let mut pool_matches = self.services.pool_breakin.matches(&BreakInPoolQuery {
            player_id: self.session.player_id,
            play_region: request.play_region,
            character_level: request.matching_parameters.character_level as u32,
            weapon_level: request.matching_parameters.max_reinforce as u32,
        });

        pool_matches.shuffle(&mut rand::rng());
        let limited = pool_matches.iter().take(request.max_count as usize);

        Ok(ResponseGetBreakInTargetListParams {
            play_region: request.play_region,
            entries: limited
                .map(|e| ResponseGetBreakInTargetListParamsEntry {
                    player_id: e.1.player_id,
                    external_id: e.1.external_id.clone(),
                })
                .collect(),
        })
    }
}

impl HandleRequest<Box<RequestBreakInTargetParams>, ResponseBreakInTargetParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestBreakInTargetParams>,
    ) -> Result<ResponseBreakInTargetParams, Box<dyn std::error::Error>> {
        // Retrieve pool entry for tapped sign if it's still there.
        let pool_key = BreakInPoolKey(request.player_id);
        let Some(entry) = self.services.pool_breakin.get(&pool_key) else {
            return Err(Box::new(Error::BreakInTargetNotFound));
        };

        // Save our connection attempt for cancellation if required.
        let invader_id = self.session.player_id;
        BREAKIN_ATTEMPTS.insert(
            (pool_key.clone(), invader_id),
            BreakInAttempt {
                invader_tx: self.push_tx.clone(),
            },
        );

        // Remove attempt entry after a while so it doesn't linger if the invasion request
        // got no response at all.
        {
            let pool_key = pool_key.clone();
            tokio::spawn(async move {
                tokio::time::sleep(BREAKIN_ATTEMPT_CLEANUP_TIMEOUT).await;
                let _ = BREAKIN_ATTEMPTS.remove(&(pool_key, invader_id));
            });
        }

        entry.target_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::BreakInTarget(BreakInTargetParams {
                        invader_player_id: self.session.player_id,
                        invader_external_id: self.session.external_id.clone(),
                        unk1: 0,
                        unk2: 0,
                        play_region: request.play_region,
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseBreakInTargetParams {})
    }
}

impl HandleRequest<Box<RequestAllowBreakInTargetParams>, ResponseAllowBreakInTargetParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestAllowBreakInTargetParams>,
    ) -> Result<ResponseAllowBreakInTargetParams, Box<dyn std::error::Error>> {
        let pool_key = BreakInPoolKey(self.session.player_id);

        let attempt = BREAKIN_ATTEMPTS
            .remove(&(pool_key, request.invading_player_id))
            .ok_or(Error::BreakInAttemptNotFound)?;

        attempt.invader_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::AllowBreakInTarget(AllowBreakInTargetParams {
                        invader_player_id: request.invading_player_id,
                        join_data: request.join_data.clone(),
                        unk1: 0x0,
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseAllowBreakInTargetParams {})
    }
}

impl HandleRequest<Box<RequestRejectBreakInTargetParams>, ResponseRejectBreakInTargetParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestRejectBreakInTargetParams>,
    ) -> Result<ResponseRejectBreakInTargetParams, Box<dyn std::error::Error>> {
        let pool_key = BreakInPoolKey(self.session.player_id);

        let attempt = BREAKIN_ATTEMPTS
            .remove(&(pool_key, request.invading_player_id))
            .ok_or(Error::BreakInAttemptNotFound)?;

        attempt.invader_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::RejectBreakInTarget(RejectBreakInTargetParams {
                        invader_player_id: request.invading_player_id,
                        reason: request.reason,
                        invader_external_id: String::new(),
                        unk2: 0,
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseRejectBreakInTargetParams {})
    }
}
