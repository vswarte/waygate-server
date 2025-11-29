use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::services::eldenring::GameServices;

pub mod auth;
pub mod ban;
pub mod health;
pub mod notification;

pub struct AppState {
    pub database: Pool<Postgres>,
    pub services: Arc<GameServices>,
}
