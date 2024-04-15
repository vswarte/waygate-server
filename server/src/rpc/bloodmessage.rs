use sqlx::Row;
use fnrpc::bloodmessage::*;
use fnrpc::shared::*;
use fnrpc::ResponseParams;
use thiserror::Error;

use crate::database::pool;
use crate::rpc;
use crate::session::ClientSession;

pub async fn handle_create_blood_message(
    session: ClientSession,
    params: RequestCreateBloodMessageParams,
) -> rpc::HandlerResult {
    let pool = pool().await?;
    let bloodmessage_id = sqlx::query("INSERT INTO bloodmessages (
            player_id,
            character_id,
            session_id,
            rating_good,
            rating_bad,
            data,
            map,
            play_region
        ) VALUES (
            $1,
            $2,
            $3,
            0,
            0,
            $4,
            $5,
            $6
        ) RETURNING bloodmessage_id")
        .bind(session.player_id)
        .bind(params.character_id)
        .bind(session.session_id)
        .bind(params.data)
        .bind(params.area.map)
        .bind(params.area.play_region)
        .fetch_one(&pool)
        .await?
        .get("bloodmessage_id");

    Ok(ResponseParams::CreateBloodMessage(
        ResponseCreateBloodMessageParams {
            object_id: bloodmessage_id,
            secondary_id: session.session_id,
        },
    ))
}

pub async fn handle_get_blood_message_list(
    params: RequestGetBloodMessageListParams,
) -> rpc::HandlerResult {
    let play_regions = params.search_areas.iter()
        .map(|a| a.play_region)
        .collect::<Vec<i32>>();

    let pool = pool().await?;
    let mut entries = sqlx::query_as::<_, BloodMessage>("SELECT * FROM bloodmessages WHERE play_region = ANY($1) ORDER BY random() LIMIT 64")
        .bind(play_regions)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

    Ok(ResponseParams::GetBloodMessageList(
        ResponseGetBloodMessageListParams { entries }
    ))
}

pub async fn handle_evaluate_blood_message(
    params: RequestEvaluateBloodMessageParams,
) -> rpc::HandlerResult {
    let pool = pool().await?;

    let query = match params.rating.try_into()? {
        BloodMessageRating::Good => {
            sqlx::query("UPDATE bloodmessages SET rating_good = rating_good + 1 WHERE bloodmessage_id = $1")
        },
        BloodMessageRating::Bad => {
            sqlx::query("UPDATE bloodmessages SET rating_bad = rating_bad + 1 WHERE bloodmessage_id = $1")
        },
    };

    query.bind(params.identifier.object_id)
        .execute(&pool)
        .await?;

    Ok(ResponseParams::EvaluateBloodMessage)
}

#[derive(sqlx::FromRow)]
struct BloodMessage {
    bloodmessage_id: i32,
    player_id: i32,
    character_id: i32,
    session_id: i32,
    rating_good: i32,
    rating_bad: i32,
    data: Vec<u8>,
    map: i32,
    play_region: i32,
}

impl Into<ResponseGetBloodMessageListParamsEntry> for BloodMessage {
    fn into(self) -> ResponseGetBloodMessageListParamsEntry {
        ResponseGetBloodMessageListParamsEntry {
            player_id: self.player_id,
            character_id: self.character_id,
            identifier: ObjectIdentifier {
                object_id: self.bloodmessage_id,
                secondary_id: self.session_id,
            },
            rating_good: self.rating_good,
            rating_bad: self.rating_bad,
            data: self.data,
            area: OnlineArea {
                map: self.map,
                play_region: self.play_region,
            },
            group_passwords: vec![],
        }
    }
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
