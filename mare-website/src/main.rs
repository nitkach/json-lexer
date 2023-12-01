use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::from_default_env().add_directive("sqlx::query=off".parse()?);

    // Initialize the logger with the specified configuration
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(filter)
        .init();

    mare_website::run().await
}
