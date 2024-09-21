use waygate_message::{RequestCreateBattleSessionParams, RequestCreateRoomParams, ResponseCreateBattleSessionParams, ResponseParams};

use crate::HandlerResult;

pub async fn handle_create_room(
    request: RequestCreateRoomParams,
) -> HandlerResult {
    // tracing::info!("Room data: {request:#?}");

    Ok(ResponseParams::CreateRoom)
}


