use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};

#[derive(Debug, Parser)]
#[command(name = crate_name!())]
#[command(author = crate_authors!())]
#[command(version = crate_version!())]
#[command(about = crate_description!(), long_about = None)]
struct Cli {
    #[arg(required = true, value_name = "STRING", help = "Input string(s)")]
    strings: Vec<String>,

    #[arg(short = 'n', help = "Don't print trailing newline")]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    print!(
        "{}{}",
        cli.strings.join(" "),
        if cli.omit_newline { "" } else { "\n" }
    );
}
