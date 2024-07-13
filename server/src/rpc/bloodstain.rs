use sqlx::Row;

use fnrpc::ResponseParams;
use fnrpc::bloodstain::*;
use fnrpc::shared::ObjectIdentifier;
use fnrpc::shared::PlayRegionArea;

use crate::database;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

pub async fn handle_create_bloodstain(
    session: ClientSession,
    params: RequestCreateBloodstainParams,
) -> rpc::HandlerResult {
    let (player_id, session_id) = {
        let lock = session.lock_read();
        (lock.player_id, lock.session_id)
    };

    let mut connection = database::acquire().await?;
    let bloodstain_id = sqlx::query("INSERT INTO bloodstains (
            player_id,
            session_id,
            advertisement_data,
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
            $6,
            $7
        ) RETURNING bloodstain_id")
        .bind(player_id)
        .bind(session_id)
        .bind(params.advertisement_data)
        .bind(params.replay_data)
        .bind(params.area.area)
        .bind(params.area.play_region)
        .bind(params.group_passwords)
        .fetch_one(&mut *connection)
        .await?
        .get("bloodstain_id");

    Ok(ResponseParams::CreateBloodstain(
        ResponseCreateBloodstainParams {
            object_id: bloodstain_id,
            secondary_id: session_id,
        }
    ))
}

pub async fn handle_get_bloodstain_list(
    params: RequestGetBloodstainListParams,
) -> rpc::HandlerResult {
    let play_regions = params.search_areas.iter()
        .map(|a| a.play_region)
        .collect::<Vec<i32>>();

    let mut connection = database::acquire().await?;
    let entries = sqlx::query_as::<_, Bloodstain>("SELECT * FROM bloodstains WHERE play_region = ANY($1) ORDER BY random() LIMIT 64")
        .bind(play_regions)
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

    Ok(ResponseParams::GetBloodstainList(
        ResponseGetBloodstainListParams { entries }
    ))
}

pub async fn handle_get_deading_ghost(
    params: RequestGetDeadingGhostParams,
) -> rpc::HandlerResult {
    log::info!("GetDeadingGhost: {:?}", params);

    let mut connection = database::acquire().await?;
    let bloodstain = sqlx::query_as::<_, Bloodstain>("SELECT * FROM bloodstains WHERE bloodstain_id = $1")
        .bind(params.identifier.object_id)
        .fetch_one(&mut *connection)
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
    session_id: i32,
    advertisement_data: Vec<u8>,
    replay_data: Vec<u8>,
    area: i32,
    play_region: i32,
    group_passwords: Vec<String>,
}

impl From<Bloodstain> for ResponseGetBloodstainListParamsEntry {
    fn from(val: Bloodstain) -> Self {
        ResponseGetBloodstainListParamsEntry {
            area: PlayRegionArea {
                area: val.area,
                play_region: val.play_region,
            },
            identifier: ObjectIdentifier {
                object_id: val.bloodstain_id,
                secondary_id: val.session_id,
            },
            advertisement_data: val.advertisement_data,
            group_passwords: val.group_passwords,
        }
    }
}

impl From<Bloodstain> for ResponseGetDeadingGhostParams {
    fn from(val: Bloodstain) -> Self {
        ResponseGetDeadingGhostParams {
            unk0: 0,
            unk4: 0,
            identifier: ObjectIdentifier {
                object_id: val.bloodstain_id,
                secondary_id: val.session_id,
            },
            replay_data: val.replay_data,
        }
    }
}
