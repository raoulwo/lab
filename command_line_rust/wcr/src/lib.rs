use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(name = crate_name!())]
#[command(author = crate_authors!())]
#[command(version = crate_version!())]
#[command(about = crate_description!(), long_about = None)]
pub struct Cli {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(short, long, help = "Show line count")]
    lines: bool,

    #[arg(short, long, help = "Show word count")]
    words: bool,

    #[arg(short = 'c', long, help = "Show byte count", conflicts_with = "chars")]
    bytes: bool,

    #[arg(short = 'm', long, help = "Show character count")]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn parse() -> Cli {
    let mut cli = Cli::parse();
    if !cli.lines && !cli.words && !cli.bytes && !cli.chars {
        cli.lines = true;
        cli.words = true;
        cli.bytes = true;
    }

    cli
}

pub fn run(cli: Cli) -> Result<()> {
    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };

    for filename in &cli.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    let formatted_file_info = format_file_info(&info, filename, &cli);
                    println!("{}", formatted_file_info);

                    total.num_lines += info.num_lines;
                    total.num_words += info.num_words;
                    total.num_bytes += info.num_bytes;
                    total.num_chars += info.num_chars;
                }
            }
        }
    }
    if cli.files.len() > 1 {
        let formatted_total_file_info = format_file_info(&total, "total", &cli);
        println!("{}", formatted_total_file_info);
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    loop {
        let bytes_read = file.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += bytes_read;
        num_chars += line.chars().count();

        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_file_info(file_info: &FileInfo, filename: &str, cli: &Cli) -> String {
    format!(
        "{}{}{}{}{}",
        format_field(file_info.num_lines, cli.lines),
        format_field(file_info.num_words, cli.words),
        format_field(file_info.num_bytes, cli.bytes),
        format_field(file_info.num_chars, cli.chars),
        if filename == "-" {
            "".to_string()
        } else {
            format!(" {}", filename)
        }
    )
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::format_field;

    use super::{count, FileInfo};
    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(expected, info.unwrap());
    }

    #[test]
    fn test_format_field() {
        assert_eq!("", format_field(1, false));
        assert_eq!("       3", format_field(3, true));
        assert_eq!("      10", format_field(10, true));
    }
}
