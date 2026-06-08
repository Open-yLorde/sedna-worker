use rand::{RngExt, rng};
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

    let random_time: u64 = rng().random_range(10..=25);

    if std::env::var("WAIT_FOR_FIRST_PING").unwrap_or("no".to_string()) == "yes"
        || std::env::var("WAIT_FOR_FIRST_PING").unwrap_or("no".to_string()) == "true"
    {
        println!("Waiting for ping...");
        tokio::time::sleep(Duration::from_secs(random_time * 60)).await;
        unsafe { std::env::remove_var("WAIT_FOR_FIRST_PING") };
    }

    let mut tick = tokio::time::interval(Duration::from_secs(random_time * 60));
    loop {
        tick.tick().await;
        ping_api(AppState::new(pool.clone()), random_time).await;
    }
}
