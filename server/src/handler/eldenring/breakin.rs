use message::{
    builder::MessageBuilder,
    eldenring::{
        AllowBreakInTargetParams, BreakInTargetParams, JoinParams, JoinPayload, ObjectIdentifier,
        PushParams, RejectBreakInTargetParams, RequestAllowBreakInTargetParams,
        RequestBreakInTargetParams, RequestGetBreakInTargetListParams,
        RequestRejectBreakInTargetParams, ResponseAllowBreakInTargetParams,
        ResponseBreakInTargetParams, ResponseGetBreakInTargetListParams,
        ResponseGetBreakInTargetListParamsEntry, ResponseRejectBreakInTargetParams, ServerError,
    },
};
use rand::prelude::*;

use crate::{
    handler::HandleRequest,
    services::eldenring::breakin::{
        BreakInAttempt, BreakInAttemptKey, BreakInPoolKey, BreakInPoolQuery,
    },
};

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestGetBreakInTargetListParams>, ResponseGetBreakInTargetListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetBreakInTargetListParams>,
    ) -> Result<ResponseGetBreakInTargetListParams, Box<dyn std::error::Error>> {
        let entries = self.services.pool_breakin.matches(&BreakInPoolQuery {
            player_id: self.session.player_id,
            play_region: request.play_region,
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            limit: request.max_count,
        });

        Ok(ResponseGetBreakInTargetListParams {
            play_region: request.play_region,
            entries: entries
                .into_iter()
                .map(|e| ResponseGetBreakInTargetListParamsEntry {
                    player_id: e.player_id,
                    external_id: e.external_id.clone(),
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
            return Err(Box::new(ServerError::NotFound));
        };

        let invader_id = self.session.player_id;

        self.services.breakin_attempts.insert(
            BreakInAttemptKey {
                target_id: pool_key.clone(),
                invader_id,
            },
            BreakInAttempt {
                invader_tx: self.push_tx.clone(),
            },
        );

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

        let attempt = self
            .services
            .breakin_attempts
            .remove(&BreakInAttemptKey {
                target_id: pool_key,
                invader_id: request.invading_player_id,
            })
            .ok_or(ServerError::NotFound)?;

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

        let attempt = self
            .services
            .breakin_attempts
            .remove(&BreakInAttemptKey {
                target_id: pool_key,
                invader_id: request.invading_player_id,
            })
            .ok_or(ServerError::NotFound)?;

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
