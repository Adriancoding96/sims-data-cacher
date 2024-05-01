mod access_cache;

use axum::Router;
use axum::routing::{get, post};
use access_cache::access_cache;
use crate::routes::access_cache::insert_cache;

pub fn create_routes() -> Router {
    Router::new().route("/", get(access_cache))
        .route("/insert", post(insert_cache))
}