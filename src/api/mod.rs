use actix_web::web;

mod health;
mod kiosk;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").configure(health::config_routes)).service(web::scope("/kiosk").configure(kiosk::config_routes));
    
}
