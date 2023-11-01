use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    let Err(err) = cli_2::run() else {
        return ExitCode::SUCCESS;
    };

    eprintln!("Application error: {err}");
    ExitCode::FAILURE
}
