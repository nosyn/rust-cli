use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("v0.1.1")
        .author("Son Nguyen")
        .about("rusty cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Concatenate FILE(s), or standard input, to standard output.")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n equivalent to -vE")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines")
                .conflicts_with("number_nonblank")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut count = 1;
                for line_result in file.lines() {
                    match line_result {
                        Err(err) => eprintln!("Failed to read line: {}", err),
                        Ok(line) => {
                            if (config.number_nonblank_lines && !line.is_empty())
                                || config.number_lines
                            {
                                println!("{:>6}\t{}", count, line);
                                count += 1;
                            } else {
                                println!("{}", line);
                            }
                        }
                    }
                }
            }
        }
    }

    // dbg!(config);
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
