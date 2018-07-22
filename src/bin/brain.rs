use std::env;
use std::fs;
use std::io::{BufReader, Read};
use std::process;

extern crate getopts;
use getopts::Options;

extern crate bincode;

#[macro_use]
extern crate noye_brain;
use noye_brain::*;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} -f|-t|-l", program);
    eprintln!("{}", opts.usage(&brief));
    process::exit(1);
}

fn err_and_usage(program: &str, msg: &str, opts: &Options) {
    eprintln!("error: {}", msg);
    print_usage(program, opts);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints the usage");
    opts.optflag("t", "train", "enable training mode");
    opts.optflag("l", "load", "enable loading mode");

    opts.optflag("f", "filter", "enables filter mode");
    opts.optmulti("i", "input", "an input file", "INPUT");
    opts.optopt("o", "output", "the output file", "OUTPUT");
    opts.optopt("d", "depth", "n-gram size", "DEPTH");
    opts.optopt("s", "save", "save duration in minutes", "SAVE");

    opts.optopt("a", "address", "the address to bind to", "ADDRESS");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    if matches.opt_present("f") {
        let inputs = matches.opt_strs("i");
        if inputs.is_empty() {
            err_and_usage(&program, "input file must be specified", &opts)
        }

        let output = matches.opt_str("o");
        if output.is_none() {
            err_and_usage(&program, "output file must be specified", &opts)
        }

        filter(&inputs, &output.unwrap());
        return;
    } else if matches.opt_present("t") {
        let input = matches.opt_str("i");
        if input.is_none() {
            err_and_usage(&program, "input file must be specified", &opts);
        }
        let input = input.unwrap();

        let output = matches.opt_str("o");
        let output = if output.is_none() {
            eprintln!("!! assuming you want brain.db");
            "brain.db".into()
        } else {
            output.unwrap()
        };

        let depth = if matches.opt_present("d") {
            let d = matches.opt_str("d").unwrap();
            d.parse().unwrap()
        } else {
            5
        };

        train(&input, &output, depth);
        return;
    } else if matches.opt_present("l") {
        let input = matches.opt_str("i");
        let input = if input.is_none() {
            eprintln!("!! assuming you want brain.db");
            "brain.db".into()
        } else {
            input.unwrap()
        };

        let file = {
            timeit!("reading {}", input);
            let size = get_file_size(&input).unwrap();
            eprintln!("size: {} KB", size.comma_separate());
            fs::File::open(&input).unwrap()
        };

        let mut buf = Vec::with_capacity(file.metadata().unwrap().len() as usize);
        let mut reader = BufReader::new(file);
        let _ = reader.read_to_end(&mut buf);

        let markov = load(&input, &buf);

        let address = if matches.opt_present("a") {
            matches.opt_str("a").unwrap()
        } else {
            "localhost:7878".into()
        };

        let mut server = Server::new(&address, &markov);
        server.start();
    } else {
        print_usage(&program, &opts)
    }
}
