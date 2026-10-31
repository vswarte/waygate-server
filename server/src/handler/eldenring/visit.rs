use message::{
    builder::MessageBuilder,
    eldenring::{
        JoinParams, JoinPayload, ObjectIdentifier, PushParams, RejectVisitParams,
        RequestGetVisitorListParams, RequestRejectVisitParams, RequestVisitParams,
        ResponseGetVisitorListParams, ResponseGetVisitorListParamsEntry, ResponseRejectVisitParams,
        ResponseVisitParams, ServerError, VisitParams,
    },
};
use rand::{seq::SliceRandom, Rng};

use crate::{
    handler::HandleRequest,
    services::eldenring::visit::{
        VisitAttemptKey, VisitorAttempt, VisitorPoolKey, VisitorPoolQuery,
    },
};

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestGetVisitorListParams>, ResponseGetVisitorListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetVisitorListParams>,
    ) -> Result<ResponseGetVisitorListParams, Box<dyn std::error::Error>> {
        let mut pool_matches = self.services.pool_visitor.matches(&VisitorPoolQuery {
            player_id: self.session.player_id,
            play_region: 0,
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            visit_type: request.visit_type,
        });
        pool_matches.shuffle(&mut rand::rng());

        Ok(ResponseGetVisitorListParams {
            entries: pool_matches
                .into_iter()
                .map(|e| ResponseGetVisitorListParamsEntry {
                    player_id: e.player_id,
                    external_id: e.external_id.clone(),
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
        let pool_key = VisitorPoolKey {
            player_id: request.player_id,
            visit_type: request.visit_type,
        };
        let Some(entry) = self.services.pool_visitor.get(&pool_key) else {
            return Err(Box::new(ServerError::NotFound));
        };

        // Save our connection attempt for cancellation if required.
        let summoner_id = self.session.player_id;
        self.services.visit_attempts.insert(
            VisitAttemptKey {
                visitor_id: pool_key.clone(),
                summoner_id,
            },
            VisitorAttempt {
                summoner_tx: self.push_tx.clone(),
            },
        );

        entry.visitor_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::Visit(VisitParams {
                        host_player_id: summoner_id,
                        host_player_external_id: self.session.external_id.clone(),
                        join_data: request.join_data.clone(),
                        // TODO: figure out how the game processes these fields
                        unk1: request.visit_type as u32,
                        unk2: request.visit_type as u32,
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
        let attempt = self
            .services
            .visit_attempts
            .remove(&VisitAttemptKey {
                visitor_id: VisitorPoolKey {
                    player_id: self.session.player_id,
                    visit_type: request.visit_type,
                },
                summoner_id: request.host_player_id,
            })
            .ok_or(ServerError::NotFound)?;

        attempt.summoner_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::RejectVisit(RejectVisitParams {
                        visitor_player_id: self.session.player_id,
                        unk2: 0,
                        unk3: String::new(),
                        visit_type: request.visit_type,
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseRejectVisitParams {})
    }
}
