use crate::state::State;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/healthz")]
pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[get("/readyz")]
pub async fn readiness(state: web::Data<Arc<State>>) -> impl Responder {
    if state
        .clients_ready
        .load(std::sync::atomic::Ordering::SeqCst)
    {
        HttpResponse::Ok().json(serde_json::json!({"status": "ready"}))
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({"status": "not ready"}))
    }
}
