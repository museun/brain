use crate::Result;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const SAMPLE_CONFIG: &str = include_str!("../sample_config.toml");

pub struct ConfiguredMarkov {
    pub config: BrainConfig,
    pub markov: markov::Markov,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrainConfig {
    #[serde(skip)]
    pub name: String,
    pub brain_file: PathBuf,
    pub read_only: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub brains: HashMap<String, BrainConfig>,
}

impl Config {
    pub fn print_default() {
        println!("{}", SAMPLE_CONFIG)
    }

    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let config = tokio::fs::read_to_string(path).await?;
        let res = toml::from_str(&config)?;
        Ok(res)
    }

    #[allow(dead_code)]
    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        tokio::fs::write(path, toml::to_string_pretty(&self)?.as_bytes()).await?;
        Ok(())
    }
}
