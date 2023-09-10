use std::collections::HashMap;
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::Backend;
use crate::domain::user::User;
use actix_web::web::Data;
use actix_web::{delete, post, web, HttpResponse};

lazy_static! {
    static ref HASHMAP: HashMap<String, User> = {
        let mut m = HashMap::new();
        let id = Uuid::new_v4();
        m.insert(id.to_string(),User {
            id,
            first_name: String::from("Corey"),
            last_name: String::from("Arnold"),
            url: String::from("test.com"),
        });
        m
    };
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(start_session).service(end_session);
}

#[post("/session")]
async fn start_session(_backend: Data<Backend>, request: web::Json<User>) -> HttpResponse {

    if HASHMAP.contains_key(&request.id.to_string()) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }


}

#[delete("/session")]
async fn end_session(_backend: Data<Backend>, request: web::Json<User>) -> HttpResponse {
    if HASHMAP.contains_key(&request.id.to_string()) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
