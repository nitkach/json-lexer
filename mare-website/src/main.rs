use anyhow::Result;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::Targets, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // FIXME tracing subscriber
    // let targets = Targets::new().with_target("sqlx::query", LevelFilter::OFF);
    // let filter = EnvFilter::from_default_env()
    //     // Filter events for the specific target
    //     .add_directive("sqlx::query".parse()?);

    // tracing_subscriber::registry()
    //     .with(fmt)
    //     .with(filter)
    //     .init();

    // ------------------------
    // let filter = EnvFilter::builder()
    //     .with_default_directive(LevelFilter::INFO.into())
    //     .with_env_var("MAREWEB_LOG")
    //     .from_env_lossy();

    // // Initialize the logger with the specified configuration
    // tracing_subscriber::registry()
    //     .with(filter)
    //     .init();

    let filter = EnvFilter::from_default_env();

    // Initialize the logger with the specified configuration
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(filter)
        .init();

    mare_website::run().await
}
