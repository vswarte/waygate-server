use message::eldenring::{
    ObjectIdentifier, PlayRegionArea, RequestCreateBloodstainParams,
    RequestGetBloodstainListParams, RequestGetDeadingGhostParams, ResponseCreateBloodstainParams,
    ResponseGetBloodstainListParams, ResponseGetBloodstainListParamsEntry,
    ResponseGetDeadingGhostParams,
};
use sqlx::Row;

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
    WITH pivot AS (SELECT random() AS r),
         regions AS (SELECT unnest($1::int[]) AS region)
    SELECT b.*
    FROM bloodstains b
    JOIN (
        SELECT bloodstain_id FROM (
            SELECT bloodstain_id
            FROM regions
            CROSS JOIN LATERAL (
                SELECT bloodstain_id, rnd
                FROM bloodstains
                WHERE play_region = regions.region AND rnd >= (SELECT r FROM pivot)
                ORDER BY rnd
                LIMIT 64
            ) lat_above
            ORDER BY rnd
            LIMIT 64
        ) above
        UNION ALL
        SELECT bloodstain_id FROM (
            SELECT bloodstain_id
            FROM regions
            CROSS JOIN LATERAL (
                SELECT bloodstain_id, rnd
                FROM bloodstains
                WHERE play_region = regions.region AND rnd < (SELECT r FROM pivot)
                ORDER BY rnd
                LIMIT 64
            ) lat_below
            ORDER BY rnd
            LIMIT 64
        ) below
        LIMIT 64
    ) s USING (bloodstain_id)";

const SELECT_BY_ID_QUERY: &str = "
    SELECT *
    FROM bloodstains
    WHERE bloodstain_id = $1";

impl HandleRequest<Box<RequestCreateBloodstainParams>, ResponseCreateBloodstainParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateBloodstainParams>,
    ) -> Result<ResponseCreateBloodstainParams, Box<dyn std::error::Error>> {
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
