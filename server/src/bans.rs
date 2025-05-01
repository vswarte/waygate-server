use serde::Serialize;
use sqlx::{Pool, Postgres, Row};

pub struct BanService {
    pub database: Pool<Postgres>,
}

impl BanService {
    pub fn new(database: Pool<Postgres>) -> Self {
        Self { database }
    }

    pub async fn add_ban(&self, external_id: &str) -> Result<i64, sqlx::Error> {
        let ban_id = sqlx::query("INSERT INTO bans (external_id) VALUES ($1) RETURNING ban_id")
            .bind(external_id)
            .fetch_one(&self.database)
            .await?
            .get("ban_id");

        Ok(ban_id)
    }

    pub async fn delete_ban(&self, external_id: &str) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query("DELETE FROM bans WHERE external_id = $1")
            .bind(external_id)
            .execute(&self.database)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn get_ban(&self, external_id: &str) -> Result<Option<BanRecord>, sqlx::Error> {
        let ban = sqlx::query_as::<_, BanRecord>("SELECT * FROM bans WHERE external_id = $1")
            .bind(external_id)
            .fetch_optional(&self.database)
            .await?;

        Ok(ban)
    }

    pub async fn list_bans(&self, limit: i64, offset: i64) -> Result<Vec<BanRecord>, sqlx::Error> {
        let bans =
            sqlx::query_as::<_, BanRecord>("SELECT * FROM bans ORDER BY ban_id LIMIT $1 OFFSET $2")
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.database)
                .await?;

        Ok(bans)
    }

    pub async fn get_total(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query("SELECT COUNT(*) FROM bans")
            .fetch_one(&self.database)
            .await?
            .get(0);

        Ok(count)
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct BanRecord {
    pub ban_id: i64,
    pub external_id: String,
    pub banned_at: i64,
}
