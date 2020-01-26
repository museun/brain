use super::*;

pub struct GenerateRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a mut reqwest::Client,
    pub(crate) brain: Option<String>,
    pub(crate) context: Option<String>,
    pub(crate) min: Option<usize>,
    pub(crate) max: Option<usize>,
}

impl<'a> GenerateRequest<'a> {
    pub fn brain(mut self, name: impl ToString) -> Self {
        self.brain.replace(name.to_string());
        self
    }

    pub fn context(mut self, context: impl ToString) -> Self {
        self.context.replace(context.to_string());
        self
    }

    pub fn min(mut self, min: usize) -> Self {
        self.min.replace(min);
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max.replace(max);
        self
    }

    pub async fn send(self) -> Result<responses::Generated> {
        let Self {
            client,
            url,
            brain,

            context,
            min,
            max,
            ..
        } = self;

        let url = format!(
            "{}/generate/{}",
            url,
            brain.ok_or_else(|| Error::NoBrainProvided)?
        );

        client
            .get(&url)
            .query(&input::GenerateOptions { context, min, max })
            .send()
            .and_then(|ok| ok.json())
            .await
            .map_err(|err| {
                tracing::error!(err = %err, "error sending");
                Error::Client { err }
            })
    }
}
