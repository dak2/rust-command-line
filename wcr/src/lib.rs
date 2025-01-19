use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  println!("{:#?}", config);
  Ok(())
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("wcr")
    .version("0.1.0")
    .author("dak2")
    .about("Rust wc")
    .arg(
      Arg::with_name("files")
        .value_name("FILES")
        .help("Input file(s) [default: -]")
        .multiple(true)
        .default_value("-"),
    )
    .arg(
      Arg::with_name("lines")
        .short("l")
        .help("Show line count")
        .takes_value(false)
    )
    .arg(
      Arg::with_name("words")
        .short("w")
        .help("Show word count")
        .takes_value(false)
    )
    .arg(
      Arg::with_name("chars")
        .short("m")
        .help("Show character count")
        .takes_value(false)
        .conflicts_with("bytes"),
    )
    .arg(
      Arg::with_name("bytes")
        .short("c")
        .help("Show byte count")
        .takes_value(false)
        .conflicts_with("chars"),
    )
    .get_matches();

  let files = matches.values_of_lossy("files").unwrap();
  let lines = matches.is_present("lines");
  let words = matches.is_present("words");
  let bytes = matches.is_present("bytes");
  let chars = matches.is_present("chars");

  gen_config(files, lines, words, bytes, chars)
}

fn gen_config(
  files: Vec<String>,
  mut lines: bool,
  mut words: bool,
  mut bytes: bool,
  chars: bool,
) -> MyResult<Config> {

  if [lines, words, bytes, chars].iter().all(|&x| !x) {
    lines = true;
    words = true;
    bytes = true;
  }

  Ok(Config {
      files,
      lines,
      words,
      bytes,
      chars,
  })
}

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  lines: bool,
  words: bool,
  bytes: bool,
  chars: bool,
}
