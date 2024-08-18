use std::error::Error;

use actix_web::{delete, get, post, Responder};
use actix_web::web::{Json, Path};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Row};
use waygate_database::database_connection;

#[get("/ban")]
async fn get_ban() -> Result<impl Responder, Box<dyn Error>>{
    let mut connection = database_connection().await?;
    let entries = query_as::<_, Ban>("SELECT * FROM bans")
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .collect::<Vec<_>>();

    Ok(Json(entries))
}

#[post("/ban")]
async fn post_ban(request: Json<NewBan>) -> Result<impl Responder, Box<dyn Error>>{
    let mut connection = database_connection().await?;
    let ban_id: i32 = sqlx::query("INSERT INTO bans (external_id) VALUES ($1) RETURNING ban_id")
        .bind(&request.external_id)
        .fetch_one(&mut *connection)
        .await?
        .get("ban_id");

    tracing::info!("Banned player. external_id = {}", &request.external_id);

    Ok(Json(ban_id))
}

#[delete("/ban/{external_id}")]
async fn delete_ban(external_id: Path<(String,)>) -> Result<impl Responder, Box<dyn Error>>{
    let external_id = &external_id.into_inner().0;

    let mut connection = database_connection().await?;
    sqlx::query("DELETE FROM bans WHERE external_id = $1")
        .bind(external_id)
        .execute(&mut *connection)
        .await?;

    tracing::info!("Lifted ban for player. external_id = {external_id}");

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
