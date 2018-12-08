use sysinfo::{self, ProcessExt, SystemExt};

use crate::markov::Markov;
use crate::util::*;

#[derive(Debug, Serialize)]
pub struct Stats {
    pub chain_keys: usize,
    pub link_sets: usize,
    pub entry_points: usize,
    pub process_memory: usize,
    pub brain_file_size: usize,
    pub timings: Option<Vec<u64>>,
}

impl Stats {
    pub fn new(m: &Markov) -> Self {
        let file_size = get_file_size(m.brain_file).expect("get file size");

        let pid = sysinfo::get_current_pid();
        let mut system = sysinfo::System::new();
        system.refresh_process(pid);
        let process = system.get_process(pid).expect("get process size");

        Stats {
            chain_keys: m.chain.len(),
            link_sets: m.chain.iter().map(|(_, v)| v.len()).sum(),
            entry_points: m.entry_points.len(),
            process_memory: process.memory() as usize,
            brain_file_size: file_size as usize,
            timings: None,
        }
    }

    pub fn new_with_timing(m: &Markov, timings: Vec<u64>) -> Self {
        let mut this = Stats::new(m);
        this.timings = Some(timings);
        this
    }
}
