use actix_web::web;

mod health;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").configure(health::config_routes));
}
