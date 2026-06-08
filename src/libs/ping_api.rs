use std::time::Instant;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct PingModel {
    id: i32,
    endpoint: String,
    delay: i32,
    success: bool,
    created_at: Option<DateTime<Utc>>,
}

pub async fn ping_api(db: AppState, time: u64) -> bool {
    println!("Pinging API...");
    let api_url = std::env::var("API_URL").unwrap_or("http://localhost:8080/api".to_string());
    let request_start = Instant::now();

    let result = reqwest::get(format!("{}/system/ping", api_url))
        .await
        .unwrap();

    let request_duration: i32 = request_start.elapsed().as_millis() as i32;

    println!("Timeout: {} minutes", time * 60);
    println!("Status: {}", result.status());
    println!("Duration: {}ms", request_duration);
    println!("");

    sqlx::query_as::<_, PingModel>(
        "INSERT INTO ping (endpoint, delay, success) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind("/api/system/ping")
    .bind(&request_duration)
    .bind(result.status().is_success())
    .fetch_one(&db.client_db)
    .await
    .unwrap();

    result.status().is_success()
}
