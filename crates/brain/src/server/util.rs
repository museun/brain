use super::models::{self, Error};
use super::server::{BrainDb, Topics};

use std::convert::Infallible;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{de::DeserializeOwned, Serialize};
use warp::{
    http::StatusCode,
    reject::{self, Reject},
    reply, Filter, Rejection, Reply,
};

#[derive(Debug)]
struct AlreadyExistsReject {
    name: String,
}

impl Reject for AlreadyExistsReject {}

type JsonResult = Result<reply::WithStatus<reply::Json>, Infallible>;

pub fn error(error: Error) -> JsonResult {
    return Ok(reply::with_status(
        reply::json(&error),
        StatusCode::BAD_REQUEST,
    ));
}

pub fn okay<T>(item: T) -> JsonResult
where
    T: Serialize,
{
    Ok(reply::with_status(reply::json(&item), StatusCode::OK))
}

pub fn json_body<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: Send + DeserializeOwned,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn recover(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(AlreadyExistsReject { name }) = err.find() {
        return Ok(reply::with_status(
            reply::json(&models::Error::AlreadyExists { name: name.clone() }),
            StatusCode::BAD_REQUEST,
        ));
    }
    Err(err)
}

pub async fn filter(topics: Arc<Topics>, name: String) -> Result<BrainDb, Rejection> {
    topics
        .brains
        .lock()
        .await
        .get(&name)
        .map(Arc::clone)
        .ok_or_else(reject::not_found)
}

pub async fn expect_unique(
    topics: Arc<Topics>,
    name: String,
) -> Result<(Arc<Topics>, String), Rejection> {
    if topics.brains.lock().await.contains_key(&name) {
        return Err(reject::custom(AlreadyExistsReject { name }));
    }
    Ok((topics, name))
}

pub async fn rotate(input: &PathBuf) -> std::io::Result<()> {
    let mut new = input.to_owned();
    new.set_extension("bak");
    tokio::fs::rename(&input, new).await
}
