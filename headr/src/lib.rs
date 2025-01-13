use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  let num_files = config.files.len();

  for(file_num, filename) in config.files.iter().enumerate() {
    match open(filename) {
      Err(err) => eprintln!("{}: {}", filename, err),
      Ok(mut file) => {
        if num_files > 1 {
          println!(
            "{}==> {} <==",
            if file_num > 0 { "\n" } else { "" },
            filename
          );
        }

        if let Some(num_bytes) = config.bytes {
          let mut handle = file.take(num_bytes as u64);
          let mut buffer = vec![0; num_bytes];
          let bytes_read = handle.read(&mut buffer)?;
          print!(
            "{}",
            String::from_utf8_lossy(&buffer[..bytes_read])
          )
        } else {
          let mut line = String::new();
          for _ in 0..config.lines {
            let bytes = file.read_line(&mut line)?;
            if bytes == 0 {
              break;
            }
            print!("{}", line);
            line.clear();
          }
        }
      }
    }
  }

  Ok(())
}

pub fn read_lines<T: BufRead>(reader: T) -> MyResult<Vec<String>> {
  let mut lines = Vec::new();
  for line in reader.lines() {
      lines.push(line?);
  }
  Ok(lines)
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
      "-" => Ok(Box::new(BufReader::new(io::stdin()))),
      _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
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
              .takes_value(true)
              .validator(|v| {
                v.parse::<usize>()
                  .map(|_| ())
                  .map_err(|_| {
                    let message = format!(
                      "error: invalid value '{v}' for \
                      '--lines <LINES>': invalid digit found in string"
                    );
                    message
                  })
              }),
      )
      .arg(
          Arg::with_name("bytes")
              .short("c")
              .long("bytes")
              .help("Number of bytes")
              .takes_value(true)
              .conflicts_with("lines")
              .validator(|v| {
                v.parse::<usize>()
                  .map(|_| ())
                  .map_err(|_| {
                    let message = format!(
                      "error: invalid value '{v}' for \
                      '--bytes <BYTES>': invalid digit found in string"
                    );
                    message
                  })
              }),
      )
      .get_matches();

    Ok(Config {
      files: matches.values_of_lossy("files").unwrap(),
      lines: matches.value_of("lines")
      .map(|v| v.parse::<usize>().unwrap()).unwrap_or(10),
      bytes: matches.value_of("bytes")
      .map(|v| v.parse::<usize>().unwrap()),
    })
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}
