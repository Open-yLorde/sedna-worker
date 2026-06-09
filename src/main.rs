use std::time::Duration;

mod database;
mod libs;
mod models;

use sqlx::{Pool, Postgres};
use tokio::time::interval;

use crate::libs::{calc_latency::calc_latency, ping_api::ping_api};

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
async fn main() -> anyhow::Result<()> {
    println!("Starting Sedna Worker...");

    dotenv::dotenv().ok();
    env_logger::init();

    let pool = database::postgres_connection::local_connect().await;

    // Get env vars
    let time_to_ping = std::env::var("TIME_TO_PING")
        .expect("TIME_TO_PING mut be set")
        .parse::<u64>()
        .unwrap();

    let time_to_calc_latency = std::env::var("TIME_TO_CALC_LATENCY")
        .expect("TIME_TO_CALC_LATENCY mut be set")
        .parse::<u64>()
        .unwrap();

    // Start pings
    let ping_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(time_to_ping * 60));
        loop {
            tick.tick().await;
            ping_api(AppState::new(ping_pool.clone()), time_to_ping).await;
        }
    });

    // Wait for 30 seconds
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Start calc latency
    let c_latency_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(time_to_calc_latency * 60 + 15));
        loop {
            tick.tick().await;
            calc_latency(AppState::new(c_latency_pool.clone())).await;
        }
    });

    // Exit
    tokio::signal::ctrl_c().await?;
    Ok(())
}
