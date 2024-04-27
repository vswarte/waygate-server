use fnrpc::player::RequestGetItemLogParams;
use fnrpc::player::RequestKillEnemyLogParams;
use fnrpc::player::RequestUpdatePlayerStatusParams;
use fnrpc::player::RequestUseItemLogParams;
use fnrpc::ResponseParams;

use crate::pool::breakin::BreakInPoolEntry;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

pub async fn handle_update_player_status(
    session: ClientSession,
    request: RequestUpdatePlayerStatusParams
) -> rpc::HandlerResult {
    log::info!("Player sent UpdatePlayerStatus. player = {}", session.lock_read().player_id);

    debug::listen_debug_notifs(&request, session).await;

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
    _request: RequestUseItemLogParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::UseItemLog)
}

pub async fn handle_get_item_log(
    _session: ClientSession,
    _request: RequestGetItemLogParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::UseItemLog)
}

pub async fn handle_kill_enemy_log(
    _session: ClientSession,
    _request: RequestKillEnemyLogParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::KillEnemyLog)
}

mod debug {
    use fnrpc::{
        player::RequestUpdatePlayerStatusParams,
        push::{
            AllowBreakInTargetParams, BreakInTargetParams, JoinParams, JoinPayload, NotifyParamsSection1, NotifyParamsSection2, PushParams, SummonSignParams
        },
        shared::ObjectIdentifier
    };
    use rand::prelude::*;

    use crate::{pool::{self, breakin::BreakInPoolEntry}, push::send_push, session::{ClientSession, ClientSessionContainer}};

    pub async fn listen_debug_notifs(params: &RequestUpdatePlayerStatusParams, session: ClientSession) {
        {
            let player_id = session.lock_read().player_id;

            if params.character.group_passwords.iter().any(|e| e == "_summon") {
                let sign: Option<ObjectIdentifier> = session.lock_read().sign.as_ref()
                    .map(|s| s.into());

                if let Some(sign) = sign {
                    send_push(player_id, get_test_summon(sign)).await.unwrap();
                }
            }

            if params.character.group_passwords.iter().any(|e| e == "_buff") {
                send_push(player_id, get_test_buff()).await.unwrap();
            }

            if params.character.group_passwords.iter().any(|e| e == "_msg") {
                send_push(player_id, get_test_announcement()).await.unwrap();
            }
        }

        let mut session = session.lock_write();
        if params.character.group_passwords.iter().any(|e| e == "_invade") && session.breakin.is_none() {
            // send_push(player_id, get_test_invasion()).await.unwrap();
            let key = pool::breakin().unwrap()
                .insert(session.player_id, BreakInPoolEntry {
                    character_level: 96,
                    weapon_level: 20,
                    steam_id: session.external_id.clone(),
                })
                .unwrap();

            session.breakin = Some(key);
        }
    }

    /// Sends a message your way that you are being invaded
    fn get_test_invasion_offer() -> PushParams {
        PushParams::Join(JoinParams {
            identifier: ObjectIdentifier {
                object_id: rand::thread_rng().gen::<i32>(),
                secondary_id: rand::thread_rng().gen::<i32>(),
            },
            join_payload: JoinPayload::BreakInTarget(BreakInTargetParams {
                invader_player_id: 0x10123,
                invader_steam_id: "1100001000056c0".to_string(),
                unk1: 0x0,
                unk2: 0x0,
                play_region: 6100000,
            }),
        })
    }

    /// Sends a message your way that you going to be invading
    fn get_test_invasion() -> PushParams {
        PushParams::Join(JoinParams {
            identifier: ObjectIdentifier {
                object_id: rand::thread_rng().gen::<i32>(),
                secondary_id: rand::thread_rng().gen::<i32>(),
            },
            join_payload: JoinPayload::AllowBreakInTarget(AllowBreakInTargetParams {
                host_player_id: 0x10123,
                unk1: 0x0,
                join_data: vec![
                    0x38, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, 
                    0x52, 0xB8, 0x6E, 0x41, 0x73, 0xE8, 0x3F, 0x42, 0x0A, 0xD7, 0x45, 0x42, 0x1A, 0x5D, 0xF1, 0xBE, 
                    0x02, 0x00, 0x00, 0x00, 0xFD, 0xFF, 0xFF, 0xFF, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 
                    0x08, 0x00, 0x00, 0x00, 0x55, 0xE7, 0x7E, 0xAC, 0x00, 0x00, 0x86, 0x01, 
                ],
            }),
        })
    }


    /// Sends a message your way that you were summoned
    fn get_test_summon(sign: ObjectIdentifier) -> PushParams {
        let push_id_1 = { thread_rng().gen::<i32>() };
        let push_id_2 = { thread_rng().gen::<i32>() };

        // let player_id = { thread_rng().gen::<i32>() };
        let player_id = 0x10123;

        log::info!("Sending test join. player_id = {}. identifier = {:?}", player_id, sign);

        PushParams::Join(fnrpc::push::JoinParams {
            identifier: fnrpc::shared::ObjectIdentifier {
                object_id: push_id_1,
                secondary_id: push_id_2,
            },
            join_payload: JoinPayload::SummonSign(SummonSignParams {
                summoning_player_id: player_id,
                steam_id: String::new(),
                summoned_player_id: sign.secondary_id,
                sign_identifier: sign,
                join_data: Vec::from([
                    0x38, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, 
                    0x52, 0xB8, 0x6E, 0x41, 0x73, 0xE8, 0x3F, 0x42, 0x0A, 0xD7, 0x45, 0x42, 0x1A, 0x5D, 0xF1, 0xBE, 
                    0x02, 0x00, 0x00, 0x00, 0xFD, 0xFF, 0xFF, 0xFF, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 
                    0x08, 0x00, 0x00, 0x00, 0x55, 0xE7, 0x7E, 0xAC, 0x00, 0x00, 0x86, 0x01, 
                ]),
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
