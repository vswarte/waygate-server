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
        ResponseRemoveSignParams, ResponseSummonSignParams, ResponseUpdateSignParams,
        SummonSignParams,
    },
};
use rand::prelude::*;
use thiserror::Error;

use crate::{
    handler::HandleRequest,
    services::eldenring::{
        area::MatchingArea,
        sign::{
            PuddleSignPoolQuery, SignPoolEntry, SignPoolKey, SignPoolQuery, SummonAttempt,
            SIGN_ATTEMPT_CLEANUP_TIMEOUT, SUMMON_ATTEMPTS,
        },
    },
};

use super::DefaultClientHandler;

#[derive(Debug, Error)]
enum Error {
    #[error("Sign could not be found.")]
    SignNotFound,
    #[error("Summon attempt could not be found.")]
    SummonAttemptNotFound,
}

impl HandleRequest<Box<RequestCreateSignParams>, ResponseCreateSignParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateSignParams>,
    ) -> Result<ResponseCreateSignParams, Box<dyn std::error::Error>> {
        let token = self.services.pool_sign.insert(SignPoolEntry {
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
        });

        let identifier = ObjectIdentifier(token.1 .0);
        self.sign_tokens.insert(identifier, token);

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
            player_id: self.session.player_id,
            external_id: self.session.external_id.clone(),
            character_level: request.matching_parameters.character_level,
            weapon_level: request.matching_parameters.max_reinforce as u32,
            location: MatchingArea::Puddle(PuddleArea {
                puddle_id: request.puddle.puddle_id,
                flags: request.puddle.flags,
            }),
            password: request.matching_parameters.password.clone().into(),
            group_passwords: request.group_passwords.clone(),
            data: request.data.clone(),
            summonee_tx: self.push_tx.clone(),
        });

        let identifier = ObjectIdentifier(token.1 .0);
        self.sign_tokens.insert(identifier, token);

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
        if !self
            .services
            .pool_sign
            .has(&SignPoolKey(request.identifier.0))
        {
            Err(Box::new(Error::SignNotFound))
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
                let identifier = ObjectIdentifier(e.0 .0);
                if request.known_signs.contains(&identifier) {
                    Some(identifier)
                } else {
                    None
                }
            })
            .collect::<Vec<ObjectIdentifier>>();

        pool_matches.retain(|e| !known_signs.contains(&ObjectIdentifier(e.0 .0)));

        Ok(ResponseGetSignListParams {
            known_signs,
            entries: pool_matches
                .iter()
                .map(|e| {
                    let MatchingArea::PlayRegion(area) = e.1.location.clone() else {
                        panic!("Filter tried passing puddle sign to GetSignList response.");
                    };

                    ResponseGetSignListParamsEntry {
                        player_id: e.1.player_id,
                        identifier: ObjectIdentifier(e.0 .0),
                        area,
                        data: e.1.data.clone(),
                        external_id: e.1.external_id.clone(),
                        unk1: 0,
                        group_passwords: e.1.group_passwords.clone(),
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
        log::info!("GetMatchAreaSignList {:?}", request.puddles);

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
                        puddle_id: p.puddle_id,
                        flags: p.flags_to_u64(),
                    })
                    .collect(),
                password: &request.matching_parameters.password,
            });

        let known_signs = pool_matches
            .iter()
            .filter_map(|e| {
                let identifier = ObjectIdentifier(e.0 .0);
                if request.known_signs.contains(&identifier) {
                    Some(identifier)
                } else {
                    None
                }
            })
            .collect::<Vec<ObjectIdentifier>>();

        pool_matches.retain(|e| !known_signs.contains(&ObjectIdentifier(e.0 .0)));

        Ok(ResponseGetMatchAreaSignListParams {
            known_signs,
            entries: pool_matches
                .iter()
                .map(|e| {
                    let MatchingArea::Puddle(ref puddle) = e.1.location else {
                        panic!("Filter tried passing non-puddle sign to GetMatchAreaSignList response.");
                    };

                    ResponseGetMatchAreaSignListParamsEntry {
                        player_id: e.1.player_id,
                        identifier: ObjectIdentifier(e.0 .0),
                        puddle: puddle.clone(),
                        unk1: 0,
                        data: e.1.data.clone(),
                        external_id: e.1.external_id.clone(),
                        unk2: 0,
                        group_passwords: e.1.group_passwords.clone(),
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
        self.sign_tokens.remove(&request.sign_identifier);

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
        let pool_key = SignPoolKey(request.identifier.0);
        let Some(entry) = self.services.pool_sign.get(&pool_key) else {
            return Err(Box::new(Error::SignNotFound));
        };

        // Save our connection attempt for cancellation if required.
        let summoner_id = self.session.player_id;
        SUMMON_ATTEMPTS.insert(
            (pool_key.clone(), summoner_id),
            SummonAttempt {
                summoner_tx: self.push_tx.clone(),
            },
        );

        // Remove attempt entry after a while so it doesn't linger if the summoning was
        // successful or the summonee goes offline.
        {
            let pool_key = pool_key.clone();
            tokio::spawn(async move {
                tokio::time::sleep(SIGN_ATTEMPT_CLEANUP_TIMEOUT).await;
                let _ = SUMMON_ATTEMPTS.remove(&(pool_key, summoner_id));
            });
        }

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
        let pool_key = SignPoolKey(request.sign_identifier.0);

        let attempt = SUMMON_ATTEMPTS
            .remove(&(pool_key, request.summoning_player_id))
            .ok_or(Error::SummonAttemptNotFound)?;

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
