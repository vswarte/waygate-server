use message::eldenring::{
    ObjectIdentifier, PlayRegionArea, RequestCreateGhostDataParams, RequestGetGhostDataListParams,
    ResponseCreateGhostDataParams, ResponseGetGhostDataListParams,
    ResponseGetGhostDataListParamsEntry,
};
use sqlx::Row;

use crate::handler::HandleRequest;

use super::DefaultClientHandler;

const INSERT_QUERY: &str = "
    INSERT INTO ghostdata (
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
    ) RETURNING ghostdata_id";

const SELECT_QUERY: &str = "
    WITH pivot AS (SELECT random() AS r),
         regions AS (SELECT unnest($1::int[]) AS region)
    SELECT g.*
    FROM ghostdata g
    JOIN (
        SELECT ghostdata_id FROM (
            SELECT ghostdata_id
            FROM regions
            CROSS JOIN LATERAL (
                SELECT ghostdata_id, rnd
                FROM ghostdata
                WHERE play_region = regions.region AND rnd >= (SELECT r FROM pivot)
                ORDER BY rnd
                LIMIT 64
            ) lat_above
            ORDER BY rnd
            LIMIT 64
        ) above
        UNION ALL
        SELECT ghostdata_id FROM (
            SELECT ghostdata_id
            FROM regions
            CROSS JOIN LATERAL (
                SELECT ghostdata_id, rnd
                FROM ghostdata
                WHERE play_region = regions.region AND rnd < (SELECT r FROM pivot)
                ORDER BY rnd
                LIMIT 64
            ) lat_below
            ORDER BY rnd
            LIMIT 64
        ) below
        LIMIT 64
    ) s USING (ghostdata_id)";

impl HandleRequest<Box<RequestCreateGhostDataParams>, ResponseCreateGhostDataParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateGhostDataParams>,
    ) -> Result<ResponseCreateGhostDataParams, Box<dyn std::error::Error>> {
        let ghostdata_id = sqlx::query(INSERT_QUERY)
            .bind(100)
            .bind(100)
            .bind(&request.replay_data)
            .bind(request.area.area as i32)
            .bind(request.area.play_region as i32)
            .bind(&request.group_passwords)
            .fetch_one(&self.services.database)
            .await?
            .get("ghostdata_id");

        Ok(ResponseCreateGhostDataParams {
            identifier: ObjectIdentifier(ghostdata_id),
        })
    }
}

impl HandleRequest<Box<RequestGetGhostDataListParams>, ResponseGetGhostDataListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetGhostDataListParams>,
    ) -> Result<ResponseGetGhostDataListParams, Box<dyn std::error::Error>> {
        let play_regions = request
            .search_areas
            .iter()
            .map(|a| a.play_region as i32)
            .collect::<Vec<i32>>();

        let entries: Vec<ResponseGetGhostDataListParamsEntry> =
            sqlx::query_as::<_, GhostDataRecord>(SELECT_QUERY)
                .bind(play_regions)
                .fetch_all(&self.services.database)
                .await?
                .into_iter()
                .map(|e| ResponseGetGhostDataListParamsEntry {
                    area: PlayRegionArea {
                        play_region: e.play_region as u32,
                        area: e.area as u32,
                    },
                    identifier: ObjectIdentifier(e.ghostdata_id),
                    replay_data: e.replay_data,
                    group_passwords: e.group_passwords,
                })
                .collect();

        Ok(ResponseGetGhostDataListParams { entries })
    }
}

#[derive(sqlx::FromRow)]
struct GhostDataRecord {
    ghostdata_id: i64,
    replay_data: Vec<u8>,
    area: i32,
    play_region: i32,
    group_passwords: Vec<String>,
}
