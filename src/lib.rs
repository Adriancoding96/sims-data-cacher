pub mod routes;

use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::sync::{Arc};
use tokio::sync::Mutex;
use lru::LruCache;
use crate::routes::create_routes;

pub async fn run() {
    let app = create_routes();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type CacheData = Arc<Mutex<LruCache<String, String>>>;

/*
*
* Arc<Mutex<LruCache<String, String>>> explanation,
* LruCache<> is a structure that allows automatic removal of the last accessed item when cache reaches capacity.
* Mutex<> ensures that only one thread can access the cache at a time.
* Arc<> is a smart pointer that that enables multiple threads to own a shared resource.
*
*/

pub fn new_cache(size: NonZeroUsize) -> CacheData {
    const MAX_SIZE: NonZeroUsize = match NonZeroUsize::new(100)  {
        Some(nz) => nz,
        None => panic!("Invalid NonZeroUsize"),
    };
    let effective_size = size.min(MAX_SIZE);
    let cache: LruCache<String, String> = LruCache::new(effective_size);
    Arc::new(Mutex::new(cache))
}