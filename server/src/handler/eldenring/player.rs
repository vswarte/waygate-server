use message::eldenring::{
    RequestUpdateLoginPlayerCharacterParams, RequestUpdatePlayerStatusParams,
    ResponseUpdateLoginPlayerCharacterParams, ResponseUpdatePlayerStatusParams,
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
        _request: &Box<RequestUpdateLoginPlayerCharacterParams>,
    ) -> Result<ResponseUpdateLoginPlayerCharacterParams, Box<dyn std::error::Error>> {
        Ok(ResponseUpdateLoginPlayerCharacterParams {
            character_id: 0,
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 0,
            unk5: 0,
            unk6: 0,
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
        if request.character.blue_ring_active && self.visitor_token.is_none() {
            let token = self.services.pool_visitor.insert(
                self.session.player_id,
                VisitorPoolEntry {
                    player_id: self.session.player_id,
                    character_level: request.character.level,
                    weapon_level: request.character.max_reinforce_level,
                    play_region: 0,
                    external_id: self.session.external_id.clone(),
                    visitor_tx: self.push_tx.clone(),
                },
            );

            self.visitor_token = Some(token);
        } else if !request.character.blue_ring_active && self.visitor_token.is_some() {
            let _ = self.visitor_token.take();
        }

        let is_invadeable = request.character.furled_finger_enabled == 1
            && (request.character.taunters_tongue_active
                || request.character.hosting_chr_types.contains(&1));

        match self.breakin_token.as_ref() {
            Some(token) if is_invadeable => {
                // Update existing breakin entry.
                self.services.pool_breakin.replace(
                    &token.1,
                    BreakInPoolEntry {
                        player_id: self.session.player_id,
                        character_level: request.character.level,
                        weapon_level: request.character.max_reinforce_level,
                        play_region: request.play_region,
                        external_id: self.session.external_id.clone(),
                        target_tx: self.push_tx.clone(),
                    },
                )?;
            }
            None if is_invadeable => {
                // Create breakin pool token if we've become potentially invadeable.
                self.breakin_token = Some(self.services.pool_breakin.insert(
                    self.session.player_id,
                    BreakInPoolEntry {
                        player_id: self.session.player_id,
                        character_level: request.character.level,
                        weapon_level: request.character.max_reinforce_level,
                        play_region: request.play_region,
                        external_id: self.session.external_id.clone(),
                        target_tx: self.push_tx.clone(),
                    },
                ));
            }
            Some(_) if !is_invadeable => {
                let _ = self.breakin_token.take();
            }
            _ => {}
        }

        Ok(ResponseUpdatePlayerStatusParams {})
    }
}
