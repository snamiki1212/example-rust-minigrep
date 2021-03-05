
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }
      Ok(Config { query: args[1].clone(), filename: args[2].clone()})
  }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
  println!("query is {:?}, filename is {:?}", config.query, config.filename);

  let mut f = File::open(config.filename)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  println!("TEXT:\n{}", contents);
  Ok(())
}

