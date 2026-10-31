use message::{
    builder::MessageBuilder,
    eldenring::{
        JoinParams, JoinPayload, ObjectIdentifier, PlayRegionArea, PuddleArea, PushParams,
        RejectSignParams, RequestCreateMatchAreaSignParams, RequestCreateSignParams,
        RequestGetMatchAreaSignListParams, RequestGetSignListParams, RequestRejectSignParams,
        RequestRemoveSignParams, RequestSummonSignParams, RequestUpdateSignParams,
        ResponseCreateMatchAreaSignParams, ResponseCreateSignParams,
        ResponseGetMatchAreaSignListParams, ResponseGetMatchAreaSignListParamsEntry,
        ResponseGetSignListParams, ResponseGetSignListParamsEntry, ResponseRejectSignParams,
        ResponseRemoveSignParams, ResponseSummonSignParams, ResponseUpdateSignParams, ServerError,
        SummonSignParams,
    },
};
use rand::prelude::*;

use crate::{
    handler::HandleRequest,
    services::eldenring::{
        area::MatchingArea,
        sign::{
            PuddleSignPoolQuery, SignPoolEntry, SignPoolQuery, SummonAttempt, SummonAttemptKey,
        },
    },
};

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestCreateSignParams>, ResponseCreateSignParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateSignParams>,
    ) -> Result<ResponseCreateSignParams, Box<dyn std::error::Error>> {
        let token = self.services.pool_sign.insert(SignPoolEntry {
            id: ObjectIdentifier::default(),
            player_id: self.session.player_id,
            external_id: self.session.external_id.clone(),
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: request.area.area,
                play_region: request.area.play_region,
            }),
            password: request.matching_parameters.password.clone().into(),
            group_passwords: request.group_passwords.clone(),
            data: request.data.clone(),
            summonee_tx: self.push_tx.clone(),
            updated_at: tokio::time::Instant::now(),
        });

        let identifier = *token.key();
        self.sign_token = Some(token);

        Ok(ResponseCreateSignParams { identifier })
    }
}

impl HandleRequest<Box<RequestCreateMatchAreaSignParams>, ResponseCreateMatchAreaSignParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateMatchAreaSignParams>,
    ) -> Result<ResponseCreateMatchAreaSignParams, Box<dyn std::error::Error>> {
        let token = self.services.pool_sign.insert(SignPoolEntry {
            id: ObjectIdentifier::default(),
            player_id: self.session.player_id,
            external_id: self.session.external_id.clone(),
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            location: MatchingArea::Puddle(PuddleArea {
                match_area: request.puddle.match_area,
                flags: request.puddle.flags,
            }),
            password: request.matching_parameters.password.clone().into(),
            group_passwords: request.group_passwords.clone(),
            data: request.data.clone(),
            summonee_tx: self.push_tx.clone(),
            updated_at: tokio::time::Instant::now(),
        });

        let identifier = *token.key();
        self.sign_token = Some(token);

        Ok(ResponseCreateMatchAreaSignParams { identifier })
    }
}

impl HandleRequest<Box<RequestUpdateSignParams>, ResponseUpdateSignParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestUpdateSignParams>,
    ) -> Result<ResponseUpdateSignParams, Box<dyn std::error::Error>> {
        if self.services.pool_sign.touch(&request.identifier).is_err() {
            Err(Box::new(ServerError::NotFound))
        } else {
            Ok(ResponseUpdateSignParams {
                identifier: request.identifier,
                unk0: 0,
            })
        }
    }
}

impl HandleRequest<Box<RequestGetSignListParams>, ResponseGetSignListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetSignListParams>,
    ) -> Result<ResponseGetSignListParams, Box<dyn std::error::Error>> {
        let mut pool_matches = self.services.pool_sign.matches(&SignPoolQuery {
            player_id: self.session.player_id,
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            areas: &request.search_areas,
            password: &request.matching_parameters.password,
        });

        let known_signs = pool_matches
            .iter()
            .filter_map(|e| {
                if request.known_signs.contains(&e.id) {
                    Some(e.id)
                } else {
                    None
                }
            })
            .collect::<Vec<ObjectIdentifier>>();

        pool_matches.retain(|e| !known_signs.contains(&e.id));

        Ok(ResponseGetSignListParams {
            known_signs,
            entries: pool_matches
                .iter()
                .map(|e| {
                    let MatchingArea::PlayRegion(area) = e.location.clone() else {
                        panic!("Filter tried passing puddle sign to GetSignList response.");
                    };

                    ResponseGetSignListParamsEntry {
                        player_id: e.player_id,
                        identifier: e.id,
                        area,
                        data: e.data.clone(),
                        external_id: e.external_id.clone(),
                        unk1: 0,
                        group_passwords: e.group_passwords.clone(),
                    }
                })
                .collect(),
        })
    }
}

impl HandleRequest<Box<RequestGetMatchAreaSignListParams>, ResponseGetMatchAreaSignListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetMatchAreaSignListParams>,
    ) -> Result<ResponseGetMatchAreaSignListParams, Box<dyn std::error::Error>> {
        let mut pool_matches = self
            .services
            .pool_sign
            .matches_puddle(&PuddleSignPoolQuery {
                player_id: self.session.player_id,
                character_level: request.matching_parameters.character_level,
                weapon_level: request.matching_parameters.max_reinforce as u32,
                puddles: request
                    .puddles
                    .iter()
                    .map(|p| PuddleArea {
                        match_area: p.puddle_id,
                        flags: p.flags_to_u64(),
                    })
                    .collect(),
                password: &request.matching_parameters.password,
            });

        let known_signs = pool_matches
            .iter()
            .filter_map(|e| {
                if request.known_signs.contains(&e.id) {
                    Some(e.id)
                } else {
                    None
                }
            })
            .collect::<Vec<ObjectIdentifier>>();

        pool_matches.retain(|e| !known_signs.contains(&e.id));

        Ok(ResponseGetMatchAreaSignListParams {
            known_signs,
            entries: pool_matches
                .iter()
                .map(|e| {
                    let MatchingArea::Puddle(ref puddle) = e.location else {
                        panic!("Filter tried passing non-puddle sign to GetMatchAreaSignList response.");
                    };

                    ResponseGetMatchAreaSignListParamsEntry {
                        player_id: e.player_id,
                        identifier: e.id,
                        puddle: puddle.clone(),
                        data: e.data.clone(),
                        external_id: e.external_id.clone(),
                        unk2: 0,
                        group_passwords: e.group_passwords.clone(),
                    }
                })
                .collect(),
        })
    }
}

impl HandleRequest<Box<RequestRemoveSignParams>, ResponseRemoveSignParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestRemoveSignParams>,
    ) -> Result<ResponseRemoveSignParams, Box<dyn std::error::Error>> {
        if self
            .sign_token
            .as_ref()
            .is_some_and(|token| token.key() == &request.sign_identifier)
        {
            self.sign_token = None;
        }

        Ok(ResponseRemoveSignParams {})
    }
}

impl HandleRequest<Box<RequestSummonSignParams>, ResponseSummonSignParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestSummonSignParams>,
    ) -> Result<ResponseSummonSignParams, Box<dyn std::error::Error>> {
        // Retrieve pool entry for tapped sign if it's still there.
        let Some(entry) = self.services.pool_sign.get(&request.identifier) else {
            return Err(Box::new(ServerError::NotFound));
        };

        let summoner_id = self.session.player_id;
        self.services.summon_attempts.insert(
            SummonAttemptKey {
                sign_id: request.identifier,
                summoner_id,
            },
            SummonAttempt {
                summoner_tx: self.push_tx.clone(),
            },
        );

        entry.summonee_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::SummonSign(SummonSignParams {
                        summoning_player_id: summoner_id,
                        summoning_player_external_id: self.session.external_id.clone(),
                        summonee_player_id: request.player_id,
                        sign_identifier: request.identifier,
                        join_data: request.data.clone(),
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseSummonSignParams {})
    }
}

impl HandleRequest<Box<RequestRejectSignParams>, ResponseRejectSignParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestRejectSignParams>,
    ) -> Result<ResponseRejectSignParams, Box<dyn std::error::Error>> {
        let attempt = self
            .services
            .summon_attempts
            .remove(&SummonAttemptKey {
                sign_id: request.sign_identifier,
                summoner_id: request.summoning_player_id,
            })
            .ok_or(ServerError::NotFound)?;

        attempt.summoner_tx.send(
            MessageBuilder::push()
                .body(PushParams::Join(JoinParams {
                    identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                    join_payload: JoinPayload::RejectSign(RejectSignParams {
                        sign_identifier: request.sign_identifier,
                        summoned_player_id: self.session.player_id,
                    }),
                }))
                .build()?,
        )?;

        Ok(ResponseRejectSignParams {})
    }
}
