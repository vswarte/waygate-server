use std::time::Duration;

use message::{
    builder::MessageBuilder,
    eldenring::{
        JoinParams, JoinPayload, ObjectIdentifier, PushParams, RejectVisitParams,
        RequestGetVisitorListParams, RequestRejectVisitParams, RequestVisitParams,
        ResponseGetVisitorListParams, ResponseGetVisitorListParamsEntry, ResponseRejectVisitParams,
        ResponseVisitParams, VisitParams,
    },
};
use rand::Rng;
use thiserror::Error;

use crate::{
    handler::HandleRequest,
    services::eldenring::visit::{
        VisitorAttempt, VisitorPoolKey, VisitorPoolQuery, VISIT_ATTEMPTS,
    },
};

use super::DefaultClientHandler;

#[derive(Debug, Error)]
enum Error {
    #[error("Visitor could not be found.")]
    VisitorNotFound,
    #[error("Visit attempt could not be found.")]
    VisitAttemptNotFound,
}

const ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);

impl HandleRequest<Box<RequestGetVisitorListParams>, ResponseGetVisitorListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetVisitorListParams>,
    ) -> Result<ResponseGetVisitorListParams, Box<dyn std::error::Error>> {
        let pool_matches = self.services.pool_visitor.matches(&VisitorPoolQuery {
            play_region: 0,
            character_level: request.matching_parameters.character_level as u32,
            weapon_level: request.matching_parameters.max_reinforce as u32,
        });

        Ok(ResponseGetVisitorListParams {
            entries: pool_matches
                .into_iter()
                .map(|e| ResponseGetVisitorListParamsEntry {
                    player_id: e.1.player_id,
                    external_id: e.1.external_id.clone(),
                    unk1: 0,
                    play_region: 0,
                    unk2: 0,
                })
                .collect(),
        })
    }
}

impl HandleRequest<Box<RequestVisitParams>, ResponseVisitParams> for DefaultClientHandler<'_> {
    async fn handle(
        &mut self,
        request: &Box<RequestVisitParams>,
    ) -> Result<ResponseVisitParams, Box<dyn std::error::Error>> {
        // Retrieve pool entry for tapped sign if it's still there.
        let pool_key = VisitorPoolKey(request.player_id);
        let Some(entry) = self.services.pool_visitor.get(&pool_key) else {
            return Err(Box::new(Error::VisitorNotFound));
        };

        // Save our connection attempt for cancellation if required.
        let summoner_id = self.session.player_id;
        VISIT_ATTEMPTS.insert(
            (pool_key.clone(), summoner_id),
            VisitorAttempt {
                summoner_tx: self.push_tx.clone(),
            },
        );

        // Remove attempt entry after a while so it doesn't linger if the summoning was
        // successful or the summonee goes offline.
        {
            let pool_key = pool_key.clone();
            tokio::spawn(async move {
                tokio::time::sleep(ATTEMPT_CLEANUP_TIMEOUT).await;
                let _ = VISIT_ATTEMPTS.remove(&(pool_key, summoner_id));
            });
        }

        entry.visitor_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::Visit(VisitParams {
                        host_player_id: summoner_id,
                        host_player_external_id: self.session.external_id.clone(),
                        join_data: request.join_data.clone(),
                        unk1: 0,
                        unk2: 0,
                        play_region: 0,
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseVisitParams {})
    }
}

impl HandleRequest<Box<RequestRejectVisitParams>, ResponseRejectVisitParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestRejectVisitParams>,
    ) -> Result<ResponseRejectVisitParams, Box<dyn std::error::Error>> {
        let pool_key = VisitorPoolKey(request.host_player_id);

        let attempt = VISIT_ATTEMPTS
            .remove(&(pool_key, request.host_player_id))
            .ok_or(Error::VisitAttemptNotFound)?;

        // attempt.summoner_tx.send(
        //     MessageBuilder::push()
        //         .body(PushParams::Join(JoinParams {
        //             identifier: ObjectIdentifier(rand::rng().random::<i64>()),
        //             join_payload: JoinPayload::RejectVisit(RejectVisitParams {
        //                 unk1: todo!(),
        //                 unk2: todo!(),
        //                 unk3: todo!(),
        //                 unk4: todo!(),
        //             }),
        //         }))
        //         .build()?,
        // )?;

        Ok(ResponseRejectVisitParams {})
    }
}
