use message::eldenring::{
    RequestGrGetPlayerEquipmentsParams, RequestGrUploadPlayerEquipmentsParams,
    ResponseGrGetPlayerEquipmentsParams, ResponseGrGetPlayerEquipmentsParamsEntry,
    ResponseGrUploadPlayerEquipmentsParams,
};

use crate::handler::HandleRequest;

use super::DefaultClientHandler;

const INSERT_QUERY: &str = "
    INSERT INTO player_equipments (
        player_id,
        session_id,
        data,
        pool
    ) VALUES (
        $1,
        $2,
        $3,
        $4
    ) ON CONFLICT (pool, player_id) DO UPDATE SET
        data = EXCLUDED.data,
        session_id = EXCLUDED.session_id,
        rnd = random()";

const SELECT_QUERY: &str = "
    WITH pivot AS (SELECT random() AS r)
    SELECT pe.* FROM player_equipments pe
    JOIN (
        SELECT player_equipments_id FROM (
            SELECT player_equipments_id
            FROM player_equipments, pivot
            WHERE pool = $1 AND rnd >= pivot.r
            ORDER BY rnd
            LIMIT 64
        ) above
        UNION ALL
        SELECT player_equipments_id FROM (
            SELECT player_equipments_id
            FROM player_equipments, pivot
            WHERE pool = $1 AND rnd < pivot.r
            ORDER BY rnd
            LIMIT 64
        ) below
        LIMIT 64
    ) s USING (player_equipments_id)";

impl
    HandleRequest<
        Box<RequestGrUploadPlayerEquipmentsParams>,
        ResponseGrUploadPlayerEquipmentsParams,
    > for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGrUploadPlayerEquipmentsParams>,
    ) -> Result<ResponseGrUploadPlayerEquipmentsParams, Box<dyn std::error::Error>> {
        sqlx::query(INSERT_QUERY)
            .bind(self.session.player_id)
            .bind(self.session.session_id)
            .bind(&request.data)
            .bind(request.pool)
            .execute(&self.services.database)
            .await?;

        Ok(ResponseGrUploadPlayerEquipmentsParams {})
    }
}

impl HandleRequest<Box<RequestGrGetPlayerEquipmentsParams>, ResponseGrGetPlayerEquipmentsParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGrGetPlayerEquipmentsParams>,
    ) -> Result<ResponseGrGetPlayerEquipmentsParams, Box<dyn std::error::Error>> {
        let entries = sqlx::query_as::<_, PlayerEquipmentsRecord>(SELECT_QUERY)
            .bind(request.pool)
            .fetch_all(&self.services.database)
            .await?
            .into_iter()
            .map(|e| ResponseGrGetPlayerEquipmentsParamsEntry {
                entry_id: e.player_equipments_id as i32,
                data: e.data,
            })
            .take(request.count as usize)
            .collect();

        Ok(ResponseGrGetPlayerEquipmentsParams { entries })
    }
}

#[derive(sqlx::FromRow)]
struct PlayerEquipmentsRecord {
    player_equipments_id: i64,
    data: Vec<u8>,
}
