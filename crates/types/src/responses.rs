use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Generated {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Saved {
    pub name: String,
    pub time: Duration,
}

impl PartialEq for Saved {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trained {
    pub data: String,
    pub time: Duration,
}

impl PartialEq for Trained {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Created {
    pub name: String,
    pub brain_file: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub brains: HashMap<String, ListItem>,
    pub config_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "brain")]
pub struct ListItem {
    pub name: String,
    pub brain_file: PathBuf,
    pub read_only: bool,
}
