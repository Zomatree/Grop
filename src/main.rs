use std::env;
use std::fs;
use grop::args;
use grop::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: args::Config = args::collect_args(&args);
    println!("Searching for {} In file {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Cant read that file");

    println!("{:?}", search::search_string(contents, &config.query));

}
