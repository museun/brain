use bincode;

use markov::Markov;
use stats::Stats;

use util::*;

pub fn load<'a>(input: &str, buf: &'a [u8]) -> Markov<'a> {
    let markov: Markov = {
        timeit!("loading {}", input);

        let markov = bincode::deserialize(&buf).expect("deserialize file");
        let stats = Stats::new(&markov);
        eprintln!("contexts: {} ", stats.chain_keys.comma_separate());
        eprintln!("links: {}", stats.link_sets.comma_separate());
        eprintln!("entry points: {}", stats.entry_points.comma_separate());
        eprintln!("mem used: {} KB", stats.process_memory.comma_separate());

        markov
    };

    markov
}
