use super::server::Topics;
use super::{expect_unique, filter, handlers, json_body, recover};

use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn generate(
    topics: Arc<Topics>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("generate" / String)
        .and(warp::get())
        .and_then(move |name| filter(Arc::clone(&topics), name))
        .and(warp::query())
        .and_then(handlers::generate)
        .recover(recover)
}

pub fn train(topics: Arc<Topics>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("train" / String)
        .and_then(move |name| filter(Arc::clone(&topics), name))
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::train)
        .recover(recover)
}

pub fn new(topics: Arc<Topics>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("new" / String)
        .and_then(move |name| expect_unique(Arc::clone(&topics), name))
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::new)
        .recover(recover)
}

pub fn save(topics: Arc<Topics>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("save" / String)
        .and(warp::put())
        .and_then(move |name| filter(Arc::clone(&topics), name))
        .and_then(handlers::save)
        .recover(recover)
}

pub fn list(topics: Arc<Topics>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("list")
        .and(warp::get())
        .map(move || Arc::clone(&topics))
        .and_then(handlers::list)
        .recover(recover)
}
