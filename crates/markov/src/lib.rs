use hashbrown::{HashMap, HashSet};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod error;
pub use error::Error;

mod args;
mod link;
mod linkset;
mod token;

pub mod types {
    #[doc(inline)]
    pub use super::args::{Max, Min};

    #[doc(inline)]
    pub use super::link::Link;

    #[doc(inline)]
    pub use super::linkset::LinkSet;

    #[doc(inline)]
    pub use super::token::Token;
}

use types::*;

mod markov;
pub use self::markov::Markov;

pub fn load(input: impl AsRef<Path>) -> Result<Markov, Error> {
    let input = input.as_ref();
    tracing::debug!("loading from file: '{}'", input.display());
    let reader = std::fs::File::open(input)?;
    let reader = snap::Reader::new(reader);
    let markov: Markov = bincode::deserialize_from(reader).map_err(Error::Deserialize)?;
    tracing::trace!("done deserializing data, got: {}", markov.name);
    Ok(markov)
}

pub fn save(markov: &Markov, output: impl AsRef<Path>) -> Result<(), Error> {
    let output = output.as_ref();
    tracing::debug!("saving '{}' to file: {}", markov.name, output.display());
    let writer = std::fs::File::create(output)?;
    let writer = snap::Writer::new(writer);
    bincode::serialize_into(writer, &markov).map_err(Error::Serialize)?;
    tracing::trace!("done serializing data");
    Ok(())
}
