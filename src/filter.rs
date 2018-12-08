use std::fs::{self, OpenOptions};
use std::io::prelude::*;

use regex::Regex;

use crate::util::*;

// TODO load these from an external source
lazy_static! {
    static ref URL: Regex = Regex::new(r"https?://").expect("to parse regex");
    static ref NOYE: Regex = Regex::new(r"<(?i)(:?noye_*)>\t(.*?)$").expect("to parse regex");
    static ref MSG: Regex = Regex::new(
        r"(:?\w{3}\s\d{2}\s\d{2}:\d{2}:\d{2}\s)?<(?i).+?>(:?\s|\t)(.*?)$"
    ).expect("to parse regex");
}

pub fn filter(inputs: &[String], output: &str) {
    let _ = fs::remove_file(output);

    for input in inputs {
        let exists = fs::metadata(&input);
        if exists.is_err() || !exists.unwrap().is_file() {
            eprintln!("file {} doesn't exist", input);
            continue;
        }

        let data = {
            timeit!("reading {}", input);
            eprintln!(
                "size: {} KB",
                get_file_size(&input).expect("get input file size")
            );
            fs::read_to_string(input).expect("read input")
        };
        let lines = {
            timeit!("filtering {}", input);
            let mut total = 0;
            let lines = data
                .split_terminator(|c| ".?!\n".contains(c))
                .inspect(|_s| total += 1)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .filter(|s| s.len() > 60)
                .filter(|s| !NOYE.is_match(s))
                .filter(|s| !URL.is_match(s))
                .filter(|s| MSG.is_match(s))
                .map(|s| {
                    let c = MSG.captures(s).unwrap();
                    c.get(c.len() - 1).expect("to get message data").as_str()
                })
                .filter(|s| s.len() > 60)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            eprintln!("got {} lines from {}", lines.len(), total);
            lines
        };
        {
            timeit!("writing {}", output);
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(output)
                .expect("to open output");

            for line in lines {
                f.write_all(line.as_bytes()).expect("to write line");
                f.write_all(b"\n").expect("to write newline");
            }
            f.flush().unwrap();
            f.sync_all().unwrap();

            eprintln!("size: {} KB", get_file_size(&output).unwrap());
        }
    }
}
