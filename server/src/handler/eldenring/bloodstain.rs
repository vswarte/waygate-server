use message::eldenring::{
    BloodstainAdvertisementData, ObjectIdentifier, PlayRegionArea, Position,
    RequestCreateBloodstainParams, RequestGetBloodstainListParams, RequestGetDeadingGhostParams,
    ResponseCreateBloodstainParams, ResponseGetBloodstainListParams,
    ResponseGetBloodstainListParamsEntry, ResponseGetDeadingGhostParams,
};
use sqlx::Row;
use wire::deserialize;

use crate::handler::HandleRequest;

use super::DefaultClientHandler;

const INSERT_QUERY: &str = "
    INSERT INTO bloodstains (
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
    ) RETURNING bloodstain_id";

const SELECT_QUERY: &str = "
    WITH pivot AS (SELECT random() AS r)
    SELECT b.*
    FROM bloodstains b
    JOIN (
        SELECT bloodstain_id FROM (
            SELECT bloodstain_id
            FROM bloodstains, pivot
            WHERE play_region = ANY($1) AND rnd >= pivot.r
            ORDER BY rnd
            LIMIT 64
        ) above
        UNION ALL
        SELECT bloodstain_id FROM (
            SELECT bloodstain_id
            FROM bloodstains, pivot
            WHERE play_region = ANY($1) AND rnd < pivot.r
            ORDER BY rnd
            LIMIT 64
        ) below
        LIMIT 64
    ) s USING (bloodstain_id)";

const SELECT_BY_ID_QUERY: &str = "
    SELECT *
    FROM bloodstains
    WHERE bloodstain_id = $1";

const ROUNDTABLE_PLAY_REGION_ID: i32 = 1110000;
/// BBOX defining the PVP-enabled area within the Roundtable Hold
/// (lower part where player can be invaded by npc invader)
const ROUNDTABLE_PVP_ENABLED_AREA_BBOX: [(f32, f32, f32); 2] =
    [(-325.0, -35.0, -325.0), (-245.0, -25.0, -245.0)];

fn is_in_bbox(pos: Position, bbox: [(f32, f32, f32); 2]) -> bool {
    let min = bbox[0];
    let max = bbox[1];
    pos.x >= min.0
        && pos.x <= max.0
        && pos.y >= min.1
        && pos.y <= max.1
        && pos.z >= min.2
        && pos.z <= max.2
}

impl HandleRequest<Box<RequestCreateBloodstainParams>, ResponseCreateBloodstainParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateBloodstainParams>,
    ) -> Result<ResponseCreateBloodstainParams, Box<dyn std::error::Error>> {
        if request.area.play_region as i32 == ROUNDTABLE_PLAY_REGION_ID {
            // check that location is in the pvp-enabled area of the roundtable
            let adv_data: BloodstainAdvertisementData = deserialize(&request.advertisement_data)?;
            let pos = adv_data.location.position;
            if !is_in_bbox(pos, ROUNDTABLE_PVP_ENABLED_AREA_BBOX) {
                return Ok(ResponseCreateBloodstainParams {
                    identifier: ObjectIdentifier(0),
                });
            }
        }
        let bloodstain_id = sqlx::query(INSERT_QUERY)
            .bind(self.session.player_id)
            .bind(self.session.session_id)
            .bind(&request.advertisement_data)
            .bind(&request.replay_data)
            .bind(request.area.area as i32)
            .bind(request.area.play_region as i32)
            .bind(&request.group_passwords)
            .fetch_one(&self.services.database)
            .await?
            .get("bloodstain_id");

        Ok(ResponseCreateBloodstainParams {
            identifier: ObjectIdentifier(bloodstain_id),
        })
    }
}

impl HandleRequest<Box<RequestGetBloodstainListParams>, ResponseGetBloodstainListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetBloodstainListParams>,
    ) -> Result<ResponseGetBloodstainListParams, Box<dyn std::error::Error>> {
        let play_regions = request
            .search_areas
            .iter()
            .map(|a| a.play_region as i32)
            .collect::<Vec<i32>>();

        if play_regions.is_empty() {
            return Ok(ResponseGetBloodstainListParams { entries: vec![] });
        }

        let entries: Vec<ResponseGetBloodstainListParamsEntry> =
            sqlx::query_as::<_, BloodstainRecord>(SELECT_QUERY)
                .bind(play_regions)
                .fetch_all(&self.services.database)
                .await?
                .into_iter()
                .map(|e| ResponseGetBloodstainListParamsEntry {
                    area: PlayRegionArea {
                        play_region: e.play_region as u32,
                        area: e.area as u32,
                    },
                    identifier: ObjectIdentifier(e.bloodstain_id),
                    advertisement_data: e.advertisement_data,
                    group_passwords: e.group_passwords,
                })
                .collect();

        Ok(ResponseGetBloodstainListParams { entries })
    }
}

impl HandleRequest<Box<RequestGetDeadingGhostParams>, ResponseGetDeadingGhostParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetDeadingGhostParams>,
    ) -> Result<ResponseGetDeadingGhostParams, Box<dyn std::error::Error>> {
        let bloodstain = sqlx::query_as::<_, BloodstainRecord>(SELECT_BY_ID_QUERY)
            .bind(request.identifier.0)
            .fetch_one(&self.services.database)
            .await?;

        Ok(ResponseGetDeadingGhostParams {
            unk0: 0,
            unk4: 0,
            identifier: ObjectIdentifier(bloodstain.bloodstain_id),
            replay_data: bloodstain.replay_data,
        })
    }
}

#[derive(sqlx::FromRow)]
struct BloodstainRecord {
    bloodstain_id: i64,
    advertisement_data: Vec<u8>,
    replay_data: Vec<u8>,
    area: i32,
    play_region: i32,
    group_passwords: Vec<String>,
}
