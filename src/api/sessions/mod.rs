use log::info;
use uuid::Uuid;
use crate::Backend;
use crate::domain::user::User;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, web, HttpResponse};


pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(start_session).service(get_session).service(end_session);
}

#[post("/sessions")]
async fn start_session(backend: Data<Backend>, request: web::Json<User>) -> HttpResponse {
    let mut users = backend.config.application.dev_users.clone().unwrap();
    users.insert(request.id.to_string(), request.0.clone());
    if backend.config.application.dev_users.clone().unwrap().contains_key(&request.id.to_string()) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }


}

// TODO Return user
#[get("/sessions/{user_id}")]
async fn get_session(backend: Data<Backend>, user_id: Path<Uuid>) -> HttpResponse {
    let users = backend.config.application.dev_users.clone().unwrap();
    let user_id = user_id.into_inner().to_string();

    if users.contains_key(&user_id) {
        let user = users.get(&user_id).unwrap();
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }



}

#[delete("/sessions/{user_id}")]
async fn end_session(backend: Data<Backend>, user_id: Path<Uuid>) -> HttpResponse {
       if let Some(_) = backend.config.application.dev_users.clone().as_mut().map(|users| users.remove(&user_id.into_inner().to_string())) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
