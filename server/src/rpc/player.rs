use std::borrow::BorrowMut;

use fnrpc::player::RequestUpdatePlayerStatusParams;
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

const UPDATE_LISTENERS: [UpdateListener; 0] = [
    // update_break_in_status,
];

pub async fn handle_update_player_status(
    session: ClientSession,
    params: RequestUpdatePlayerStatusParams
) -> rpc::HandlerResult {
    if params.character.group_passwords.iter().any(|e| e == "freebuff") {
        push::send_push(session.player_id, get_test_buff()).await.unwrap();
    }

    if params.character.group_passwords.iter().any(|e| e == "announce") {
        push::send_push(session.player_id, get_test_announcement()).await.unwrap();
    }

    // let mut status_map = STATUS_MAP.get_or_init(Default::default)
    //     .lock()
    //     .expect("Could not acquire lock for status map");
    //
    // let previous_params = status_map.insert(session.player_id, params.clone());
    for listener in UPDATE_LISTENERS.iter() {
        // listener(&session, &params, previous_params.as_ref());
        listener(session.clone(), &params);
    }

    Ok(ResponseParams::UpdatePlayerStatus)
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
            message: "Test announcement: I ate too much cheese".to_string(),
            unk1: 0x0,
            unk2: 0x0,
            unk3: 0x0,
        },
    })
}

// TODO: make this less cursed
fn update_break_in_status(
    session: ClientSession,
    status: &RequestUpdatePlayerStatusParams,
    _previous: Option<&RequestUpdatePlayerStatusParams>,
) {
    // let invasion_pool = invasion_pool_mut()
    //     .unwrap()
    //
    // match &session.invasion_pool_token {
    //     Some(_) => {
    //         if status.character.online_activity == 0x0 {
    //             // Drop entry from pool
    //             let _ = session.invasion_pool_token.take();
    //         } else {
    //
    //         }
    //     },
    //     None => {
    //         if status.character.online_activity != 0x0 {
    //             let token = invasion_pool.insert(
    //                 session.player_id as u32,
    //                 status.into(),
    //             );
    //         }
    //     },
    // }
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
