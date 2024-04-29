use fnrpc::player::RequestGetItemLogParams;
use fnrpc::player::RequestJoinMultiplayParams;
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

    {
        let mut session = session.lock_write();
        session.invadeable = session.invadeable && request.character.online_activity == 0x1;
        session.matching = Some((&request).into());

        session.update_invadeability()?;
    }

    debug::listen_debug_notifs(&request, session).await;

    Ok(ResponseParams::UpdatePlayerStatus)
}

impl From<(String, &RequestUpdatePlayerStatusParams)> for BreakInPoolEntry {
    fn from(value: (String, &RequestUpdatePlayerStatusParams)) -> Self {
        Self {
            character_level: value.1.character.level,
            weapon_level: value.1.character.max_reinforce_level,
            steam_id: value.0.to_string(),
        }
    }
}

pub async fn handle_use_item_log(
    session: ClientSession,
    request: RequestUseItemLogParams,
) -> rpc::HandlerResult {
    log::info!("Player sent UseItemLog {:?}", request.used_items);

    // TODO: move this hack somewhere else?
    // Sniff out tongue usage and toggle state. I truly cannot find any other
    // reliable way of checking if this effect is in effect. I checked all
    // networking, endlessly diffed PlayerUpdateStatus and reversed the
    // majority of CreateMatchingTicket (which is getting sent at too low of a
    // frequency anyway for realtime features like this).
    let tongue = request.used_items.iter()
        .find(|i| i.item_id == 108)
        .map(|i| i.times_used % 2 == 1);
    if tongue.is_some_and(|x| x) {
        let mut session = session.lock_write();
        session.invadeable = !session.invadeable;
        session.update_invadeability()?;
    }

    Ok(ResponseParams::UseItemLog)
}

pub async fn handle_join_multiplay(
    _session: ClientSession,
    _request: RequestJoinMultiplayParams,
) -> rpc::HandlerResult {
    log::info!("Player sent JoinMultiplay. player_id = {}", _session.lock_read().player_id);
    Ok(ResponseParams::JoinMultiplay)
}

pub async fn handle_join_muliplay_log(
    _session: ClientSession,
) -> rpc::HandlerResult {
    Ok(ResponseParams::JoinMultiplay)
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
        push::{NotifyParamsSection1, NotifyParamsSection2, PushParams},
    };
    use rand::prelude::*;

    use crate::{push::send_push, session::{ClientSession, ClientSessionContainer}};

    pub async fn listen_debug_notifs(params: &RequestUpdatePlayerStatusParams, session: ClientSession) {
        {
            let player_id = session.lock_read().player_id;
            if params.character.group_passwords.iter().any(|e| e == "_buff") {
                send_push(player_id, get_test_buff()).await.unwrap();
            }

            if params.character.group_passwords.iter().any(|e| e == "_msg") {
                send_push(player_id, get_test_announcement()).await.unwrap();
            }
        }
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
