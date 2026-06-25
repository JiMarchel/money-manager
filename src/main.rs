use dotenvy::dotenv;
use money_manager::bootstrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    bootstrap::app::run().await
}
