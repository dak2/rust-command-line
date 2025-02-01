use clap::{App, Arg};
use std::{
  error::Error,
  fs::File,
  io::{self, BufRead, BufReader},
};
use std::collections::HashMap;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  in_file: String,
  out_file: Option<String>,
  count: bool,
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("uniqr")
    .version("0.1.0")
    .author("dak2")
    .about("Rust uniq")
    .arg(
        Arg::with_name("count")
            .short("c")
            .long("count")
            .help("Show counts")
            .takes_value(false)
    )
    .arg(
        Arg::with_name("in_file")
            .value_name("IN_FILE")
            .help("Input file")
            .default_value("-")
    )
    .arg(
        Arg::with_name("out_file")
            .value_name("OUT_FILE")
            .help("Output file")
    )
    .get_matches();

    let count = matches.is_present("count");
    let in_file = matches.value_of("in_file").unwrap().to_string();
    let out_file = matches.value_of("out_file").map(String::from);

    Ok(Config {
        in_file,
        out_file,
        count
    })
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
      "-" => Ok(Box::new(BufReader::new(io::stdin()))),
      _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

pub fn run(config: Config) -> MyResult<()> {
  let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
  let mut line = String::new();

  loop {
    let bytes = file.read_line(&mut line)?;
    if bytes == 0 {
      break;
    }
    let trimmed_line = line.trim().to_string();
    *frequencies.entry(trimmed_line).or_insert(0) += 1;
    line.clear();
  }

  for (string, count) in frequencies.iter() {
    if config.count {
      println!("{:4} {}", count, string);
    } else {
      println!("{}", string);
    }
  }

  Ok(())
}
