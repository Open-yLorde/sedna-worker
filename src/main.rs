use std::time::Duration;

mod database;
mod libs;

use sqlx::{Pool, Postgres};

use crate::libs::ping_api::ping_api;

#[derive(Clone)]
pub struct AppState {
    pub client_db: Pool<Postgres>,
}

impl AppState {
    pub fn new(client_db: Pool<Postgres>) -> AppState {
        AppState { client_db }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Sedna Worker...");

    dotenv::dotenv().ok();
    env_logger::init();

    let pool = database::postgres_connection::local_connect().await;
    let time_to_ping = std::env::var("TIME_TO_PING")
        .expect("TIME_TO_PING mut be set")
        .parse::<u64>()
        .unwrap();

    let mut tick = tokio::time::interval(Duration::from_secs(time_to_ping * 60));
    loop {
        tick.tick().await;
        ping_api(AppState::new(pool.clone()), time_to_ping).await;
    }
}
