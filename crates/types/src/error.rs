use serde::{Deserialize, Serialize};

// TODO this isn't a real error
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "error")]
pub enum Error {
    ReadOnly,
    NotEnoughState,
    CannotRotate { file: String, reason: String },
    CannotSave { file: String, reason: String },
    AlreadyExists { name: String },
}
