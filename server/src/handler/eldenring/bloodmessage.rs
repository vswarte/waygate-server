use sqlx::Row;
use thiserror::Error;

use message::eldenring::{
    ObjectIdentifier, PlayRegionArea, RequestCreateBloodMessageParams,
    RequestEvaluateBloodMessageParams, RequestGetBloodMessageListParams,
    RequestReentryBloodMessageParams, RequestRemoveBloodMessageParams,
    ResponseCreateBloodMessageParams, ResponseEvaluateBloodMessageParams,
    ResponseGetBloodMessageListParams, ResponseGetBloodMessageListParamsEntry,
    ResponseReentryBloodMessageParams, ResponseRemoveBloodMessageParams,
};

use super::DefaultClientHandler;
use crate::handler::HandleRequest;

impl HandleRequest<Box<RequestCreateBloodMessageParams>, ResponseCreateBloodMessageParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateBloodMessageParams>,
    ) -> Result<ResponseCreateBloodMessageParams, Box<dyn std::error::Error>> {
        let bloodmessage_id = sqlx::query(
            "INSERT INTO bloodmessages (
            player_id,
            character_id,
            session_id,
            rating_good,
            rating_bad,
            data,
            area,
            play_region,
            group_passwords
        ) VALUES (
            $1,
            $2,
            $3,
            0,
            0,
            $4,
            $5,
            $6,
            $7
        ) RETURNING bloodmessage_id",
        )
        .bind(self.session.player_id)
        .bind(request.character_id)
        .bind(self.session.session_id)
        .bind(&request.data)
        .bind(request.area.area)
        .bind(request.area.play_region)
        .bind(&request.group_passwords)
        .fetch_one(&self.services.database)
        .await?
        .get("bloodmessage_id");

        Ok(ResponseCreateBloodMessageParams {
            identifier: ObjectIdentifier(bloodmessage_id),
        })
    }
}

impl HandleRequest<Box<RequestGetBloodMessageListParams>, ResponseGetBloodMessageListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestGetBloodMessageListParams>,
    ) -> Result<ResponseGetBloodMessageListParams, Box<dyn std::error::Error>> {
        let play_regions = request
            .search_areas
            .iter()
            .map(|a| a.play_region)
            .collect::<Vec<i32>>();

        let entries = sqlx::query_as::<_, BloodMessageRecord>(
            "SELECT * FROM bloodmessages WHERE play_region = ANY($1) ORDER BY random() LIMIT 64",
        )
        .bind(play_regions)
        .fetch_all(&self.services.database)
        .await?
        .into_iter()
        .map(|e| ResponseGetBloodMessageListParamsEntry {
            player_id: e.player_id,
            character_id: e.character_id,
            identifier: ObjectIdentifier(e.bloodmessage_id),
            rating_good: e.rating_good,
            rating_bad: e.rating_bad,
            data: e.data,
            area: PlayRegionArea {
                play_region: e.play_region,
                area: e.area,
            },
            group_passwords: e.group_passwords,
        })
        .collect();

        Ok(ResponseGetBloodMessageListParams { entries })
    }
}

impl HandleRequest<Box<RequestEvaluateBloodMessageParams>, ResponseEvaluateBloodMessageParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestEvaluateBloodMessageParams>,
    ) -> Result<ResponseEvaluateBloodMessageParams, Box<dyn std::error::Error>> {
        let query = match request.rating.try_into()? {
            BloodMessageRating::Good => sqlx::query(
                "UPDATE bloodmessages SET rating_good = rating_good + 1 WHERE bloodmessage_id = $1",
            ),
            BloodMessageRating::Bad => sqlx::query(
                "UPDATE bloodmessages SET rating_bad = rating_bad + 1 WHERE bloodmessage_id = $1",
            ),
        };

        query
            .bind(request.identifier.0)
            .execute(&self.services.database)
            .await?;

        Ok(ResponseEvaluateBloodMessageParams {})
    }
}

impl HandleRequest<Box<RequestRemoveBloodMessageParams>, ResponseRemoveBloodMessageParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestRemoveBloodMessageParams>,
    ) -> Result<ResponseRemoveBloodMessageParams, Box<dyn std::error::Error>> {
        sqlx::query("DELETE FROM bloodmessages WHERE bloodmessage_id = $1 AND player_id = $2")
            .bind(request.identifier.0)
            .bind(self.session.player_id)
            .execute(&self.services.database)
            .await?;

        Ok(ResponseRemoveBloodMessageParams {})
    }
}

impl HandleRequest<Box<RequestReentryBloodMessageParams>, ResponseReentryBloodMessageParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestReentryBloodMessageParams>,
    ) -> Result<ResponseReentryBloodMessageParams, Box<dyn std::error::Error>> {
        let bloodmessages = request
            .identifiers
            .iter()
            .map(|a| a.0)
            .collect::<Vec<i64>>();

        let identifiers = sqlx::query_as::<_, BloodMessageRecord>(
            "SELECT * FROM bloodmessages WHERE bloodmessage_id = ANY($1)",
        )
        .bind(bloodmessages)
        .fetch_all(&self.services.database)
        .await?
        .into_iter()
        .map(|e| ObjectIdentifier(e.bloodmessage_id))
        .collect();

        Ok(ResponseReentryBloodMessageParams { identifiers })
    }
}

#[derive(sqlx::FromRow)]
struct BloodMessageRecord {
    bloodmessage_id: i64,
    player_id: i32,
    character_id: i32,
    rating_good: i32,
    rating_bad: i32,
    data: Vec<u8>,
    area: i32,
    play_region: i32,
    group_passwords: Vec<String>,
}

#[derive(Debug)]
pub enum BloodMessageRating {
    Good,
    Bad,
}

#[derive(Debug, Error)]
pub enum BloodMessageRatingError {
    #[error("Unknown rating value from write")]
    UnknownRatingValue,
}

impl TryFrom<u32> for BloodMessageRating {
    type Error = BloodMessageRatingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Good,
            1 => Self::Bad,
            _ => return Err(BloodMessageRatingError::UnknownRatingValue),
        })
    }
}
