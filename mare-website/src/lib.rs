mod app;
mod database;

pub async fn run() -> anyhow::Result<()> {
    if let Err(err) = dotenvy::dotenv() {
        return Err(anyhow::Error::new(err));
    }

    app::run().await
}
