use sqlx::Row;

use fnrpc::ResponseParams;
use fnrpc::bloodstain::*;
use fnrpc::shared::ObjectIdentifier;
use fnrpc::shared::OnlineArea;

use crate::database::pool;
use crate::rpc;
use crate::session::ClientSession;

pub async fn handle_create_bloodstain(
    session: ClientSession,
    params: RequestCreateBloodstainParams,
) -> rpc::HandlerResult {
    let pool = pool().await?;
    let bloodstain_id = sqlx::query("INSERT INTO bloodstains (
            player_id,
            session_id,
            advertisement_data,
            replay_data,
            area,
            play_region
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6
        ) RETURNING bloodstain_id")
        .bind(session.player_id)
        .bind(session.session_id)
        .bind(params.advertisement_data)
        .bind(params.replay_data)
        .bind(params.area.area)
        .bind(params.area.play_region)
        .fetch_one(&pool)
        .await?
        .get("bloodstain_id");

    Ok(ResponseParams::CreateBloodstain(
        ResponseCreateBloodstainParams {
            object_id: bloodstain_id,
            secondary_id: session.session_id,
        }
    ))
}

pub async fn handle_get_bloodstain_list(
    params: RequestGetBloodstainListParams,
) -> rpc::HandlerResult {
    let play_regions = params.search_areas.iter()
        .map(|a| a.play_region)
        .collect::<Vec<i32>>();

    let pool = pool().await?;
    let mut entries = sqlx::query_as::<_, Bloodstain>("SELECT * FROM bloodstains WHERE play_region = ANY($1) ORDER BY random() LIMIT 64")
        .bind(play_regions)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

    Ok(ResponseParams::GetBloodstainList(
        ResponseGetBloodstainListParams { entries }
    ))
}

pub async fn handle_get_deading_ghost<'a>(
    params: RequestGetDeadingGhostParams,
) -> rpc::HandlerResult {
    let pool = pool().await?;
    let bloodstain = sqlx::query_as::<_, Bloodstain>("SELECT * FROM bloodstains WHERE bloodstain_id = $1")
        .bind(params.identifier.object_id)
        .fetch_one(&pool)
        .await?;

    Ok(ResponseParams::GetDeadingGhost(
        ResponseGetDeadingGhostParams {
            unk0: 0,
            unk4: 0,
            identifier: ObjectIdentifier {
                object_id: bloodstain.bloodstain_id,
                secondary_id: bloodstain.session_id,
            },
            replay_data: bloodstain.replay_data,
        }
    ))
}

#[derive(sqlx::FromRow)]
struct Bloodstain {
    bloodstain_id: i32,
    player_id: i32,
    session_id: i32,
    advertisement_data: Vec<u8>,
    replay_data: Vec<u8>,
    area: i32,
    play_region: i32,
}

impl Into<ResponseGetBloodstainListParamsEntry> for Bloodstain {
    fn into(self) -> ResponseGetBloodstainListParamsEntry {
        ResponseGetBloodstainListParamsEntry {
            area: OnlineArea {
                area: self.area,
                play_region: self.play_region,
            },
            identifier: ObjectIdentifier {
                object_id: self.bloodstain_id,
                secondary_id: self.session_id,
            },
            advertisement_data: self.advertisement_data,
            group_passwords: vec![],
        }
    }
}

impl Into<ResponseGetDeadingGhostParams> for Bloodstain {
    fn into(self) -> ResponseGetDeadingGhostParams {
        ResponseGetDeadingGhostParams {
            unk0: 0,
            unk4: 0,
            identifier: ObjectIdentifier {
                object_id: self.bloodstain_id,
                secondary_id: self.session_id,
            },
            replay_data: self.replay_data,
        }
    }
}
