use actix_web::{
    http::header::ContentType,
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use message::{
    builder::MessageBuilder,
    eldenring::{
        NotifyParams, NotifyParamsSection1, NotifyParamsSection2, ObjectIdentifier, PushParams,
    },
};
use rand::Rng;
use serde::Deserialize;

use super::AppState;

/// Route to hit from eg. a docker container to ensure the process is still running.
#[post("/notification/announcement")]
pub async fn announcement(
    state: Data<AppState>,
    req: web::Json<NotifyMessageRequest>,
) -> impl Responder {
    let message = wire::serialize(
        MessageBuilder::push()
            .body(PushParams::Notify(NotifyParams {
                identifier: ObjectIdentifier(rand::rng().random::<i64>()),
                timestamp: 0,
                section1: NotifyParamsSection1::Variant1 { unk1: 0, unk2: 0 },
                section2: NotifyParamsSection2::Variant1 {
                    message: req.message.clone(),
                    unk1: 0x0,
                    unk2: 0x0,
                    unk3: 0x0,
                },
            }))
            .build()
            .expect("Could not build push message"),
    ).expect("Could not serialize push message");

    state
        .services
        .notifications
        .broadcast(message)
        .expect("Could not broadcast push message");

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("true")
}

#[derive(Debug, Deserialize)]
struct NotifyMessageRequest {
    message: String,
}
