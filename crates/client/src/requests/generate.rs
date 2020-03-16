use super::*;

pub struct GenerateRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a reqwest::Client,
    pub(crate) brain: String,
    pub(crate) context: Option<String>,
    pub(crate) min: Option<usize>,
    pub(crate) max: Option<usize>,
}

impl<'a> GenerateRequest<'a> {
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
        let resp = self
            .client
            .get(&format!("{}/generate/{}", self.url, self.brain))
            .query(&input::GenerateOptions {
                context: self.context,
                min: self.min,
                max: self.max,
            })
            .send()
            .await;
        check_response(resp).await
    }
}
