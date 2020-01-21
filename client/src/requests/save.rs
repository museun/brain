use super::*;

pub struct SaveRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a mut reqwest::Client,
    pub(crate) brain: Option<String>,
}

impl<'a> SaveRequest<'a> {
    pub fn brain(mut self, name: impl ToString) -> Self {
        self.brain.replace(name.to_string());
        self
    }

    pub async fn send(self) -> Result<responses::Saved> {
        let Self {
            client, url, brain, ..
        } = self;

        let url = format!(
            "{}/save/{}",
            url,
            brain.ok_or_else(|| Error::NoBrainProvided)?
        );
        client
            .put(&url)
            .send()
            .and_then(|ok| ok.json())
            .await
            .map_err(|err| Error::Client { err })
    }
}
