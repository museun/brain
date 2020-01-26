use super::*;

pub struct TrainRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a mut reqwest::Client,
    pub(crate) brain: Option<String>,
    pub(crate) data: Option<String>,
}

impl<'a> TrainRequest<'a> {
    pub fn brain(mut self, name: impl ToString) -> Self {
        self.brain.replace(name.to_string());
        self
    }

    pub fn data(mut self, data: impl ToString) -> Self {
        self.data.replace(data.to_string());
        self
    }

    pub async fn send(self) -> Result<responses::Trained> {
        let Self {
            client,
            url,
            brain,
            data,
            ..
        } = self;

        let url = format!(
            "{}/train/{}",
            url,
            brain.ok_or_else(|| Error::NoBrainProvided)?
        );

        client
            .post(&url)
            .json(&input::TrainData {
                data: data.ok_or_else(|| Error::NoDataProvided)?,
            })
            .send()
            .and_then(|ok| ok.json())
            .await
            .map_err(|err| {
                tracing::error!(err = %err, "error sending");
                Error::Client { err }
            })
    }
}
