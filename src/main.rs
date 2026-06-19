use std::time::Duration;

mod database;
mod models;
mod routines;
mod services;

use sqlx::{Pool, Postgres};
use std::env::var;
use tokio::time::interval;

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
    let delay_time: u64 = var("DELAY_TIME")
        .expect("DELAY_TIME mut be set")
        .parse::<u64>()
        .unwrap();

    println!("Delay time: {} minutes \n", delay_time);

    // Start pings
    let ping_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(delay_time * 60));
        loop {
            tick.tick().await;
            services::heartbeat::heartbeat(
                AppState {
                    client_db: ping_pool.clone(),
                },
                delay_time,
            )
            .await;
        }
    });

    // Wait for 15 seconds
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Start calc latency
    let c_latency_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(delay_time * 60));
        loop {
            tick.tick().await;
            services::calc_latency::calc_latency(AppState {
                client_db: c_latency_pool.clone(),
            })
            .await;
        }
    });

    // Wait for 15 seconds
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Start make status
    let c_status_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(delay_time * 60));
        loop {
            tick.tick().await;
            services::make_status::make_status(AppState {
                client_db: c_status_pool.clone(),
            })
            .await;
        }
    });

    // Start clear old data
    let time_to_clear_old_data: u64 = var("TIME_TO_CLEAR_OLD_DATA")
        .expect("TIME_TO_CLEAR_OLD_DATA must be set")
        .parse::<u64>()
        .unwrap();

    // Variable
    let clear_old_data_on_start: bool = var("CLEAR_OLD_DATA_ON_START")
        .expect("CLEAR_OLD_DATA_ON_START must be set")
        .parse::<bool>()
        .unwrap();

    // Check if clear old data on start is true and wait
    if !clear_old_data_on_start {
        tokio::time::sleep(Duration::from_secs(time_to_clear_old_data * 3600)).await;
    };

    // Wait for 15 seconds
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Start clear old data
    let c_old_data_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(time_to_clear_old_data * 3600));
        loop {
            tick.tick().await;
            routines::clear_old_data::clear_old_data(AppState {
                client_db: c_old_data_pool.clone(),
            })
            .await;
        }
    });

    // Exit
    tokio::signal::ctrl_c().await?;
    Ok(())
}
