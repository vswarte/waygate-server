use std::error::Error;

use actix_web::web::{Json, Path, Query};
use actix_web::{delete, get, post, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Row};
use waygate_database::database_connection;
use waygate_session::{clear_bans, create_ban};

use crate::pagination::{PaginatedResponse, PaginationParameters};

const DEFAULT_INDEX_LIMIT: i32 = 100;

#[get("/ban")]
async fn get_ban(
    Query(pagination): Query<PaginationParameters>,
) -> Result<impl Responder, Box<dyn Error>> {
    let mut connection = database_connection().await?;

    // TODO: make a single query
    let total: i64 = query("SELECT COUNT(ban_id) as total FROM bans")
        .fetch_one(&mut *connection)
        .await?
        .get("total");

    let entries = query_as::<_, Ban>("SELECT * FROM bans ORDER BY ban_id LIMIT $1 OFFSET $2")
        .bind(pagination.limit.as_ref().unwrap_or(&DEFAULT_INDEX_LIMIT))
        .bind(pagination.offset.as_ref().unwrap_or(&0))
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .collect::<Vec<_>>();

    Ok(Json(PaginatedResponse::new(total, entries)))
}

#[post("/ban")]
async fn post_ban(request: Json<NewBan>) -> Result<impl Responder, Box<dyn Error>> {
    let ban_id = create_ban(&request.external_id).await?;
    Ok(Json(ban_id))
}

#[delete("/ban/{external_id}")]
async fn delete_ban(external_id: Path<(String,)>) -> Result<impl Responder, Box<dyn Error>> {
    let external_id = &external_id.into_inner().0;
    clear_bans(external_id).await?;
    Ok(Json(true))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Ban {
    ban_id: i32,
    external_id: String,
}

#[derive(Debug, Deserialize)]
struct NewBan {
    external_id: String,
}
