use crate::rpc;
use crate::rpc::message::*;

pub async fn handle_register_ugc() -> rpc::HandlerResult {
    Ok(ResponseParams::RegisterUGC(
        ResponseRegisterUGCParams {
            unk1: 0,
            unk2: 0,
        }
    ))
}
