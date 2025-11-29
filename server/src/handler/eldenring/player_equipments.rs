use message::eldenring::{
    RequestGrGetPlayerEquipmentsParams, RequestGrUploadPlayerEquipmentsParams,
    ResponseGrGetPlayerEquipmentsParams, ResponseGrGetPlayerEquipmentsParamsEntry,
    ResponseGrUploadPlayerEquipmentsParams,
};

use crate::handler::HandleRequest;

use super::DefaultClientHandler;

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
        sqlx::query(
            "INSERT INTO player_equipments (
                player_id,
                session_id,
                data,
                pool
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            )",
        )
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
        let entries = sqlx::query_as::<_, PlayerEquipmentsRecord>(
            "
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
        ",
        )
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
