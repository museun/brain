use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Sample {
    pub duration: Duration,
    pub count: usize,
}

impl std::fmt::Debug for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "at {:.2?}. {}: {:.3}/s",
            self.duration,
            self.count,
            self.count as f64 / self.duration.as_secs_f64()
        )
    }
}

impl Sample {
    pub fn lines_per_sec(&self) -> f64 {
        self.count as f64 / self.duration.as_secs_f64()
    }
}

#[derive(Debug)]
pub struct Stats {
    start: Instant,
    sampling: AtomicBool,
    count: AtomicUsize,
    sample_rate: usize,
    points: Sender<Sample>,
}

impl Stats {
    pub fn new(sample_rate: usize) -> (Self, Receiver<Sample>) {
        let (tx, rx) = channel();
        let this = Self {
            start: Instant::now(),
            sampling: AtomicBool::new(true),
            count: AtomicUsize::new(0),
            sample_rate,
            points: tx,
        };
        (this, rx)
    }

    pub fn tick(&self) {
        let count = self.count.fetch_add(1, Ordering::SeqCst);
        if self.sampling.load(Ordering::SeqCst)
            && count >= self.sample_rate
            && count % self.sample_rate == 0
        {
            let duration = self.start.elapsed();
            if self.points.send(Sample { duration, count }).is_err() {
                tracing::warn!("sampling disabled");
                self.sampling.store(false, Ordering::SeqCst)
            }
        }
    }

    pub fn done(self) -> Sample {
        drop(self.points);
        let duration = self.start.elapsed();
        let count = self.count.into_inner();
        Sample { duration, count }
    }
}
