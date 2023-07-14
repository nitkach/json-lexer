use std::process::ExitCode;

fn main() -> ExitCode {
    let Err(err) = cli_2::run() else {
        return ExitCode::SUCCESS;
    };

    eprintln!("Application error: {err}");
    ExitCode::FAILURE
}
