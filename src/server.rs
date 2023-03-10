use actix_web::{web, get, put, delete, HttpResponse, Responder};
use crate::db;

#[derive(Clone)]
pub struct AppState {
    pub etcd_client: db::ConnectionManager,
}

pub fn config(cfg: &mut web::ServiceConfig) {
   cfg
    .service(web::scope("/schema")
        .service(get_all_config_keys)
        .service(add_config_key)
        .service(delete_config_key));
}

#[get("/")]
pub async fn get_all_config_keys(state: web::Data<AppState>) -> impl Responder { 
    match db::get_config_key_all(&mut state.etcd_client.get_client()).await {
        Ok(keys) => {
            let mut resp = keys.into_iter().collect::<Vec<String>>();
            resp.sort();
            HttpResponse::Ok().json(resp)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()) // TODO Change to error json object
    }
}

#[put("/{config_key}")]
pub async fn add_config_key(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let config_key = path.into_inner();
    match db::add_config_key(&mut state.etcd_client.get_client(), config_key).await {
        Ok(success) => if success { HttpResponse::Created().finish() } else { HttpResponse::Ok().finish() },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()) // TODO Change to error json object
    }
}

#[delete("/{config_key}")]
pub async fn delete_config_key(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let config_key = path.into_inner();
    match db::remove_config_key(&mut state.etcd_client.get_client(), config_key).await {
        Ok(success) => if success { HttpResponse::Created().finish() } else { HttpResponse::Ok().finish() },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), // TODO Change to error json object
    }
}
