mod error;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

pub mod requests;
use requests::*;

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

    pub fn generate<'a>(&'a mut self) -> GenerateRequest<'a> {
        GenerateRequest {
            url: &self.host,
            client: &mut self.client,
            brain: None,
            context: None,
            min: None,
            max: None,
        }
    }

    pub fn train<'a>(&'a mut self) -> TrainRequest<'a> {
        TrainRequest {
            url: &self.host,
            client: &mut self.client,
            brain: None,
            data: None,
        }
    }

    pub fn new_brain<'a>(&'a mut self) -> NewBrainRequest<'a> {
        NewBrainRequest {
            url: &self.host,
            client: &mut self.client,
            brain: None,
            brain_file: None,
            depth: None,
        }
    }

    pub fn save<'a>(&'a mut self) -> SaveRequest<'a> {
        SaveRequest {
            url: &self.host,
            client: &mut self.client,
            brain: None,
        }
    }

    pub fn list<'a>(&'a mut self) -> ListRequest<'a> {
        ListRequest {
            url: &self.host,
            client: &mut self.client,
        }
    }
}
