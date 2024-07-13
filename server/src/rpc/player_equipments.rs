use fnrpc::player_equipments::*;
use fnrpc::ResponseParams;

use crate::database;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

pub async fn handle_gr_get_player_equipments(
    request: RequestGrGetPlayerEquipmentsParams,
) -> rpc::HandlerResult {
    let mut connection = database::acquire().await?;
    let entries = sqlx::query_as::<_, PlayerEquipments>("
        SELECT
            distinct on (player_id) player_id, *
        FROM 
            player_equipments
        WHERE 
            pool = $1
        ORDER BY
            player_id,
            player_equipments_id DESC
        LIMIT 64
    ")
        .bind(request.pool)
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|e| e.into())
        .take(request.count as usize)
        .collect();

    Ok(ResponseParams::GrGetPlayerEquipments(ResponseGrGetPlayerEquipmentsParams {
        entries,
    }))
}

pub async fn handle_gr_upload_player_equipments(
    session: ClientSession,
    request: RequestGrUploadPlayerEquipmentsParams,
) -> rpc::HandlerResult {
    let (player_id, session_id) = {
        let lock = session.lock_read();
        (lock.player_id, lock.session_id)
    };

    let mut connection = database::acquire().await?;
    sqlx::query("INSERT INTO player_equipments (
            player_id,
            session_id,
            data,
            pool
        ) VALUES (
            $1,
            $2,
            $3,
            $4
        )")
        .bind(player_id)
        .bind(session_id)
        .bind(request.data)
        .bind(request.pool)
        .execute(&mut *connection)
        .await?;

    Ok(ResponseParams::GrUploadPlayerEquipments)
}

#[derive(sqlx::FromRow)]
struct PlayerEquipments {
    player_equipments_id: i32,
    data: Vec<u8>,
}

impl From<PlayerEquipments> for ResponseGrGetPlayerEquipmentsParamsEntry {
    fn from(val: PlayerEquipments) -> Self {
        ResponseGrGetPlayerEquipmentsParamsEntry {
            entry_id: val.player_equipments_id,
            data: val.data,
        }
    }
}
