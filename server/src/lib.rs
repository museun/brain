use hashbrown::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod util;
use util::*;

mod handlers;
mod routes;

pub mod models;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct Server {
    brains: HashMap<String, models::BrainDb>,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_brain(&mut self, brain: config::ConfiguredMarkov) {
        let config::ConfiguredMarkov { config, markov } = brain;
        let name = config.name.clone();
        let brain = Arc::new(models::Brain {
            config,
            markov: Mutex::new(markov),
        });
        self.brains.insert(name, brain);
    }

    pub fn with(mut self, brain: config::ConfiguredMarkov) -> Self {
        self.add_brain(brain);
        self
    }

    pub async fn run(self, config: impl Into<PathBuf>, port: u16) {
        let brains = Arc::new(models::Topics::new(config, self.brains));
        let routes = routes::generate(Arc::clone(&brains))
            .or(routes::save(Arc::clone(&brains)))
            .or(routes::train(Arc::clone(&brains)))
            .or(routes::new(Arc::clone(&brains)))
            .or(routes::list(Arc::clone(&brains)));

        let addr = format!("127.0.0.1:{}", port);
        let addr = addr
            .parse::<std::net::SocketAddr>()
            .map_err(|err| {
                tracing::error!("cannot parse: '{}': {}", addr, err);
                err
            })
            .unwrap();

        tracing::info!("listening on: {}", addr);
        warp::serve(routes).run(addr).await;
    }
}
