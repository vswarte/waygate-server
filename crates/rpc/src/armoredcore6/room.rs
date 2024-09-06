use std::sync::atomic::Ordering;

use thiserror::Error;
use waygate_connection::ClientSession;
use waygate_message::armoredcore6::{
    RequestCreateRoomParams, RequestDeleteRoomParams, RequestGenerateRoomBattleIDParams, RequestGetRoomListParams, RequestUpdateRoomParams, ResponseCreateRoomParams, ResponseDeleteRoomParams, ResponseGenerateRoomBattleIDParams, ResponseGetRoomListParams, ResponseParams, ResponseUpdateRoomParams
};
use waygate_pool::{
    armoredcore6::room::{normalize_room_keyword, RoomPoolEntry, RoomPoolQuery},
    ROOM_POOL,
};

#[derive(Debug, Error)]
pub enum RoomError {
    #[error("Could not find room for player")]
    MissingRoom,

    #[error("Could not obtain lock for room")]
    RoomLock,
}

use crate::HandlerResult;

pub async fn handle_get_room_list(request: Box<RequestGetRoomListParams>) -> HandlerResult {
    let entries = ROOM_POOL
        .match_entries::<RoomPoolQuery>(&request.into())
        .iter()
        .map(|m| m.into())
        .collect::<Vec<_>>();

    Ok(ResponseParams::GetRoomList(ResponseGetRoomListParams {
        entries,
    }))
}

pub async fn handle_create_room(
    session: ClientSession,
    request: Box<RequestCreateRoomParams>,
) -> HandlerResult {
    let entry = RoomPoolEntry::from(request);
    let key = ROOM_POOL.insert(0, entry)?;

    let room_id = key.1 .0 as u32;
    session.game_session_mut().room = Some(key);

    Ok(ResponseParams::CreateRoom(ResponseCreateRoomParams {
        room_id,
    }))
}

pub async fn handle_update_room(
    session: ClientSession,
    request: Box<RequestUpdateRoomParams>,
) -> HandlerResult {
    dbg!(&request);

    // Normalize input because the client just sends in the keyword with a
    // fixed size lenght of 33?
    let mut room_details = request.room_details;
    room_details.keywords = normalize_room_keyword(room_details.keywords.as_str());

    let session = &session.game_session_mut();
    let guard = session.room.as_ref().unwrap();
    let pool_entry = ROOM_POOL.by_key(&guard.1)
        .ok_or(RoomError::MissingRoom)?;

    let mut lock = pool_entry.room_details.write()
        .map_err(|_| RoomError::RoomLock)?;

    *lock = room_details;
    pool_entry.players_in_room.store(request.players_in_room, Ordering::Relaxed);

    Ok(ResponseParams::UpdateRoom(ResponseUpdateRoomParams {}))
}

pub async fn handle_delete_room(
    session: ClientSession,
    _request: Box<RequestDeleteRoomParams>,
) -> HandlerResult {
    // let _ = session.game_session_mut().room.take();

    Ok(ResponseParams::DeleteRoom(ResponseDeleteRoomParams {}))
}

pub async fn handle_generate_room_battle_id(
    request: Box<RequestGenerateRoomBattleIDParams>,
) -> HandlerResult {
    // let _ = session.game_session_mut().room.take();

    Ok(ResponseParams::GenerateRoomBattleID(ResponseGenerateRoomBattleIDParams {
        unk1: 87874609782342,
    }))
}
