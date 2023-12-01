mod app;
mod database;

pub async fn run() -> anyhow::Result<()> {
    println!("Enter main");

    if let Err(err) = dotenvy::dotenv() {
        println!("Failed to load .env file: {}", err);
    }
    app::run().await
}
