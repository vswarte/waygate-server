use fnrpc::player::RequestGetItemLogParams;
use fnrpc::player::RequestKillEnemyLogParams;
use fnrpc::player::RequestUpdatePlayerStatusParams;
use fnrpc::player::RequestUseItemLogParams;
use fnrpc::ResponseParams;

use crate::pool::breakin::BreakInPoolEntry;
use crate::rpc;
use crate::session::ClientSession;

pub async fn handle_update_player_status(
    session: ClientSession,
    params: RequestUpdatePlayerStatusParams
) -> rpc::HandlerResult {
    log::info!("Player sent UpdatePlayerStatus. player = {}", session.player_id);

    debug::listen_debug_notifs(&params, &session).await;

    Ok(ResponseParams::UpdatePlayerStatus)
}


impl From<(String, &RequestUpdatePlayerStatusParams)> for BreakInPoolEntry {
    fn from(value: (String, &RequestUpdatePlayerStatusParams)) -> Self {
        Self {
            character_level: value.1.character.level as u16,
            weapon_level: value.1.character.max_reinforce_level as u16,
            steam_id: value.0.to_string(),
        }
    }
}

pub async fn handle_use_item_log(
    _session: ClientSession,
    _params: RequestUseItemLogParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::UseItemLog)
}

pub async fn handle_get_item_log(
    _session: ClientSession,
    _params: RequestGetItemLogParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::UseItemLog)
}

pub async fn handle_kill_enemy_log(
    _session: ClientSession,
    _params: RequestKillEnemyLogParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::KillEnemyLog)
}

mod debug {
    use fnrpc::{
        player::RequestUpdatePlayerStatusParams,
        push::{
            JoinPayload,
            SummonSignParams,
            NotifyParamsSection1,
            NotifyParamsSection2,
            PushParams,
        },
        shared::ObjectIdentifier
    };
    use rand::prelude::*;

    use crate::{push::send_push, session::ClientSession};

    pub async fn listen_debug_notifs(params: &RequestUpdatePlayerStatusParams, session: &ClientSession) {
        if params.character.group_passwords.iter().any(|e| e == "_summon") {
            send_push(session.player_id, get_test_summon(session.player_id)).await.unwrap();
        }

        if params.character.group_passwords.iter().any(|e| e == "_buff") {
            send_push(session.player_id, get_test_buff()).await.unwrap();
        }

        if params.character.group_passwords.iter().any(|e| e == "_msg") {
            send_push(session.player_id, get_test_announcement()).await.unwrap();
        }
    }

    /// Sends a message your way that you were summoned
    fn get_test_summon(recipient_id: i32) -> PushParams {
        let push_id_1 = { thread_rng().gen::<i32>() };
        let push_id_2 = { thread_rng().gen::<i32>() };

        let player_id = { thread_rng().gen::<i32>() };
        let session_id = { thread_rng().gen::<i32>() };
        let sign_id = { thread_rng().gen::<i32>() };

        log::info!(
            "Sending test join. player_id = {}. session_id = {}. sign_id = {}.",
            player_id,
            session_id,
            sign_id,
        );

        PushParams::Join(fnrpc::push::JoinParams {
            identifier: fnrpc::shared::ObjectIdentifier {
                object_id: push_id_1,
                secondary_id: push_id_2,
            },
            join_payload: JoinPayload::SummonSign(SummonSignParams {
                summoning_player_id: player_id,
                steam_id: String::from("1100001000056c0"),
                summoned_player_id: recipient_id,
                sign_identifier: ObjectIdentifier {
                    object_id: sign_id,
                    secondary_id: session_id,
                },
                join_data: Vec::new(),
            }),
        })
    }

    fn get_test_buff() -> PushParams {
        let push_id_1 = { thread_rng().gen::<i32>() };
        let push_id_2 = { thread_rng().gen::<i32>() };

        PushParams::Notify(fnrpc::push::NotifyParams {
            identifier: fnrpc::shared::ObjectIdentifier {
                object_id: push_id_1,
                secondary_id: push_id_2,
            },
            timestamp: 0,
            section1: NotifyParamsSection1::Variant2 {
                unk1: 0x0,
                password: String::from("_buff"),
            },
            section2: NotifyParamsSection2::Variant2 {
                unk1: 0x0,
                password: String::from("_buff"),
                player_data: Vec::from([
                    0x03, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x55, 0xA1, 0x00, 
                    0x47, 0x00, 0x72, 0x00, 0x69, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6D, 0x00, 0x75, 0x00, 0x6E, 0x00, 
                    0x64, 0x00, 0x75, 0x00, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x00
                ]),
            },
        })
    }

    fn get_test_announcement() -> PushParams {
        let push_id_1 = { thread_rng().gen::<i32>() };
        let push_id_2 = { thread_rng().gen::<i32>() };

        PushParams::Notify(fnrpc::push::NotifyParams {
            identifier: fnrpc::shared::ObjectIdentifier {
                object_id: push_id_1,
                secondary_id: push_id_2,
            },
            timestamp: 0,
            section1: NotifyParamsSection1::Variant1 {
                unk1: 0x0,
                unk2: 0x0,
            },
            section2: NotifyParamsSection2::Variant1 {
                message: "Test Announcement".to_string(),
                unk1: 0x0,
                unk2: 0x0,
                unk3: 0x0,
            },
        })
    }
}
