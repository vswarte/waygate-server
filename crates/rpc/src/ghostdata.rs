use sqlx::Row;
use waygate_connection::ClientSession;
use waygate_database::database_connection;

use crate::HandlerResult;

use waygate_message::*;

pub async fn handle_create_ghostdata(
    session: ClientSession,
    request: RequestCreateGhostDataParams,
) -> HandlerResult {

    let mut connection = database_connection().await?;
    let ghostdata_id = sqlx::query("INSERT INTO ghostdata (
            player_id,
            session_id,
            replay_data,
            area,
            play_region,
            group_passwords
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6
        ) RETURNING ghostdata_id")
        .bind(session.player_id)
        .bind(session.session_id)
        .bind(request.replay_data)
        .bind(request.area.area)
        .bind(request.area.play_region)
        .bind(request.group_passwords)
        .fetch_one(&mut *connection)
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
    request: RequestGetGhostDataListParams
) -> HandlerResult {
    let play_regions = request.search_areas.iter()
        .map(|a| a.play_region)
        .collect::<Vec<i32>>();

    let mut connection = database_connection().await?;
    let entries = sqlx::query_as::<_, GhostData>("SELECT * FROM ghostdata WHERE play_region = ANY($1) ORDER BY random() LIMIT 64")
        .bind(play_regions)
        .fetch_all(&mut *connection)
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
    session_id: i32,
    replay_data: Vec<u8>,
    area: i32,
    play_region: i32,
    group_passwords: Vec<String>,
}

impl From<GhostData> for ResponseGetGhostDataListParamsEntry {
    fn from(val: GhostData) -> Self {
        ResponseGetGhostDataListParamsEntry {
            area: PlayRegionArea {
                area: val.area,
                play_region: val.play_region,
            },
            identifier: ObjectIdentifier {
                object_id: val.ghostdata_id,
                secondary_id: val.session_id,
            },
            replay_data: val.replay_data,
            group_passwords: val.group_passwords,
        }
    }
}
