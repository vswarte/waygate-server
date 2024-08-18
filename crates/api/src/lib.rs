use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};
use thiserror::Error;

mod auth;
mod ban;
mod notify;

use auth::CheckKey;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Could not start the API webserver. {0}")]
    Start(#[from] std::io::Error),
}

pub async fn serve_api(bind: &str, api_key: &str) -> Result<(), ApiError> {
    let api_key = api_key.to_string();
    Ok(
        HttpServer::new(move || {
                App::new()
                    .wrap(CheckKey::new(&api_key))
                    .service(health)
                    .service(notify::notify_message)
                    .service(ban::get_ban)
                    .service(ban::post_ban)
                    .service(ban::delete_ban)
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
