use super::*;

pub struct NewBrainRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a mut reqwest::Client,
    pub(crate) brain: String,
    pub(crate) brain_file: String,
    pub(crate) depth: Option<usize>,
}

impl<'a> NewBrainRequest<'a> {
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth.replace(depth);
        self
    }

    pub async fn send(self) -> Result<responses::Created> {
        let resp = self
            .client
            .post(&format!("{}/new/{}", self.url, self.brain))
            .json(&input::NewBrain {
                brain_file: self.brain_file,
                depth: self.depth.unwrap_or(5),
            })
            .send()
            .await;

        check_response(resp).await
    }
}
