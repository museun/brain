use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Min(pub usize);

impl Default for Min {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max(pub usize);

impl Default for Max {
    fn default() -> Self {
        Self(30)
    }
}
