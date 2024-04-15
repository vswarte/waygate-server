use sqlx::Row;

use fnrpc::shared::*;
use fnrpc::ghostdata::*;
use fnrpc::ResponseParams;

use crate::database;
use crate::database::pool;
use crate::rpc;
use crate::session::ClientSession;

pub async fn handle_create_ghostdata(
    session: ClientSession,
    params: RequestCreateGhostDataParams,
) -> rpc::HandlerResult {
    let pool = pool().await?;
    let ghostdata_id = sqlx::query("INSERT INTO ghostdata (
            player_id,
            session_id,
            replay_data,
            map,
            play_region
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5
        ) RETURNING ghostdata_id")
        .bind(session.player_id)
        .bind(session.session_id)
        .bind(params.replay_data)
        .bind(params.area.map)
        .bind(params.area.play_region)
        .fetch_one(&pool)
        .await?
        .get("ghostdata_id");

    Ok(ResponseParams::CreateBloodstain(
        ResponseCreateGhostDataParams {
            object_id: ghostdata_id,
            secondary_id: session.session_id,
        }
    ))
}

pub async fn handle_get_ghostdata_list(
    params: RequestGetGhostDataListParams
) -> rpc::HandlerResult {
    let play_regions = params.search_areas.iter()
        .map(|a| a.play_region)
        .collect::<Vec<i32>>();

    let pool = pool().await?;
    let mut entries = sqlx::query_as::<_, GhostData>("SELECT * FROM ghostdata WHERE play_region = ANY($1) ORDER BY random() LIMIT 64")
        .bind(play_regions)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

    Ok(ResponseParams::GetGhostDataList(
        ResponseGetGhostDataListParams { entries }
    ))
}

#[derive(sqlx::FromRow)]
struct GhostData {
    ghostdata_id: i32,
    player_id: i32,
    session_id: i32,
    replay_data: Vec<u8>,
    map: i32,
    play_region: i32,
}

impl Into<ResponseGetGhostDataListParamsEntry> for GhostData {
    fn into(self) -> ResponseGetGhostDataListParamsEntry {
        ResponseGetGhostDataListParamsEntry {
            area: OnlineArea {
                map: self.map,
                play_region: self.play_region,
            },
            identifier: ObjectIdentifier {
                object_id: self.ghostdata_id,
                secondary_id: self.session_id,
            },
            replay_data: self.replay_data,
            group_passwords: vec![],
        }
    }
}
