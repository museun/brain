use crate::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
pub enum Token {
    Word(Vec<u8>),
    End,
}
