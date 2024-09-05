use waygate_message::armoredcore6::*;

use crate::HandlerResult;

pub async fn handle_register_ugc(request: Box<RequestRegisterUGCParams>) -> HandlerResult {
    Ok(ResponseParams::RegisterUGC(ResponseRegisterUGCParams {
        ugc_code: String::from("CHEESE-CAKE"),
    }))
}

pub async fn handle_get_ugc_status(request: Box<RequestGetUGCStatusParams>) -> HandlerResult {
    Ok(ResponseParams::GetUGCStatus(ResponseGetUGCStatusParams {
        entries: vec![],
    }))
}
