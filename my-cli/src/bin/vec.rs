use std::process::ExitCode;
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[clap(long)]
    first: f64,

    #[clap(long, short)]
    second: f64
}

fn main() -> ExitCode {
    // dbg!(std::env::current_dir().unwrap());


    let args = Args::parse();

    // let args = std::env::args().collect::<Vec<String>>();
    // let json = match serde_json::from_str::<Args>(&args[1]) {
    //     Ok(json) => json,
    //     Err(error) => {
    //         eprintln!("{error}");
    //         return ExitCode::FAILURE;
    //     },
    // };

    // ---------------

    // let serde_json::Value::Object(map) = &json else {
    //     eprintln!("Error: Expected JSON Object");
    //     return ExitCode::FAILURE;
    // };

    // let Some(first) = map.get("-f") else {
    //     eprintln!("Error: Expected a key-value");
    //     return ExitCode::FAILURE;
    // };

    // let Some(first) = first.as_f64() else {
    //     eprintln!("Error: Expected JSON Number");
    //     return ExitCode::FAILURE;
    // };

    // let Some(second) = map.get("-s") else {
    //     eprintln!("Error: Expected a key-value");
    //     return ExitCode::FAILURE;
    // };

    // let Some(second) = second.as_f64() else {
    //     eprintln!("Error: Expected JSON Number");
    //     return ExitCode::FAILURE;
    // };

    println!("{} + {} = {}", args.first, args.second, args.first + args.second);

    ExitCode::SUCCESS
}
