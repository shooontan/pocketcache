use actix_web::{web, HttpRequest, Responder};
use pocketcache::cache::Cache;
use std::sync::{Arc, Mutex};

use crate::models::user::User;

pub async fn get(req: HttpRequest, cache: web::Data<Arc<Mutex<Cache<User>>>>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    // delete in cache
    let mut cache = cache.lock().unwrap();
    cache.delete(name);

    format!("Goodbye {}!", &name)
}
