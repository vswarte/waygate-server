use waygate_message::armoredcore6::*;

use crate::HandlerResult;

pub async fn handle_update_login_player_character(
) -> HandlerResult {
    Ok(ResponseParams::UpdateLoginPlayerCharacter(ResponseUpdateLoginPlayerCharacterParams {
        character_id: 0x0,
        unk1: 0x0,
        unk2: 0x0,
        unk3: 0x0,
        unk4: 0x0,
        unk5: 0x0,
        unk6: 0x0,
    }))
}
