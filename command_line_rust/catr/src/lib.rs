use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
        long = "number",
        help = "Number lines",
        group = "line_numbers"
    )]
    number_lines: bool,

    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number non-blank lines",
        group = "line_numbers"
    )]
    number_nonblank_lines: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}

pub fn run(cli: Cli) -> Result<()> {
    for filename in cli.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_index = 1;
                for line in file.lines() {
                    let line = line?;

                    if cli.number_lines {
                        println!("{:>6}\t{}", line_index, line);
                        line_index += 1;
                    } else if cli.number_nonblank_lines {
                        if !line.is_empty() {
                            println!("{:>6}\t{}", line_index, line);
                            line_index += 1;
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
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
