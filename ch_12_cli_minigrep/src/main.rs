use std::{env, process};
use ch_12_cli_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];
    let config = Config::new(env::args()).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // let contents = fs::read_to_string(config.filename).expect("Should have been able to read the file.");

    // println!("With text:\n {contents}");
    // run(config);
    if let Err(e) = ch_12_cli_minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
