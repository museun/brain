#![feature(nll, pattern)]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate tiny_http;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde_json;
extern crate sysinfo;

#[macro_use]
mod util;
pub use crate::util::*;

mod filter;
pub use crate::filter::filter;

mod train;
pub use crate::train::train;

mod load;
pub use crate::load::load;

mod host;
pub use crate::host::Server;

mod markov;
pub use crate::markov::Markov;

mod stats;
pub use crate::stats::Stats;
