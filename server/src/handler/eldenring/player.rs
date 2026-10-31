use message::eldenring::{
    RequestUpdateLoginPlayerCharacterParams, RequestUpdatePlayerStatusParams,
    ResponseUpdateLoginPlayerCharacterParams, ResponseUpdatePlayerStatusParams, VisitType,
};

use crate::{
    handler::HandleRequest,
    services::eldenring::{breakin::BreakInPoolEntry, visit::VisitorPoolEntry},
};

use super::DefaultClientHandler;

impl
    HandleRequest<
        Box<RequestUpdateLoginPlayerCharacterParams>,
        ResponseUpdateLoginPlayerCharacterParams,
    > for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestUpdateLoginPlayerCharacterParams>,
    ) -> Result<ResponseUpdateLoginPlayerCharacterParams, Box<dyn std::error::Error>> {
        Ok(ResponseUpdateLoginPlayerCharacterParams {
            character_id: request.character_id,
            quickmatch_united_combat_rank: 0,
            quickmatch_united_combat_points: 0,
            quickmatch_duel_rank: 0,
            quickmatch_duel_points: 0,
            quickmatch_spirit_ashes_rank: 0,
            quickmatch_spirit_ashes_points: 0,
        })
    }
}

impl HandleRequest<Box<RequestUpdatePlayerStatusParams>, ResponseUpdatePlayerStatusParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestUpdatePlayerStatusParams>,
    ) -> Result<ResponseUpdatePlayerStatusParams, Box<dyn std::error::Error>> {
        let character = &request.character;
        let multiplayer = &character.multiplayer_data;
        let now = tokio::time::Instant::now();

        let visitor_entry = |visit_type| VisitorPoolEntry {
            player_id: self.session.player_id,
            character_level: character.level,
            weapon_level: character.max_reinforce_level,
            play_region: 0,
            visit_type,
            external_id: self.session.external_id.clone(),
            visitor_tx: self.push_tx.clone(),
            updated_at: now,
        };
        let hunter = multiplayer
            .can_be_hunter
            .then(|| visitor_entry(VisitType::Hunter));
        let sinner_hunter = multiplayer
            .can_be_sinner_hunter
            .then(|| visitor_entry(VisitType::SinnerHunter));

        let breakin = multiplayer.is_invadeable.then(|| BreakInPoolEntry {
            player_id: self.session.player_id,
            character_level: character.level,
            weapon_level: character.max_reinforce_level,
            play_region: request.play_region,
            external_id: self.session.external_id.clone(),
            target_tx: self.push_tx.clone(),
            updated_at: now,
        });

        let services = self.services;
        services.pool_visitor.sync(&mut self.hunter_token, hunter);
        services
            .pool_visitor
            .sync(&mut self.sinner_hunter_token, sinner_hunter);
        services.pool_breakin.sync(&mut self.breakin_token, breakin);

        Ok(ResponseUpdatePlayerStatusParams {})
    }
}
