mod error;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

#[doc(inline)]
pub use types;

pub mod requests;
use requests::*;

#[derive(Clone)]
pub struct Client {
    host: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(host: impl ToString) -> Self {
        Self {
            host: host.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn generate<'a>(&'a self, brain: impl ToString) -> GenerateRequest<'a> {
        GenerateRequest {
            url: &self.host,
            client: &self.client,
            brain: brain.to_string(),
            context: None,
            min: None,
            max: None,
        }
    }

    pub fn train<'a>(&'a self, brain: impl ToString, data: impl ToString) -> TrainRequest<'a> {
        TrainRequest {
            url: &self.host,
            client: &self.client,
            brain: brain.to_string(),
            data: data.to_string(),
        }
    }

    pub fn new_brain<'a>(
        &'a self,
        brain: impl ToString,
        brain_file: impl ToString,
    ) -> NewBrainRequest<'a> {
        NewBrainRequest {
            url: &self.host,
            client: &self.client,
            brain: brain.to_string(),
            brain_file: brain_file.to_string(),
            depth: None,
        }
    }

    pub fn save<'a>(&'a self, brain: impl ToString) -> SaveRequest<'a> {
        SaveRequest {
            url: &self.host,
            client: &self.client,
            brain: brain.to_string(),
        }
    }

    pub fn list<'a>(&'a self) -> ListRequest<'a> {
        ListRequest {
            url: &self.host,
            client: &self.client,
        }
    }
}

#[cfg(test)]
mod tests;
