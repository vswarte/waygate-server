use std::error::Error;

use actix_web::{
    delete, get, post, web::{Data, Json, Path, Query}, Responder
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Row};

use crate::api::AppState;

const DEFAULT_INDEX_LIMIT: i32 = 100;

#[get("/ban")]
async fn get_ban(
    state: Data<AppState>,
    Query(pagination): Query<PaginationParameters>,
) -> Result<impl Responder, Box<dyn Error>> {
    let total: i64 = query("SELECT COUNT(ban_id) as total FROM bans")
        .fetch_one(&state.database)
        .await?
        .get("total");

    let entries = query_as::<_, Ban>("SELECT * FROM bans ORDER BY ban_id LIMIT $1 OFFSET $2")
        .bind(pagination.limit.as_ref().unwrap_or(&DEFAULT_INDEX_LIMIT))
        .bind(pagination.offset.as_ref().unwrap_or(&0))
        .fetch_all(&state.database)
        .await?
        .into_iter()
        .collect::<Vec<_>>();

    Ok(Json(PaginatedResponse::new(total, entries)))
}

#[post("/ban")]
async fn post_ban(
    state: Data<AppState>,
    request: Json<NewBan>,
) -> Result<impl Responder, Box<dyn Error>> {
    // Clear previous ban if one exists
    sqlx::query("DELETE FROM bans WHERE external_id = $1")
        .bind(&request.external_id)
        .execute(&state.database)
        .await?;

    // Insert ban per request
    let ban_id: i64 = sqlx::query("INSERT INTO bans (external_id) VALUES ($1) RETURNING ban_id")
        .bind(&request.external_id)
        .fetch_one(&state.database)
        .await?
        .get("ban_id");

    Ok(Json(ban_id))
}

#[delete("/ban/{external_id}")]
async fn delete_ban(
    state: Data<AppState>,
    external_id: Path<(String,)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let external_id = &external_id.into_inner().0;
    sqlx::query("DELETE FROM bans WHERE external_id = $1")
        .bind(external_id)
        .execute(&state.database)
        .await?;

    Ok(Json(true))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Ban {
    ban_id: i64,
    external_id: String,
}

#[derive(Debug, Deserialize)]
struct NewBan {
    external_id: String,
}

#[derive(Deserialize)]
pub struct PaginationParameters {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pagination: PaginatedResponseMeta,
    data: Vec<T>,
}

#[derive(Serialize)]
struct PaginatedResponseMeta {
    total: i64,
    count: usize,
}

impl<T> PaginatedResponse<T> {
    pub fn new(total: i64, data: Vec<T>) -> Self {
        let count = data.len();
        let pagination = PaginatedResponseMeta { total, count };

        Self { pagination, data }
    }
}
