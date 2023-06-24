use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use camino::Utf8PathBuf;
use clap::Parser;
use serde::Deserialize;
use serde_json;

#[derive(serde::Serialize)]
struct JsonOutput {
    tags_count: Vec<(usize, String)>,
    // urls: Vec<String>
}

#[derive(clap::Subcommand)]
enum Command {
    TagsPopularity{
        #[clap(short, default_value_t = 10)]
        tags_count: usize,

        #[clap(short)]
        is_reversed: bool
    },
    Size,
    SearchByTag{
        tag: String
    },
    SearchAnimated,
    SearchByScore{
        score: i32
    }
}

// #TODO
#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Specify path for json file to parse.
    /// File must contain derpibooru response to the query: /api/v1/json/search/images.
    ///
    /// For more information visit: https://derpibooru.org/pages/api
    path: Utf8PathBuf,
}

#[derive(Debug, Deserialize)]
struct Response {
    images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
struct Image {
    tags: Vec<String>,
    size: usize,
    view_url: String,
    animated: bool,
    score: i32
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Vec<String>,
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let derpi_data = serde_json::from_str::<Response>(&fs::read_to_string(args.path)?)?;

    match args.command {
        Command::TagsPopularity { tags_count: limit, is_reversed } => {
            sort_by_popularity(derpi_data, limit, is_reversed)?;
        },
        Command::Size => {
            calculate_size(derpi_data);
        },
        Command::SearchByTag { tag } => {
            search_by_tag(derpi_data, tag)?;
        },
        Command::SearchAnimated => {
            search_animated(derpi_data)?;
        },
        Command::SearchByScore { score } => {
            search_by_score(derpi_data, score)?
        },
    }

    Ok(())
}

fn search_by_score(derpi_data: Response, score: i32) -> Result<(), Box<dyn Error>> {
    for image in derpi_data.images {
        if image.score >= score {
            println!("{}", image.view_url);
            return Ok(())
        }
    }
    Err("No such score found".to_owned().into())
}

fn search_animated(derpi_data: Response) -> Result<(), Box<dyn Error>> {
    for image in derpi_data.images {
        if image.animated {
            println!("{}", image.view_url);
            return Ok(());
        }
    }
    Err("No animated images found".to_owned().into())
}

fn search_by_tag(derpi_data: Response, target: String) -> Result<(), Box<dyn Error>> {
    for image in derpi_data.images {
        for tag in image.tags {
            if tag == target {
                println!("{}", image.view_url);
                return Ok(())
            }
        }
    }
    Err("Tag not found".to_owned().into())
}

fn calculate_size(derpi_data: Response) {
    let mut size: usize = 0;
    for image in &derpi_data.images {
        size += image.size / 1024;
    }

    println!("{size}");
}

fn sort_by_popularity(derpi_data: Response, limit: usize, is_reversed: bool) -> Result<(), Box<dyn Error>> {
    let mut tags_frequency = BTreeMap::<String, usize>::new();
    for image in &derpi_data.images {
        for tag in &image.tags {
            match tags_frequency.entry(tag.clone()) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
                Entry::Occupied(occupied_entry) => {
                    *occupied_entry.into_mut() += 1;
                }
            }
        }
    }
    let mut tags_count: Vec<(usize, String)> = Vec::new();
    for (tag, count) in tags_frequency {
        tags_count.push((count, tag));
    }
    tags_count.sort_unstable();
    if !is_reversed {
        tags_count.reverse();
    }
    let mut ranged_tags_count: Vec<(usize, String)> = Vec::new();
    let mut count = 0;
    for elem in tags_count {
        if count >= limit {
            break;
        }
        ranged_tags_count.push(elem);
        count += 1;
    }
    let json = serde_json::to_string_pretty(&JsonOutput {
        tags_count: ranged_tags_count,
    })?;
    println!("{}", &json);
    Ok(())
}

fn main() -> ExitCode {
    match try_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            println!("{error:#?}");
            return ExitCode::FAILURE;
        }
    }
}
