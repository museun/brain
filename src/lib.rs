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
pub use util::*;

mod filter;
pub use filter::filter;

mod train;
pub use train::train;

mod load;
pub use load::load;

mod host;
pub use host::Server;

mod markov;
pub use markov::Markov;

mod stats;
pub use stats::Stats;
