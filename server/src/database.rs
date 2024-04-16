use sqlx::{Pool, Postgres};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Could not get DATABASE_URL from .env {0}")]
    MissingDatabaseUrl(#[from] dotenvy::Error),

    #[error("Sqlx error {0}")]
    Sqlx(#[from] sqlx::Error),
}

pub async fn pool() -> Result<Pool<Postgres>, DatabaseError> {
    let pool = Pool::<Postgres>::connect(&dotenvy::var("DATABASE_URL")?).await?;

    Ok(pool)
}
