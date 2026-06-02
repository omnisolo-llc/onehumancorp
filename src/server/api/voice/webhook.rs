use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TwilioWebhookPayload {
    pub call_sid: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub call_status: Option<String>,
}

pub async fn handle_incoming_call(
    payload: web::Form<TwilioWebhookPayload>,
) -> impl Responder {
    tracing::info!("Received incoming call: {:?}", payload);
    HttpResponse::Ok().body("<Response><Say>Hello from OHC Voice Agent</Say></Response>")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/webhooks/voice/incoming")
            .route(web::post().to(handle_incoming_call)),
    );
}
