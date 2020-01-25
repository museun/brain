pub enum Command {
    Train(pico_args::Arguments),
    Load(pico_args::Arguments),
}

pub fn parse_args() -> anyhow::Result<Command> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains("-h") {
        print_help_and_quit(Quit::ShowShortHelp, Status::Okay);
    }

    if args.contains("--help") {
        print_help_and_quit(Quit::ShowFullHelp, Status::Okay);
    }

    if args.contains(["-v", "--version"]) {
        print_help_and_quit(Quit::ShowVersion, Status::Okay);
    }

    match args.subcommand()?.as_ref().map(|s| s.as_str()) {
        Some("train") => Ok(Command::Train(args)),
        Some(..) => print_help_and_quit(Quit::ShowShortHelp, Status::Error(1)),
        None => Ok(Command::Load(args)),
    }
}

// TODO redo this
enum Quit {
    ShowVersion,
    ShowShortHelp,
    ShowFullHelp,
}

enum Status {
    Okay,
    Error(i32),
}

fn print_help_and_quit(quit: Quit, status: Status) -> ! {
    let (name, version) = (env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    match quit {
        Quit::ShowVersion => println!("{} v{}", name, version),
        Quit::ShowShortHelp => {
            println!("{} v{}", name, version);
            println!("{}", crate::usage::USAGE_SHORT)
        }
        Quit::ShowFullHelp => {
            println!("{} v{}", name, version);
            println!("{}", crate::usage::USAGE_LONG)
        }
    }

    match status {
        Status::Okay => std::process::exit(0),
        Status::Error(code) => (std::process::exit(code)),
    }
}
