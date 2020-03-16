use super::*;

pub struct TrainRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a  reqwest::Client,
    pub(crate) brain: String,
    pub(crate) data: String,
}

impl<'a> TrainRequest<'a> {
    pub async fn send(self) -> Result<responses::Trained> {
        let resp = self
            .client
            .post(&format!("{}/train/{}", self.url, self.brain))
            .json(&input::TrainData { data: self.data })
            .send()
            .await;

        check_response(resp).await
    }
}
