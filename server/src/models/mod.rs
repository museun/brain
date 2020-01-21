use hashbrown::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use config::BrainConfig;
use markov::Markov;

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

mod error;
pub use error::Error;

pub mod input;
pub mod responses;
