mod util;
use util::*;

mod handlers;
mod routes;

mod server;

pub use server::Server;

mod models {
    pub use types::*;
}

#[cfg(test)]
mod tests;
