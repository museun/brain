#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Deserialize(bincode::Error),
    Serialize(bincode::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "io error: {}", err),
            Error::Deserialize(err) => write!(f, "deserialize error: {}", err),
            Error::Serialize(err) => write!(f, "serialize error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Deserialize(err) => Some(err),
            Error::Serialize(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
