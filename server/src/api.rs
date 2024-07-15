use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};
use key::CheckKey;
use thiserror::Error;

use crate::config;

mod key;
mod notify;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Could not start the API webserver")]
    Start(#[from] std::io::Error),
}

pub async fn start_api() -> Result<(), ApiError> {
    Ok(
        HttpServer::new(|| {
                App::new()
                    .wrap(CheckKey)
                    .service(health)
                    .service(notify::notify_message)
            })
            .bind(config::get().api_bind.as_str())?
            .run()
            .await?
    )
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("true")
}

