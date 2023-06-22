use std::{path::PathBuf, fs::File};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Vec<String>,
}

fn main() {
    let derpi_path = PathBuf::from("..\\derpibooru_response.json");
    File::open(derpi_path);
}
