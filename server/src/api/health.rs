use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

/// Route to hit from eg. a docker container to ensure the process is still running.
#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("true")
}
