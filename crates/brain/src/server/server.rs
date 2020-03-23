use hashbrown::HashMap;
use markov::Markov;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

use super::routes;
use crate::config::{BrainConfig, ConfiguredMarkov};

pub type BrainDb = Arc<Brain>;

pub struct Brain {
    pub config: BrainConfig,
    pub markov: Mutex<Markov>,
}

impl std::fmt::Debug for Brain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Brain")
            .field("name", &self.config.name)
            .field("read_only", &self.config.read_only)
            .finish()
    }
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

impl std::fmt::Debug for Topics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Topics").finish()
    }
}

#[derive(Default)]
pub struct Server {
    brains: HashMap<String, BrainDb>,
}

impl Server {
    pub fn add_brain(&mut self, brain: ConfiguredMarkov) {
        let ConfiguredMarkov { config, markov } = brain;
        let name = config.name.clone();
        let brain = Arc::new(Brain {
            config,
            markov: Mutex::new(markov),
        });
        self.brains.insert(name, brain);
    }

    pub async fn run(self, config: impl Into<PathBuf>, port: u16) {
        let brains = Arc::new(Topics::new(config, self.brains));
        let routes = routes::generate(Arc::clone(&brains))
            .or(routes::save(Arc::clone(&brains)))
            .or(routes::train(Arc::clone(&brains)))
            .or(routes::new(Arc::clone(&brains)))
            .or(routes::list(Arc::clone(&brains)));

        let address = std::env::var("BRAIN_LISTEN_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let addr = format!("{}:{}", address, port);
        let addr = addr
            .parse::<std::net::SocketAddr>()
            .map_err(|err| {
                log::error!(target: "brain", "cannot parse: '{}': {}", addr, err);
                err
            })
            .unwrap();

        log::info!(target: "brain", "listening on: {}", addr);
        warp::serve(routes).run(addr).await;
    }
}
