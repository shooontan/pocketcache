mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use pocketcache::cache::Cache;
use pocketcache::time::Expiration;
use std::sync::{Arc, Mutex};

use models::user::User;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cache = Arc::new(Mutex::new(Cache::<User>::new(Expiration::Minute(1))));

    HttpServer::new(move || {
        App::new()
            .data(cache.clone())
            .route("/hello/{name}", web::get().to(handlers::hello::get))
            .route("/again/{name}", web::get().to(handlers::again::get))
            .route("/bye/{name}", web::get().to(handlers::bye::get))
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
}
