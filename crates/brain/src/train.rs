use crate::config::{BrainConfig, Config};
use crate::stats::Stats;
use markov::Markov;

use hashbrown::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{fs::File, io::BufReader};
use {futures::prelude::*, tokio::prelude::*};

use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

const PROGRESS_MAX: usize = 100;

struct Arguments {
    depth: Option<usize>,
    input: PathBuf,
    output: PathBuf,
    name: String,
}

pub async fn train(args: pico_args::Arguments) -> anyhow::Result<()> {
    let Arguments {
        depth,
        input,
        output,
        name,
    } = parse_args(args)?;

    log::debug!(target: "brain", "counting lines");
    let count = line_count(&input).await?;
    // TODO Humanize this
    log::info!(target: "brain", "training {} lines", count);

    let (mut stats, samples) = Stats::new(count / PROGRESS_MAX);
    let sync = display_progress_bar(samples);

    let markov = train_brain(&name, depth, &input, &mut stats).await?;
    let report = stats.done();

    // wait for the progress bar task to end
    sync.await.unwrap().finish_and_clear();
    log::info!(target: "brain",
        "total lines: {}, took: {:.2?}, {:.3} lines per second",
        report.count,
        report.duration,
        report.lines_per_sec()
    );

    let now = Instant::now();
    markov::save(&markov, &output)?;
    log::debug!(target: "brain", "saving took: {:.2?}", now.elapsed());

    let size = file_size_as_kib(&output).await?;
    log::info!(target: "brain", "{} file size: {:.2} KiB", output.display(), size);

    print_append_message(&markov.name, output, depth, input);

    // fast drop
    std::mem::forget(markov);

    Ok(())
}

fn parse_args(mut args: pico_args::Arguments) -> anyhow::Result<Arguments> {
    let depth: Option<usize> = args.opt_value_from_str(["-d", "--depth"])?;
    let input: PathBuf = args.value_from_str(["-i", "--input"])?;

    if !input.is_file() {
        anyhow::bail!("a file must be provided")
    }

    let (mut output, name): (String, String) = match (
        args.value_from_str(["-o", "--output"]),
        args.value_from_str(["-n", "--name"]),
    ) {
        (Ok(output), Ok(name)) => (output, name),
        (Ok(output), Err(..)) => (output.trim_end_matches(".db").to_string(), output),
        (Err(..), Ok(name)) => (format!("{}.db", name), name),
        (Err(..), Err(..)) => {
            let file = input
                .file_stem()
                .ok_or_else(|| anyhow::anyhow!("invalid input file name"))?
                .to_string_lossy()
                .to_string();
            (file.clone(), file)
        }
    };

    if !output.ends_with(".db") {
        output.push_str(".db");
    }

    args.finish()?;

    let arguments = Arguments {
        depth,
        input,
        name,
        output: output.into(),
    };
    Ok(arguments)
}

fn display_progress_bar(
    samples: Receiver<crate::stats::Sample>,
) -> tokio::task::JoinHandle<ProgressBar> {
    let pb = ProgressBar::new(PROGRESS_MAX as _);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {msg} ({eta})")
            .progress_chars("#>-"),
    );

    let bar = pb.clone();
    tokio::task::spawn(async move {
        while !bar.is_finished() {
            tokio::time::delay_for(Duration::from_millis(250)).await;
            bar.tick();
        }
    });

    tokio::task::spawn(async move {
        for sample in samples {
            pb.set_message(&format!("{:.3}/s", sample.lines_per_sec()));
            pb.inc(1);
        }
        pb
    })
}

fn print_append_message(
    name: impl ToString,
    brain_file: impl Into<PathBuf>,
    depth: Option<usize>,
    input: impl AsRef<Path>,
) {
    let name = name.to_string();
    let brain_file = brain_file.into();

    let config = BrainConfig {
        name: name.clone(),
        brain_file,
        read_only: true,
    };

    let toml = toml::to_string_pretty(&Config {
        brains: {
            let mut map = HashMap::new();
            map.insert(name, config);
            map
        },
    })
    .unwrap();

    log::info!(target: "brain", "add this to brain.toml");
    println!();
    println!("# generated from {}", input.as_ref().display());
    println!("# depth: {}", depth.unwrap_or(5));
    println!("{}", toml);
    log::info!(target: "brain", "end of config");
}

async fn train_brain(
    name: &str,
    depth: impl Into<Option<usize>>,
    input: impl AsRef<Path>,
    stats: &mut Stats,
) -> anyhow::Result<markov::Markov> {
    let mut lines = BufReader::new(File::open(input).await?).lines();
    let mut markov = Markov::new(depth.into().unwrap_or(5), name);
    while let Some(Ok(line)) = lines.next().await {
        stats.tick();
        markov.train_text(&line);
    }
    Ok(markov)
}

async fn file_size_as_kib(file: impl AsRef<Path>) -> anyhow::Result<f64> {
    let size = File::open(file).await?.metadata().await?.len();
    Ok(size as f64 / 1024.0)
}

async fn line_count(input: impl AsRef<Path>) -> anyhow::Result<usize> {
    let count = BufReader::new(File::open(input).await?)
        .lines()
        .fold(0_usize, |a, _| async move { a + 1 })
        .await;
    Ok(count)
}
