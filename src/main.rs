use std::time::Duration;

mod ping_api;

use ping_api::ping_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Sedna Worker...");

    dotenv::dotenv().ok();
    env_logger::init();

    let mut tick = tokio::time::interval(Duration::from_secs(20 * 60));

    loop {
        tick.tick().await;
        ping_api().await;
    }
}
