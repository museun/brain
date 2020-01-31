use super::*;
pub struct ListRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a mut reqwest::Client,
}

impl<'a> ListRequest<'a> {
    pub async fn send(self) -> Result<responses::List> {
        let url = format!("{}/list", self.url);
        let resp = self.client.get(&url).send().await;
        check_response(resp).await
    }
}
