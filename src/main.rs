// Clippy Rules
#![deny(clippy::unwrap_used)]
#![allow(clippy::single_char_pattern)]

use veritas::{server, db};
use actix_web::{HttpServer, App, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_env().unwrap();

    let client = db::ConnectionManager::new(vec!["127.0.0.1:2379".to_string()], None).await.unwrap();

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(server::AppState{ etcd_client: client.clone() })).service(web::scope("/api").configure(server::config))
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
