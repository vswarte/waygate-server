use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use waygate_connection::broadcast;

use waygate_message::{NotifyParams, NotifyParamsSection1, NotifyParamsSection2, ObjectIdentifier, PushParams};

#[post("/notify/message")]
async fn notify_message(req: web::Json<NotifyMessageRequest>) -> impl Responder {
    let push_id_1 = { thread_rng().gen::<i32>() };
    let push_id_2 = { thread_rng().gen::<i32>() };

    broadcast(PushParams::Notify(
        NotifyParams {
            identifier: ObjectIdentifier {
                object_id: push_id_1,
                secondary_id: push_id_2,
            },
            timestamp: 0,
            section1: NotifyParamsSection1::Variant1 {
                unk1: 0x0,
                unk2: 0x0,
            },
            section2: NotifyParamsSection2::Variant1 {
                message: req.message.clone(),
                unk1: 0x0,
                unk2: 0x0,
                unk3: 0x0,
            }
        }
    )).await.expect("Could not broadcast message");

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("true")
}

#[derive(Debug, Deserialize)]
struct NotifyMessageRequest {
    message: String,
}
