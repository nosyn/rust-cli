use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("v0.1.1")
        .author("Son Nguyen")
        .about("rusty unique (uniq)")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("IN_FILE")
                .help("Input file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .takes_value(false)
                .help("show counts"),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    match open(&config.in_file) {
        Err(err) => eprintln!("{}: {}", config.in_file, err),
        Ok(file) => {}
    }

    dbg!(config);
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
