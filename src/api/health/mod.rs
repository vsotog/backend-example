use crate::Backend;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(readiness).service(liveness);
}

#[get("/readiness")]
async fn readiness(_backend: Data<Backend>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/liveness")]
async fn liveness(_backend: Data<Backend>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
