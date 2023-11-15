use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // FIXME tracing subscriber
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();

    mare_website::run().await
}
