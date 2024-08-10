use waygate_message::*;

use crate::HandlerResult;

pub async fn handle_register_ugc() -> HandlerResult {
    Ok(ResponseParams::RegisterUGC(
        ResponseRegisterUGCParams {
            unk1: 0,
            unk2: 0,
        }
    ))
}
