use std::borrow::BorrowMut;

use fnrpc::player::RequestGetItemLogParams;
use fnrpc::player::RequestKillEnemyLogParams;
use fnrpc::player::RequestUpdatePlayerStatusParams;
use fnrpc::player::RequestUseItemLogParams;
use rand::prelude::*;
use fnrpc::ResponseParams;
use crate::pool::breakin::InvasionPoolEntry;
use crate::pool::invasion_pool_mut;
use crate::rpc;
use crate::push;
use crate::session::ClientSession;

type UpdateListener = fn(
    ClientSession,
    &RequestUpdatePlayerStatusParams,
);

pub async fn handle_update_player_status(
    session: ClientSession,
    params: RequestUpdatePlayerStatusParams
) -> rpc::HandlerResult {
    log::info!("Player sent UpdatePlayerStatus. player = {}", session.player_id);

    #[cfg(feature = "debug-push")]
    listen_debug_notifs(&params, &session).await;

    Ok(ResponseParams::UpdatePlayerStatus)
}

async fn listen_debug_notifs(params: &RequestUpdatePlayerStatusParams, session: &ClientSession) {
    if params.character.group_passwords.iter().any(|e| e == "freebuff") {
        push::send_push(session.player_id, get_test_buff()).await.unwrap();
    }

    if params.character.group_passwords.iter().any(|e| e == "announce") {
        push::send_push(session.player_id, get_test_announcement()).await.unwrap();
    }
}

fn get_test_buff() -> fnrpc::push::PushParams {
    let push_id_1 = { thread_rng().gen::<i32>() };
    let push_id_2 = { thread_rng().gen::<i32>() };

    fnrpc::push::PushParams::Notify(fnrpc::push::PushNotifyParams {
        identifier: fnrpc::shared::ObjectIdentifier {
            object_id: push_id_1,
            secondary_id: push_id_2,
        },
        timestamp: 0,
        section1: fnrpc::push::PushNotifyParamsSection1::Variant2 {
            unk1: 0x0,
            password: String::from("freebuff"),
        },
        section2: fnrpc::push::PushNotifyParamsSection2::Variant2 {
            unk1: 0x0,
            password: String::from("freebuff"),
            player_data: Vec::from([
                0x03, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x55, 0xA1, 0x00, 
                0x47, 0x00, 0x72, 0x00, 0x69, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6D, 0x00, 0x75, 0x00, 0x6E, 0x00, 
                0x64, 0x00, 0x75, 0x00, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
                0x00, 0x00, 0x00, 0x00
            ]),
        },
    })
}

fn get_test_announcement() -> fnrpc::push::PushParams {
    let push_id_1 = { thread_rng().gen::<i32>() };
    let push_id_2 = { thread_rng().gen::<i32>() };

    fnrpc::push::PushParams::Notify(fnrpc::push::PushNotifyParams {
        identifier: fnrpc::shared::ObjectIdentifier {
            object_id: push_id_1,
            secondary_id: push_id_2,
        },
        timestamp: 0,
        section1: fnrpc::push::PushNotifyParamsSection1::Variant1 {
            unk1: 0x0,
            unk2: 0x0,
        },
        section2: fnrpc::push::PushNotifyParamsSection2::Variant1 {
            message: "Test Announcement".to_string(),
            unk1: 0x0,
            unk2: 0x0,
            unk3: 0x0,
        },
    })
}

impl From<(String, &RequestUpdatePlayerStatusParams)> for InvasionPoolEntry {
    fn from(value: (String, &RequestUpdatePlayerStatusParams)) -> Self {
        Self {
            character_level: value.1.character.level as u16,
            weapon_level: value.1.character.max_reinforce_level as u16,
            steam_id: value.0.to_string(),
        }
    }
}

pub async fn handle_use_item_log(
    session: ClientSession,
    params: RequestUseItemLogParams,
) -> rpc::HandlerResult {
    log::info!("Player sent UseItemLog. player = {}", session.player_id);
    log::info!("params = {:#?}", params);

    Ok(ResponseParams::UseItemLog)
}

pub async fn handle_get_item_log(
    session: ClientSession,
    params: RequestGetItemLogParams,
) -> rpc::HandlerResult {
    log::info!("Player sent GetItemLog. player = {}", session.player_id);
    log::info!("params = {:#?}", params);

    Ok(ResponseParams::UseItemLog)
}

pub async fn handle_kill_enemy_log(
    session: ClientSession,
    params: RequestKillEnemyLogParams,
) -> rpc::HandlerResult {
    log::info!("Player sent KillEnemyLog. player = {}", session.player_id);
    log::info!("params = {:#?}", params);

    Ok(ResponseParams::KillEnemyLog)
}
