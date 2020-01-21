use server::models;

#[derive(Debug)]
pub enum Error {
    NoBrainProvided,
    NoDataProvided,
    NoBrainFileProvided,
    Server { err: models::Error },
    Client { err: reqwest::Error },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoBrainProvided => f.write_str("No brain provided"),
            Error::NoDataProvided => f.write_str("No data provided"),
            Error::NoBrainFileProvided => f.write_str("No brain file provided"),
            // TODO
            Error::Server { err } => write!(f, "server error: {:?}", err),
            Error::Client { err } => write!(f, "client error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Client { err } => Some(err),

            // TODO
            Error::Server { .. } => None,
            Error::NoBrainProvided => None,
            Error::NoDataProvided => None,
            Error::NoBrainFileProvided => None,
        }
    }
}
