use actix_web::web;

mod health;
mod sessions;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    health::config_routes(cfg);
    sessions::config_routes(cfg);
    
    
}
