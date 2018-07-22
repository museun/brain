use std::fs;

use bincode;

use markov::Markov;
use stats::Stats;
use util::*;

pub fn train(input: &str, output: &str, depth: usize) {
    let data = {
        timeit!("reading {}", input);
        let size = get_file_size(&input).unwrap();
        eprintln!("size: {} KB", size.comma_separate());
        fs::read_to_string(input).expect("read input")
    };

    // the brain is the output file
    let mut markov = Markov::with_depth(depth, &output);
    {
        timeit!("training");
        eprintln!("training with depth: {}", depth);
        markov.train_text(&data);
    }

    {
        timeit!("writing {}", output);
        let data = bincode::serialize(&markov).unwrap();
        fs::write(output, data).unwrap();
        let size = get_file_size(&output).unwrap();
        eprintln!("size: {} KB", size.comma_separate());
    }

    let stats = Stats::new(&markov);
    eprintln!("contexts: {} ", stats.chain_keys.comma_separate());
    eprintln!("links: {}", stats.link_sets.comma_separate());
    eprintln!("entry points: {}", stats.entry_points.comma_separate());
    eprintln!("mem used: {} KB", stats.process_memory.comma_separate());
}
