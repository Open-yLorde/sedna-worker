use std::{
    io::{Write, stdout},
    time::Instant,
};

use crate::{AppState, executors, models::heartbeat::HeartbeatModel};
use curl::easy::Easy;

pub async fn heartbeat(db: AppState, time: u64) {
    let api_url = std::env::var("API_URL").expect("API_URL must be set");
    let use_discord_webhooks: bool = std::env::var("USE_DISCORD_WEBHOOKS")
        .expect("USE_DISCORD_WEBHOOKS must be set")
        .parse::<bool>()
        .unwrap_or(true);
    let discord_webhook_url = std::env::var("DISCORD_WEBHOOK_URL")
        .expect("DISCORD_WEBHOOK_URL must be set, if disable type none");
    let save_data_on_database: bool = std::env::var("SAVE_DATA_ON_DATABASE")
        .expect("SAVE_DATA_ON_DATABASE must be set")
        .parse::<bool>()
        .unwrap_or(true);

    println!("\nHEARTBEAT:");
    let request_start = Instant::now();

    // HTTP request to google to check connection
    let mut easy = Easy::new();
    easy.url("https://google.com").unwrap();
    easy.perform().unwrap();

    match easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }) {
        Ok(_) => {
            let result = reqwest::get(format!("{}/system/ping", api_url))
                .await
                .unwrap();

            let request_duration: i32 = request_start.elapsed().as_millis() as i32;

            println!("Status: {}", result.status());
            println!("Duration: {}ms", request_duration);

            if use_discord_webhooks {
                match executors::send_discord_webhook::send_discord_webhook(
                    &discord_webhook_url,
                    "Heartbeat",
                    &format!("Heartbeat Duration: {}ms", request_duration),
                )
                .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Error sending discord webhook: {}", err);
                    }
                }
            }

            if !save_data_on_database {
                return;
            }

            sqlx::query_as::<_, HeartbeatModel>(
                r#"
                INSERT INTO heartbeat (endpoint, delay, timeout, success, status_code)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            )
            .bind("/api/system/ping")
            .bind(&request_duration)
            .bind(time as i32)
            .bind(result.status().is_success())
            .bind(result.status().as_u16() as i32)
            .fetch_one(&db.client_db)
            .await
            .unwrap();
        }
        Err(_) => {
            println!("No connection found, saving wildcard data...");
            if !save_data_on_database {
                return;
            }
            sqlx::query_as::<_, HeartbeatModel>(
                r#"
                INSERT INTO heartbeat (endpoint, delay, timeout, success, status_code)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            )
            .bind("/api/system/ping")
            .bind(1000)
            .bind(time as i32)
            .bind(true)
            .bind(200)
            .fetch_one(&db.client_db)
            .await
            .unwrap();
        }
    }

    println!("");
}
