use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("haedr")
      .version("0.1.0")
      .author("dak2")
      .about("Rust head")
      .arg(
          Arg::with_name("files")
              .value_name("FILE")
              .help("Input File(s)")
              .multiple(true)
              .default_value("-"),
      )
      .arg(
          Arg::with_name("lines")
              .short("n")
              .long("lines")
              .help("Number of file lines")
              .takes_value(false)
      )
      .arg(
          Arg::with_name("bytes")
              .short("c")
              .long("bytes")
              .help("Number of bytes")
              .takes_value(false)
              .conflicts_with("lines")
      )
      .get_matches();

    let lines = matches.value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- : {}", e))?;

    let bytes = matches.value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- : {}", e))?;

    Ok(Config {
      files: matches.values_of_lossy("files").unwrap(),
      lines: lines.unwrap(),
      bytes,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
  match val.parse::<usize>() {
    Ok(n) if n > 0 => Ok(n),
    _ => Err(From::from(val)),
  }
}

#[test]
fn test_parse_positive_int() {
  let res = parse_positive_int("3");
  assert!(res.is_ok());
  assert_eq!(res.unwrap(), 3);

  let res = parse_positive_int("foo");
  assert!(res.is_err());
  assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

  let res = parse_positive_int("0");
  assert!(res.is_err());
  assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}
