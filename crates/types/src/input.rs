use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateOptions {
    pub context: Option<String>,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainData {
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewBrain {
    pub brain_file: String,
    pub depth: usize,
}
