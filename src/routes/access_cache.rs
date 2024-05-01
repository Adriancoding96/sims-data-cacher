use std::convert::Infallible;
use std::num::NonZeroUsize;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{Json, Response};
use serde_json::{to_string, Value};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::{CacheData, new_cache};
//use thiserror::Error;

/*
#[derive(Error, Debug)]
pub enum CacheError {
    #[error("serialization failed")]
    SerializationError(#[from] serde_json::Error),
    #[error("serialization failed")]
    InternalServerError,
}*/

#[derive(Deserialize, Serialize)]
pub struct DataModel {
    key: String, //Stores unique identifier of entity/dto
    json_object: Value, //JSON representation of entity/dto
}

static CACHE: Lazy<CacheData> = Lazy::new( || {new_cache(NonZeroUsize::new(100).unwrap())});

//Function to attempt to retrieve data from cache. If the data is not present return string to signal main API
//TODO Remove Infallible and pattern matching for error handling
pub async fn access_cache(data: Json<DataModel>) -> Result<Response, Infallible> {
    let mut cache = CACHE.lock().await;
    if let Some(_value) = cache.get(&data.key) {
        match to_string(&data.json_object) { //Validation check to ensure the JSON structure is okay
            Ok(json) => Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(json))
                .unwrap()),
            Err(_) => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid JSON structure"))
                .unwrap())
        }
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("data not in cache"))
            .unwrap())
    }
}

//Function to insert new data in to the cache
//TODO Remove Infallible and pattern matching for error handling
pub async fn insert_cache(data: Json<DataModel>) -> Result<Response, Infallible> {
    let mut cache = CACHE.lock().await;
    match to_string(&data.json_object) { //Serialize json object before caching
        Ok(json_str) => {
            cache.put(data.key.clone(), json_str); // If serialization is successful, insert the serialized string into the cache
            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::from("Data cached successfully"))
                .unwrap())
        },
        Err(_) => {
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to serialize JSON object"))
                .unwrap())
        }
    }
}