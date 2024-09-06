use rand::{thread_rng, Rng};
use tracing::info;
use waygate_connection::ClientSession;
use waygate_database::database_connection;
use waygate_message::armoredcore6::*;

use crate::HandlerResult;

fn generate_ugc_code() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = thread_rng();

    (0..12)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub async fn handle_register_ugc(
    session: ClientSession,
    request: Box<RequestRegisterUGCParams>,
) -> HandlerResult {
    info!("handle_register_ugc {request:?}");
    let ugc_code = generate_ugc_code();

    let mut connection = database_connection().await?;
    sqlx::query("INSERT INTO ugc (ugc_code, steam_id, data) VALUES ($1, $2, $3)")
        .bind(&ugc_code)
        .bind(&session.external_id)
        .bind(request.data)
        .execute(&mut *connection)
        .await?;

    Ok(ResponseParams::RegisterUGC(ResponseRegisterUGCParams {
        ugc_code,
    }))
}

pub async fn handle_get_ugc(request: Box<RequestGetUGCParams>) -> HandlerResult {
    let mut connection = database_connection().await?;
    let ugc = sqlx::query_as::<_, Ugc>("SELECT * FROM ugc WHERE ugc_code = $1")
        .bind(&request.ugc_code)
        .fetch_one(&mut *connection)
        .await?;

    Ok(ResponseParams::GetUGC(ResponseGetUGCParams {
        unk1: 0,
        ugc_code: ugc.ugc_code,
        data: ugc.data,
        unk2: 0,
        steam_id: ugc.steam_id,
    }))
}

pub async fn handle_get_ugc_status(request: Box<RequestGetUGCStatusParams>) -> HandlerResult {
    let mut connection = database_connection().await?;
    let entries = sqlx::query_as::<_, Ugc>("SELECT * FROM ugc WHERE ugc_code = ANY($1)")
        .bind(request.ugc_codes)
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

    Ok(ResponseParams::GetUGCStatus(ResponseGetUGCStatusParams { entries }))
}

#[derive(sqlx::FromRow)]
struct Ugc {
    ugc_id: i32,
    ugc_code: String,
    steam_id: String,
    data: Vec<u8>,
}

impl From<Ugc> for ResponseGetUGCStatusParamsEntry {
    fn from(val: Ugc) -> Self {
        ResponseGetUGCStatusParamsEntry {
            ugc_code: val.ugc_code,
            unk2: 0,
        }
    }
}
