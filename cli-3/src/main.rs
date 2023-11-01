use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    run_main().await
}

async fn run_main() -> ExitCode {
    if let Err(err) = dotenvy::dotenv() {
        println!("{err}");
        return ExitCode::FAILURE;
    }

    if let Err(err) = cli_3::run().await {
        println!("{err}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}
