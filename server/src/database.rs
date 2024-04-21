use std::sync;

use sqlx::{pool::PoolConnection, Pool, Postgres};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Could not get DATABASE_URL from .env {0}")]
    MissingDatabaseUrl(#[from] dotenvy::Error),

    #[error("Sqlx error {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Could not acquire database connection from pool")]
    Acquire,

    #[error("Could not run database migrations {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
}


static POOL: sync::OnceLock<Pool<Postgres>> = sync::OnceLock::new();

pub async fn init() -> Result<(), DatabaseError> {
    let pool = Pool::<Postgres>::connect(&dotenvy::var("DATABASE_URL")?)
            .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    log::info!("Ran migrations");

    POOL.set(pool).expect("Could not set POOL static");
    log::info!("Initialized database pool");

    Ok(())
}

pub async fn acquire() -> Result<PoolConnection<Postgres>, DatabaseError> {
    match POOL.get() {
        Some(p) => Ok(p.acquire().await?),
        None => Err(DatabaseError::Acquire),
    }
}
