use rand::Rng;
use sqlx::Row;

use message::{
    builder::MessageBuilder,
    eldenring::{
        BloodMessageRating, EvaluateBloodMessageParams, JoinParams, JoinPayload, ObjectIdentifier,
        PlayRegionArea, PushParams, RequestCreateBloodMessageParams,
        RequestEvaluateBloodMessageParams, RequestGetBloodMessageListParams,
        RequestReentryBloodMessageParams, RequestRemoveBloodMessageParams,
        ResponseCreateBloodMessageParams, ResponseEvaluateBloodMessageParams,
        ResponseGetBloodMessageListParams, ResponseGetBloodMessageListParamsEntry,
        ResponseReentryBloodMessageParams, ResponseRemoveBloodMessageParams,
    },
};

use super::DefaultClientHandler;
use crate::handler::HandleRequest;

const INSERT_QUERY: &str = "
    INSERT INTO bloodmessages (
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
    ) RETURNING bloodmessage_id";

const SELECT_QUERY: &str = "
    WITH pivot AS (SELECT random() AS r)
    SELECT b.*
    FROM bloodmessages b
    JOIN (
        SELECT bloodmessage_id FROM (
            SELECT bloodmessage_id
            FROM bloodmessages, pivot
            WHERE play_region = ANY($1) AND rnd >= pivot.r
            ORDER BY rnd
            LIMIT 64
        ) above
        UNION ALL
        SELECT bloodmessage_id FROM (
            SELECT bloodmessage_id
            FROM bloodmessages, pivot
            WHERE play_region = ANY($1) AND rnd < pivot.r
            ORDER BY rnd
            LIMIT 64
        ) below
        LIMIT 64
    ) s USING (bloodmessage_id)";

impl HandleRequest<Box<RequestCreateBloodMessageParams>, ResponseCreateBloodMessageParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        request: &Box<RequestCreateBloodMessageParams>,
    ) -> Result<ResponseCreateBloodMessageParams, Box<dyn std::error::Error>> {
        let bloodmessage_id = sqlx::query(INSERT_QUERY)
            .bind(self.session.player_id)
            .bind(request.character_id)
            .bind(self.session.session_id)
            .bind(&request.data)
            .bind(request.area.area as i32)
            .bind(request.area.play_region as i32)
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
            .map(|a| a.play_region as i32)
            .collect::<Vec<i32>>();

        let entries = sqlx::query_as::<_, BloodMessageRecord>(SELECT_QUERY)
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
                    play_region: e.play_region as u32,
                    area: e.area as u32,
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
        let rating = request.rating.try_into()?;
        let query = match rating {
            BloodMessageRating::Good => sqlx::query(
                "UPDATE bloodmessages SET rating_good = rating_good + 1 WHERE bloodmessage_id = $1 RETURNING player_id",
            ),
            BloodMessageRating::Bad => sqlx::query(
                "UPDATE bloodmessages SET rating_bad = rating_bad + 1 WHERE bloodmessage_id = $1 RETURNING player_id",
            ),
        };

        let player_id: i32 = query
            .bind(request.identifier.0)
            .fetch_one(&self.services.database)
            .await?
            .get("player_id");

        let message = MessageBuilder::push()
            .body(PushParams::Join(JoinParams {
                identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                join_payload: JoinPayload::EvaluateBloodMessage(EvaluateBloodMessageParams {
                    unk1: 0,
                    sign_identifier: request.identifier,
                    evaluation: rating,
                }),
            }))
            .build()
            .expect("Could not build push message");

        self.services
            .notifications
            .notify_player(player_id, message)?;

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
