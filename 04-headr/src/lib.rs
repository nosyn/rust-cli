use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    bytes: Option<usize>,
    lines: usize,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("v0.1.1")
        .author("Son Nguyen")
        .about("rusty head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Concatenate FILE(s), or standard input, to standard output.")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .conflicts_with("lines")
                .takes_value(true)
                .help("Print bytes of each of the specified files."),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("Print count lines of each of the specified files.")
                .default_value("10"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?
        .unwrap();

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let file_length = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        if file_length > 1 {
            println!(
                "{}==> {} <==",
                if file_num > 0 { "\n" } else { "" },
                filename
            );
        }

        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if let Some(bytes) = config.bytes {
                    let mut handle = file.take(bytes as u64);
                    let mut buffer = vec![0; bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
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

    // dbg!(config);
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

// --------------------------------------------------
#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
