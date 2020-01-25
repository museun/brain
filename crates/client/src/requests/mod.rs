use crate::{Error, Result};
use futures::prelude::*;
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
