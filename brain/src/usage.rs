pub const USAGE_SHORT: &str = r##"
usage:
    train a new brain from a file, using the defaults:
        brain train --input foo.txt

    load the brain.toml config and starts the http api
        brain --port 9000

subcommands:
    train

flags:
    -h,--help
    -v,--version

required:
    -i,--input <filename>

optional:
    -o,--output <filename> [default: input file stem]
    -n,--name <string> [default: input file stem]
    -d,--depth <number> [default: 3]
    -p,--port <number> [default: 9000]
"##;

pub const USAGE_LONG: &str = r##"
usage:
    train a new brain from a file, using the defaults:
        brain train --input foo.txt

    load the brain.toml config and starts the http api
        brain --port 9000

subcommands:
    train

flags:
    -h,--help
        display this message
    -v,--version
        display the version

required:
    -i,--input <filename>
        the input file to train from

optional:
    -o,--output <filename> [default: input file stem]
        output file to save to, e.g. foo.db 
        (.db will be appended if its not provided)

    -n,--name <string> [default: input file stem]
        the name of the database

    -d,--depth <number> [default: 5]
        the training depth

    -p,--port <number> [default: 9000]
        port to listen on
"##;
