use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use anyhow::{anyhow, Result};
use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};

#[cfg(test)]
use pretty_assertions::assert_eq;

#[derive(Debug, Parser)]
#[command(name = crate_name!())]
#[command(author = crate_authors!())]
#[command(version = crate_version!())]
#[command(about = crate_description!(), long_about = None)]
pub struct Cli {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long,
        help = "Number of lines",
        value_parser = parse_lines,
        default_value = "10",
    )]
    lines: usize,

    #[arg(
        short = 'c',
        long,
        help = "Number of bytes",
        value_parser = parse_bytes,
        conflicts_with = "lines",
    )]
    bytes: Option<usize>,
}

pub fn parse() -> Cli {
    Cli::parse()
}

fn parse_lines(val: &str) -> Result<usize> {
    parse_positive_int(val).map_err(|err| anyhow!(format!("illegal line count -- {}", err)))
}

fn parse_bytes(val: &str) -> Result<usize> {
    parse_positive_int(val).map_err(|err| anyhow!(format!("illegal byte count -- {}", err)))
}

fn parse_positive_int(val: &str) -> Result<usize> {
    match val.parse() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err(anyhow!(val.to_string())),
    }
}

pub fn run(cli: Cli) -> Result<()> {
    let num_files = cli.files.len();

    for (file_num, filename) in cli.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = cli.bytes {
                    let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    let mut line = String::new();
                    for _ in 0..cli.lines {
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

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_lines() {
    let res = parse_lines("1");
    assert!(res.is_ok());
    assert_eq!(1, res.unwrap());

    let res = parse_lines("foo");
    assert!(res.is_err());
    assert_eq!(
        "illegal line count -- foo".to_string(),
        res.unwrap_err().to_string()
    );

    let res = parse_lines("0");
    assert!(res.is_err());
    assert_eq!(
        "illegal line count -- 0".to_string(),
        res.unwrap_err().to_string()
    );
}

#[test]
fn test_parse_bytes() {
    let res = parse_bytes("1");
    assert!(res.is_ok());
    assert_eq!(1, res.unwrap());

    let res = parse_bytes("foo");
    assert!(res.is_err());
    assert_eq!(
        "illegal byte count -- foo".to_string(),
        res.unwrap_err().to_string()
    );

    let res = parse_bytes("0");
    assert!(res.is_err());
    assert_eq!(
        "illegal byte count -- 0".to_string(),
        res.unwrap_err().to_string()
    );
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("1");
    assert!(res.is_ok());
    assert_eq!(1, res.unwrap());

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!("foo".to_string(), res.unwrap_err().to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!("0".to_string(), res.unwrap_err().to_string());
}
