use super::*;

pub struct SaveRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a  reqwest::Client,
    pub(crate) brain: String,
}

impl<'a> SaveRequest<'a> {
    pub async fn send(self) -> Result<responses::Saved> {
        let resp = self
            .client
            .put(&format!("{}/save/{}", self.url, self.brain))
            .send()
            .await;

        check_response(resp).await
    }
}
