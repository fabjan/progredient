use std::process;

use progredient::{self, config};

fn main() {
    let cfg = match config::from_argv() {
        progredient::config::ParseResult::ShowHelp => {
            println!("{}", config::usage());
            process::exit(0);
        }
        progredient::config::ParseResult::ErrorUnknownArgument(arg) => {
            println!("Unknown argument: {arg}");
            process::exit(64);
        }
        progredient::config::ParseResult::Ok(cfg) => cfg,
    };

    let output = progredient::render(&cfg);

    println!("{output}");
}
