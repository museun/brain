use crate::config::{BrainConfig, Config, ConfiguredMarkov};
use futures::prelude::*;
use std::path::PathBuf;

pub struct Arguments {
    pub port: u16,
    pub config_file: PathBuf,
    pub brains: Vec<ConfiguredMarkov>,
}

pub async fn load(mut args: pico_args::Arguments) -> anyhow::Result<Arguments> {
    let port: u16 = args.opt_value_from_str(["-p", "--port"])?.unwrap_or(9090);

    // TODO maybe get this from an --config argument
    let config_file = "brain.toml".into();

    // make a default one if it doesn't exist
    tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(&config_file)
        .await?;

    let config = Config::load(&config_file).await.unwrap_or_default();

    let set = futures::stream::FuturesUnordered::new();
    for (name, mut config) in config.brains {
        config.name = name;
        set.push(load_brain(config));
    }

    Ok(Arguments {
        port,
        config_file,
        brains: set.try_collect().await?,
    })
}

async fn load_brain(config: BrainConfig) -> anyhow::Result<ConfiguredMarkov> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::task::spawn_blocking(|| {
        let now = std::time::Instant::now();
        let res = markov::load(&config.brain_file)
            .map_err(|err| {
                tracing::error!(
                    "cannot load brain file '{}' for '{}': {}",
                    config.brain_file.display(),
                    &config.name,
                    err
                );
                err
            })
            .map(|markov| {
                tracing::debug!("loading took: {:.2?}", now.elapsed());
                ConfiguredMarkov { markov, config }
            });
        let _ = tx.send(res);
    });

    let res = rx.await??;
    Ok(res)
}
