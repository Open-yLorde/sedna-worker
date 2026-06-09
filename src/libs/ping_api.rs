use std::time::Instant;

use crate::{AppState, models::ping::PingModel};

pub async fn ping_api(db: AppState, time: u64) {
    println!("Pinging API...");
    let api_url = std::env::var("API_URL").unwrap_or("http://localhost:8080/api".to_string());
    let request_start = Instant::now();

    let result = reqwest::get(format!("{}/system/ping", api_url))
        .await
        .unwrap();

    let request_duration: i32 = request_start.elapsed().as_millis() as i32;

    println!("Timeout: {} minutes", time);
    println!("Status: {}", result.status());
    println!("Duration: {}ms", request_duration);
    println!("");

    sqlx::query_as::<_, PingModel>(
        "INSERT INTO ping (endpoint, delay, timeout, success, status_code) VALUES ($1, $2, $3, $4, $5) RETURNING *",
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
