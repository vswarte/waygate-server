use sqlx::Row;
use fnrpc::bloodmessage::*;
use fnrpc::shared::*;
use fnrpc::ResponseParams;
use thiserror::Error;

use crate::database;
use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

pub async fn handle_create_blood_message(
    session: ClientSession,
    request: RequestCreateBloodMessageParams,
) -> rpc::HandlerResult {
    let (player_id, session_id) = {
        let lock = session.lock_read();
        (lock.player_id, lock.session_id)
    };

    let mut connection = database::acquire().await?;
    let bloodmessage_id = sqlx::query("INSERT INTO bloodmessages (
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
        ) RETURNING bloodmessage_id")
        .bind(player_id)
        .bind(request.character_id)
        .bind(session_id)
        .bind(request.data)
        .bind(request.area.area)
        .bind(request.area.play_region)
        .bind(request.group_passwords)
        .fetch_one(&mut *connection)
        .await?
        .get("bloodmessage_id");

    Ok(ResponseParams::CreateBloodMessage(
        ResponseCreateBloodMessageParams {
            object_id: bloodmessage_id,
            secondary_id: session_id,
        },
    ))
}

pub async fn handle_get_blood_message_list(
    request: RequestGetBloodMessageListParams,
) -> rpc::HandlerResult {
    let play_regions = request.search_areas.iter()
        .map(|a| a.play_region)
        .collect::<Vec<i32>>();

    let mut connection = database::acquire().await?;
    let entries = sqlx::query_as::<_, BloodMessage>("SELECT * FROM bloodmessages WHERE play_region = ANY($1) ORDER BY random() LIMIT 64")
        .bind(play_regions)
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

    Ok(ResponseParams::GetBloodMessageList(
        ResponseGetBloodMessageListParams { entries }
    ))
}

pub async fn handle_evaluate_blood_message(
    request: RequestEvaluateBloodMessageParams,
) -> rpc::HandlerResult {
    let query = match request.rating.try_into()? {
        BloodMessageRating::Good => {
            sqlx::query("UPDATE bloodmessages SET rating_good = rating_good + 1 WHERE bloodmessage_id = $1")
        },
        BloodMessageRating::Bad => {
            sqlx::query("UPDATE bloodmessages SET rating_bad = rating_bad + 1 WHERE bloodmessage_id = $1")
        },
    };

    let mut connection = database::acquire().await?;
    query.bind(request.identifier.object_id)
        .execute(&mut *connection)
        .await?;

    Ok(ResponseParams::EvaluateBloodMessage)
}


pub async fn handle_reentry_blood_message(
    request: RequestReentryBloodMessageParams,
) -> rpc::HandlerResult {
    let bloodmessages = request.identifiers.iter()
        .map(|a| a.object_id)
        .collect::<Vec<i32>>();

    let mut connection = database::acquire().await?;
    let identifiers = sqlx::query_as::<_, BloodMessage>("SELECT * FROM bloodmessages WHERE bloodmessage_id = ANY($1)")
        .bind(bloodmessages)
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|e| ObjectIdentifier {
            object_id: e.bloodmessage_id,
            secondary_id: e.session_id,
        })
        .collect();

    Ok(ResponseParams::ReentryBloodMessage(
        ResponseReentryBloodMessageParams { identifiers }
    ))
}

pub async fn handle_remove_blood_message(
    session: ClientSession,
    request: RequestRemoveBloodMessageParams,
) -> rpc::HandlerResult {
    let player_id = session.lock_read().player_id;

    let mut connection = database::acquire().await?;
    sqlx::query("DELETE FROM bloodmessages WHERE bloodmessage_id = $1 AND player_id = $2")
        .bind(request.identifier.object_id)
        .bind(player_id)
        .execute(&mut *connection)
        .await?;

    Ok(ResponseParams::RemoveBloodMessage)
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
    area: i32,
    play_region: i32,
    group_passwords: Vec<String>,
}

impl From<BloodMessage> for ResponseGetBloodMessageListParamsEntry {
    fn from(val: BloodMessage) -> Self {
        ResponseGetBloodMessageListParamsEntry {
            player_id: val.player_id,
            character_id: val.character_id,
            identifier: ObjectIdentifier {
                object_id: val.bloodmessage_id,
                secondary_id: val.session_id,
            },
            rating_good: val.rating_good,
            rating_bad: val.rating_bad,
            data: val.data,
            area: PlayRegionArea {
                area: val.area,
                play_region: val.play_region,
            },
            group_passwords: val.group_passwords,
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

