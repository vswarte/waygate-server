use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};
use thiserror::Error;

mod auth;
mod notify;

use auth::CheckKey;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Could not start the API webserver. {0}")]
    Start(#[from] std::io::Error),
}

pub async fn serve_api(bind: &str, key: &str) -> Result<(), ApiError> {
    Ok(
        HttpServer::new(|| {
                App::new()
                    .wrap(CheckKey)
                    .service(health)
                    .service(notify::notify_message)
            })
            .bind(bind)?
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
