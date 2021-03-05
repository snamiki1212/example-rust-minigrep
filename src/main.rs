use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config { query: args[1].clone(), filename: args[2].clone()})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arg: {:?}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

fn run (config: Config) -> Result<(), Box<dyn Error>>{
    println!("query is {:?}, filename is {:?}", config.query, config.filename);

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("TEXT:\n{}", contents);
    Ok(())
}

