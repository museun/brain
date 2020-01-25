use super::*;

pub struct NewBrainRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a mut reqwest::Client,
    pub(crate) brain: Option<String>,
    pub(crate) brain_file: Option<String>,
    pub(crate) depth: Option<usize>,
}

impl<'a> NewBrainRequest<'a> {
    pub fn brain(mut self, name: impl ToString) -> Self {
        self.brain.replace(name.to_string());
        self
    }

    pub fn brain_file(mut self, file: impl ToString) -> Self {
        self.brain_file.replace(file.to_string());
        self
    }

    pub async fn send(self) -> Result<responses::Created> {
        let Self {
            client,
            url,
            brain,
            brain_file,
            depth,
            ..
        } = self;

        let url = format!(
            "{}/new/{}",
            url,
            brain.ok_or_else(|| Error::NoBrainProvided)?
        );
        client
            .post(&url)
            .json(&input::NewBrain {
                brain_file: brain_file.ok_or_else(|| Error::NoBrainFileProvided)?,
                depth: depth.unwrap_or(5),
            })
            .send()
            .and_then(|ok| ok.json())
            .await
            .map_err(|err| Error::Client { err })
    }
}
