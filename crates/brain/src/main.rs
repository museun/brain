mod util;
use util::*;

mod handlers;
mod routes;

mod args;
mod stats;
mod usage;

mod load;
mod train;

mod config;

mod models {
    pub use types::*;
}

mod server;

mod error;
use error::Error;

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::Subscriber::builder()
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let load::Arguments {
        port,
        config_file,
        brains,
    } = match args::parse_args()? {
        args::Command::Train(args) => return train::train(args).await,
        args::Command::Load(args) => match load::load(args).await {
            Ok(args) => args,
            Err(err) => {
                tracing::error!("cannot load configuration file at 'brain.toml': {}", err);
                tracing::error!("verify the file exists and well-formed.");
                tracing::error!("here's a sample config:");
                config::Config::print_default();
                std::process::exit(1);
            }
        },
    };

    let mut server = server::Server::default();
    for brain in brains {
        server.add_brain(brain)
    }
    server.run(config_file, port).await;
    Ok(())
}
