use std::error::Error;

use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    Responder,
};
use serde::{Deserialize, Serialize};

use crate::api::AppState;

const DEFAULT_INDEX_LIMIT: i32 = 100;

#[get("/ban")]
async fn get_ban(
    state: Data<AppState>,
    Query(pagination): Query<PaginationParameters>,
) -> Result<impl Responder, Box<dyn Error>> {
    let total = state.services.bans.get_total().await?;
    let entries = state
        .services
        .bans
        .list_bans(
            pagination.limit.unwrap_or(DEFAULT_INDEX_LIMIT) as i64,
            pagination.offset.unwrap_or(0) as i64,
        )
        .await?;

    Ok(Json(PaginatedResponse::new(total, entries)))
}

#[get("/ban/{external_id}")]
async fn get_ban_by_id(
    state: Data<AppState>,
    external_id: Path<(String,)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let external_id = &external_id.into_inner().0;
    let ban = state.services.bans.get_ban(external_id).await?;

    match ban {
        Some(ban) => Ok(actix_web::HttpResponse::Ok().json(ban)),
        None => Ok(actix_web::HttpResponse::NotFound().finish()),
    }
}

#[post("/ban")]
async fn post_ban(
    state: Data<AppState>,
    request: Json<NewBan>,
) -> Result<impl Responder, Box<dyn Error>> {
    let ban_id = state.services.bans.add_ban(&request.external_id).await?;

    Ok(Json(ban_id))
}

#[delete("/ban/{external_id}")]
async fn delete_ban(
    state: Data<AppState>,
    external_id: Path<(String,)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let external_id = &external_id.into_inner().0;
    let deleted = state.services.bans.delete_ban(external_id).await?;

    Ok(Json(deleted))
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
