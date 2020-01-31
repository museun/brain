use super::models::{self, Error};
use super::server::{Brain, BrainDb, Topics};
use super::{error, okay, rotate};
use crate::config::{BrainConfig, Config};

use std::sync::Arc;
use warp::Reply;

type Result<R> = std::result::Result<R, warp::Rejection>;

#[tracing::instrument(level = "trace")]
pub async fn generate(db: BrainDb, opts: models::input::GenerateOptions) -> Result<impl Reply> {
    use rand::prelude::*;
    let data = db.markov.lock().await.generate(
        &mut thread_rng(),
        opts.min.unwrap_or(5),
        opts.max.unwrap_or(30),
        opts.context.as_ref().map(|s| s.as_str()),
    );

    match data {
        Some(data) => {
            tracing::trace!(%data, "generated");
            okay(models::responses::Generated {
                name: db.config.name.to_string(),
                data: data.to_string(),
            })
        }
        None => {
            tracing::warn!("not enough state");
            error(Error::NotEnoughState)
        }
    }
}

pub async fn train(db: BrainDb, input: models::input::TrainData) -> Result<impl Reply> {
    if db.config.read_only {
        return error(Error::ReadOnly);
    }

    let now = std::time::Instant::now();
    db.markov.lock().await.train_text(&input.data);
    okay(models::responses::Trained {
        data: input.data,
        time: now.elapsed(),
    })
}

pub async fn new(
    (topics, name): (Arc<Topics>, String),
    input: models::input::NewBrain,
) -> Result<impl Reply> {
    use tokio::io::AsyncWriteExt as _;
    let models::input::NewBrain { depth, brain_file } = input;
    let brain = Brain {
        config: BrainConfig {
            name: name.clone(),
            brain_file: brain_file.clone().into(),
            read_only: false,
        },
        markov: tokio::sync::Mutex::new(markov::Markov::new(depth, &name)),
    };

    let brain = std::sync::Arc::new(brain);
    save(Arc::clone(&brain)).await?;

    let mut config = Config::default();
    config.brains.insert(name.clone(), brain.config.clone());

    let data = format!(
        "\n# generated from <user_input>\n# depth: {}\n{}",
        depth,
        toml::to_string_pretty(&config).unwrap()
    );

    // TODO handle this better
    // notably the errors, reject any changes
    tokio::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&topics.config_path)
        .await
        .expect("must be able to open brain.toml")
        .write_all(data.as_bytes())
        .await
        .expect("must be able to append to brain.toml");

    topics.brains.lock().await.insert(name.clone(), brain);

    okay(models::responses::Created { name, brain_file })
}

pub async fn save(db: BrainDb) -> Result<impl Reply> {
    let name = &db.config.brain_file;

    // TODO pass in options for determining if we should rotate
    // if the file exists, try rotating it
    if tokio::fs::metadata(&name).await.is_ok() {
        if let Err(err) = rotate(&name).await {
            return error(Error::CannotRotate {
                file: name.to_string_lossy().to_string(),
                reason: err.to_string(),
            });
        }
    }

    let markov = db.markov.lock().await.clone();
    let (tx, rx) = tokio::sync::oneshot::channel();
    let file_name = name.clone();

    tokio::task::spawn_blocking(move || {
        let now = std::time::Instant::now();
        let res = markov::save(&markov, file_name).map(|_| now.elapsed());
        let _ = tx.send(res);
    });

    // unwrap is for the channel, not the value
    match rx.await.unwrap() {
        Ok(time) => okay(models::responses::Saved {
            name: name.to_string_lossy().to_string(),
            time,
        }),
        Err(err) => error(Error::CannotSave {
            file: name.to_string_lossy().to_string(),
            reason: err.to_string(),
        }),
    }
}

pub async fn list(topics: Arc<Topics>) -> Result<impl Reply> {
    let mut brains = hashbrown::HashMap::new();

    for (k, v) in &*topics.brains.lock().await {
        brains.insert(
            k.clone(),
            models::responses::ListItem {
                name: k.clone(),
                brain_file: v.config.brain_file.clone().into(),
                read_only: v.config.read_only,
            },
        );
    }
    okay(models::responses::List {
        brains,
        config_path: topics.config_path.clone().into(),
    })
}
