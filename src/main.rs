use std::time::Duration;

mod database;
mod executors;
mod models;
mod routines;
mod services;

use sqlx::{Pool, Postgres};
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

    dotenvy::dotenv().ok();
    env_logger::init();

    let pool = database::postgres_connection::local_connect::local_connect().await;
    database::postgres_connection::prod_connect::prod_connect().await;

    dotenvy::dotenv().expect("The .env file could not be found!");

    // Get env vars
    let delay_time: u64 = std::env::var("DELAY_TIME")
        .expect("DELAY_TIME mut be set")
        .parse::<u64>()
        .unwrap();

    let save_data_on_database: bool = std::env::var("SAVE_DATA_ON_DATABASE")
        .expect("SAVE_DATA_ON_DATABASE must be set")
        .parse::<bool>()
        .unwrap_or(true);

    let clear_old_data: bool = std::env::var("CLEAR_OLD_DATA")
        .expect("CLEAR_OLD_DATA must be set")
        .parse::<bool>()
        .unwrap_or(true);

    let check_for_deploys = std::env::var("CHECK_FOR_DEPLOYS")
        .expect("CHECK_FOR_DEPLOYS must be set")
        .parse::<bool>()
        .unwrap_or(true);

    let time_for_check_new_deploy: u64 = std::env::var("TIME_TO_CHECK_FOR_NEW_DEPLOY")
        .expect("TIME_TO_CHECK_FOR_NEW_DEPLOY must be set")
        .parse::<u64>()
        .unwrap();

    println!("Delay time: {} minutes", delay_time);
    println!("Save data: {}", save_data_on_database);
    println!("Clear old data: {}", clear_old_data);
    println!("Check for deploys: {}", check_for_deploys);
    println!(
        "Time to check for new deploy: {} minutes",
        time_for_check_new_deploy
    );
    println!("");

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

    // Check for deploys
    if check_for_deploys {
        // Wait for 10 seconds
        tokio::time::sleep(Duration::from_secs(10)).await;

        // Start check for deploys
        let check_for_deploys_pool = pool.clone();
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(time_for_check_new_deploy * 60));
            loop {
                tick.tick().await;
                routines::check_for_deploys::check_for_deploys(AppState {
                    client_db: check_for_deploys_pool.clone(),
                })
                .await;
            }
        });
    }

    if save_data_on_database {
        // Wait for 10 seconds
        tokio::time::sleep(Duration::from_secs(10)).await;

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
    }

    if save_data_on_database {
        // Wait for 10 seconds
        tokio::time::sleep(Duration::from_secs(10)).await;

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
    }

    // Start clear old data
    let time_to_clear_old_data: u64 = std::env::var("TIME_TO_CLEAR_OLD_DATA")
        .expect("TIME_TO_CLEAR_OLD_DATA must be set")
        .parse::<u64>()
        .unwrap();

    // Variable
    let clear_old_data_on_start: bool = std::env::var("CLEAR_OLD_DATA_ON_START")
        .expect("CLEAR_OLD_DATA_ON_START must be set")
        .parse::<bool>()
        .unwrap_or(true);

    if clear_old_data {
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
    }

    // Exit
    tokio::signal::ctrl_c().await?;
    Ok(())
}
