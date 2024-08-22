use std::error::Error;

use sqlx::query;
use sqlx::prelude::*;
use waygate_database::database_connection;

pub async fn is_banned(external_id: &str) -> Result<bool, Box<dyn Error>> {
    let mut connection = database_connection().await?;

    let is_banned: bool = query("SELECT EXISTS(SELECT 1 FROM bans WHERE external_id = $1)")
        .bind(external_id)
        .fetch_one(&mut *connection)
        .await?
        .get("exists");

    Ok(is_banned)
}

#[tracing::instrument]
pub async fn clear_bans(external_id: &str) -> Result<(), Box<dyn Error>> {
    let mut connection = database_connection().await?;
    sqlx::query("DELETE FROM bans WHERE external_id = $1")
        .bind(external_id)
        .execute(&mut *connection)
        .await?;

    Ok(())
}

#[tracing::instrument]
pub async fn create_ban(external_id: &str) -> Result<i32, Box<dyn Error>> {
    let mut connection = database_connection().await?;
    sqlx::query("DELETE FROM bans WHERE external_id = $1")
        .bind(external_id)
        .execute(&mut *connection)
        .await?;

    let mut connection = database_connection().await?;
    let ban_id: i32 = sqlx::query("INSERT INTO bans (external_id) VALUES ($1) RETURNING ban_id")
        .bind(external_id)
        .fetch_one(&mut *connection)
        .await?
        .get("ban_id");

    Ok(ban_id)
}
