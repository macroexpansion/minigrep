use std::env;
use std::process;

use ndqc_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ndqc_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
