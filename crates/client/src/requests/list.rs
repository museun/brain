use super::*;
pub struct ListRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) client: &'a mut reqwest::Client,
}

impl<'a> ListRequest<'a> {
    pub async fn send(self) -> Result<responses::List> {
        let Self { client, url, .. } = self;
        let url = format!("{}/list", url,);
        client
            .get(&url)
            .send()
            .and_then(|ok| ok.json())
            .await
            .map_err(|err| {
                tracing::error!(err = %err, "error sending");
                Error::Client { err }
            })
    }
}
