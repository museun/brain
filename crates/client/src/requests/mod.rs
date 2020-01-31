use crate::{Error, Result};
use types::{input, responses};

mod generate;
pub use generate::GenerateRequest;

mod list;
pub use list::ListRequest;

mod save;
pub use save::SaveRequest;

mod train;
pub use train::TrainRequest;

mod new_brain;
pub use new_brain::NewBrainRequest;

type Response = std::result::Result<reqwest::Response, reqwest::Error>;

async fn check_response<T>(resp: Response) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let resp = resp.map_err(|err| Error::Client { err })?;
    if !dbg!(resp.status()).is_success() {
        let error = Error::Server {
            err: resp
                .json::<types::Error>()
                .await
                .map_err(|err| Error::Client { err })?,
        };
        return Err(error);
    }
    resp.json().await.map_err(|err| Error::Client { err })
}
