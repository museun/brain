use config::BrainConfig;
use hashbrown::HashMap;
use markov::Markov;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod util;
use util::*;

mod handlers;
mod routes;

#[cfg(test)]
mod tests;

pub mod models {
    pub use types::*;
}

pub type BrainDb = Arc<Brain>;

pub struct Brain {
    pub config: BrainConfig,
    pub markov: Mutex<Markov>,
}

pub struct Topics {
    pub brains: Mutex<HashMap<String, BrainDb>>,
    pub config_path: PathBuf,
}

impl Topics {
    pub fn new(path: impl Into<PathBuf>, brains: HashMap<String, BrainDb>) -> Self {
        Self {
            config_path: path.into(),
            brains: Mutex::new(brains),
        }
    }
}

#[derive(Default)]
pub struct Server {
    brains: HashMap<String, BrainDb>,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_brain(&mut self, brain: config::ConfiguredMarkov<Markov>) {
        let config::ConfiguredMarkov { config, markov } = brain;
        let name = config.name.clone();
        let brain = Arc::new(Brain {
            config,
            markov: Mutex::new(markov),
        });
        self.brains.insert(name, brain);
    }

    pub fn with(mut self, brain: config::ConfiguredMarkov<Markov>) -> Self {
        self.add_brain(brain);
        self
    }

    pub async fn run(self, config: impl Into<PathBuf>, port: u16) {
        let brains = Arc::new(Topics::new(config, self.brains));
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