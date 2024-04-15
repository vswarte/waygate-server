use std::sync;
use std::collections;
use fnrpc::push::PushParams;
use thiserror::Error;
use tokio::sync as tokiosync;

pub type ClientPushChannelRX = tokiosync::mpsc::Receiver<Vec<u8>>;
pub type ClientPushChannelTX = tokiosync::mpsc::Sender<Vec<u8>>;
type ClientPushMap = collections::HashMap<i32, ClientPushChannelTX>;

static CLIENT_PUSH_MAP: sync::OnceLock<tokiosync::RwLock<ClientPushMap>> = sync::OnceLock::new();

pub async fn add_client_channel(player_id: i32, channel: ClientPushChannelTX) {
    CLIENT_PUSH_MAP.get_or_init(|| Default::default())
        .write()
        .await
        .insert(player_id, channel);
}

#[derive(Debug, Error)]
pub enum PushError {
    #[error("Message was pushed for a player that is not online")]
    PlayerNotOnline,

    #[error("Could not send the push message")]
    Send,

    #[error("Could not encode push message")]
    Wire(#[from] fnrpc::FNWireError),
}

pub async fn send_push(
    player_id: i32,
    params: PushParams,
) -> Result<(), PushError> {
    let buffer = {
        let mut buffer: Vec<u8> = vec![
            0x06, // Message type (push)
            0x00, // ???
        ];

        let mut serialized = fnrpc::serialize(params)?;

        buffer.append(&mut serialized);
        buffer
    };

    CLIENT_PUSH_MAP.get_or_init(|| Default::default())
        .write()
        .await
        .get_mut(&player_id)
        .ok_or(PushError::PlayerNotOnline)?
        .send(buffer)
        .await
        .map_err(|_| PushError::Send)?;

    Ok(())
}
