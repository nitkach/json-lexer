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

#[derive(clap::ValueEnum, Clone)]
enum ScoreMode {
    Max,
    Min,
}

#[derive(clap::Subcommand)]
enum Command {
    TagsPopularity {
        #[clap(long, default_value_t = 10)]
        tags_count: usize,

        #[clap(long)]
        reversed: bool,
    },
    Size,
    Score {
        #[arg(value_enum)]
        mode: Option<ScoreMode>,
    },
    SearchByTag {
        tag: String,
    },
    SearchAnimated,
    SearchByScore {
        score: i32,
    },
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
    size: i32,
    view_url: String,
    animated: bool,
    score: i32,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Vec<String>,
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let derpi_data = match serde_json::from_str::<Response>(&match fs::read_to_string(args.path) {
        Ok(it) => it,
        Err(err) => return Err(Box::new(err)),
    }) {
        Ok(it) => it,
        Err(err) => return Err(err.into()),
    };

    match args.command {
        Command::TagsPopularity {
            tags_count: limit,
            reversed: is_reversed,
        } => {
            sort_by_popularity(derpi_data, limit, is_reversed)?;
        }
        Command::Size => {
            // 64536189
            // println!("{}", sum(derpi_data, get_size));
            // 64536189
            println!("{}", fold(derpi_data, 0, size_addition));
        }
        Command::Score { mode } => {
            // <MODE>  [possible values: max, min]
            // [MODE]  [possible values: max, min]
            match mode {
                Some(ScoreMode::Max) => {
                    println!("{}", comparison(derpi_data, max, get_score)?);
                }
                Some(ScoreMode::Min) => {
                    println!("{}", comparison(derpi_data, min, get_score)?);
                }
                None => {
                    println!("{}", fold(derpi_data, 0, score_addition))
                }
            }
        }
        Command::SearchByTag { tag } => {
            search_by_tag(derpi_data, tag)?;
        }
        Command::SearchAnimated => {
            linear_search(derpi_data, get_animated)?;
        }
        Command::SearchByScore { score: _ } => {
            linear_search(derpi_data, find_score)?;
        }
    }
    Ok(())
}

// fn get_size(image: &Image) -> i32 {
//     image.size
// }

fn get_score(image: &Image) -> i32 {
    image.score
}

fn get_animated(image: &Image) -> bool {
    image.animated
}

fn find_score(image: &Image) -> bool {
    image.score >= 30
}

fn max(a: i32, b: i32) -> bool {
    a > b
}

fn min(a: i32, b: i32) -> bool {
    a < b
}

// unite fn max and fn min by using callback fn cmp?
fn comparison(
    derpi_data: Response,
    compare: fn(i32, i32) -> bool,
    get_field: fn(&Image) -> i32,
) -> Result<i32, Box<dyn Error>> {
    let mut extremum: Option<i32> = None;
    for image in derpi_data.images {
        // match max: Some(max).cmp(get_field(...)) | None: max = Some(get_field(...))
        let maybe_extremum = get_field(&image);

        // match extremum {
        //         Some(x) => x,
        //         None => f(),
        //     }
        match extremum {
            Some(value) => {
                if compare(maybe_extremum, value) {
                    extremum = Some(maybe_extremum);
                }
            }
            None => {
                extremum = Some(maybe_extremum);
            }
        }
    }
    extremum.ok_or_else(|| "Empty json".to_owned().into())
    // match extremum {
    //     Some(max) => Ok(max),
    //     None => return Err("Empty json".to_owned().into()),
    // }
}

// impl Searchable for Foo
// struct Foo

// #[derive(Searchable)]
// struct Bar

trait Searchable {
    fn condition(image: &Image) -> bool;
}

fn linear_search(
    derpi_data: Response,
    condition: fn(&Image) -> bool,
) -> Result<(), Box<dyn Error>> {
    for image in derpi_data.images {
        if condition(&image) {
            println!("{}", image.view_url);
            return Ok(());
        }
    }
    Err("Matching image not found".to_owned().into())
}

fn search_by_tag(derpi_data: Response, target: String) -> Result<(), Box<dyn Error>> {
    for image in derpi_data.images {
        for tag in image.tags {
            if tag == target {
                println!("{}", image.view_url);
                return Ok(());
            }
        }
    }
    Err("Tag not found".to_owned().into())
}

fn size_addition(acc: i32, image: Image) -> i32 {
    acc + image.size
}

// fn size_product(acc: i32, image: Image) -> i32 {
//     acc * image.size
// }

fn score_addition(acc: i32, image: Image) -> i32 {
    acc + image.score
}

// fn score_product(acc: i32, image: Image) -> i32 {
//     acc * image.score
// }

fn fold_tags(mut tags_frequency: BTreeMap<String, usize>, image: Image) -> BTreeMap<String, usize> {
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
    tags_frequency
}

fn fold<T>(derpi_data: Response, initial: T, operation: fn(acc: T, image: Image) -> T) -> T {
    let mut acc = initial;
    for image in derpi_data.images {
        acc = operation(acc, image);
    }
    acc
}

fn sort_by_popularity(
    derpi_data: Response,
    limit: usize,
    reversed: bool,
) -> Result<(), Box<dyn Error>> {
    // let tags_frequency: BTreeMap<String, usize> =
    //     derpi_data
    //         .images
    //         .into_iter()
    //         .fold(BTreeMap::<String, usize>::new(), |mut acc, image| {
    //             // for tag in &image.tags {
    //             //     match acc.entry(tag.clone()) {
    //             //         Entry::Vacant(vacant_entry) => {
    //             //             vacant_entry.insert(1);
    //             //         }
    //             //         Entry::Occupied(occupied_entry) => {
    //             //             *occupied_entry.into_mut() += 1;
    //             //         }
    //             //     }
    //             // };
    //             // acc
    //             let _ = image
    //                 .tags
    //                 .into_iter()
    //                 .map(|tag| {
    //                     match acc.entry(tag.clone()) {
    //                         Entry::Vacant(vacant_entry) => {
    //                             vacant_entry.insert(1);
    //                         }
    //                         Entry::Occupied(occupied_entry) => {
    //                             *occupied_entry.into_mut() += 1;
    //                         }
    //                     };
    //                     tag
    //                 })
    //                 .collect::<Vec<String>>();
    //             acc
    //         });

    let mut tags_frequency = BTreeMap::<String, usize>::new();
    tags_frequency = fold(derpi_data, tags_frequency, fold_tags);

    // let mut tags_count: Vec<(usize, String)> = tags_frequency
    //     .into_iter()
    //     .map(|(tag, count)| (count, tag))
    //     .collect();

    let mut tags_count: Vec<(usize, String)> = Vec::new();
    for (tag, count) in tags_frequency {
        tags_count.push((count, tag));
    }

    tags_count.sort_unstable();
    if !reversed {
        tags_count.reverse();
    }

    // let ranged_tags_count: Vec<(usize, String)> = tags_count.into_iter().take(limit).collect();

    let mut count = 0;
    let mut ranged_tags_count = Vec::new();
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
