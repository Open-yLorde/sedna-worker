use crate::{
    AppState,
    models::{latency::LatencyModel, status::StatusModel},
};

pub async fn make_status(db: AppState) {
    println!("STATUS:");

    let total_pings: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM heartbeat")
        .fetch_one(&db.client_db)
        .await
        .unwrap_or(1);

    let total_pings_success: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM heartbeat WHERE success = true")
            .fetch_one(&db.client_db)
            .await
            .unwrap_or(1);

    let total_pings_fail: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM heartbeat WHERE success = false")
            .fetch_one(&db.client_db)
            .await
            .unwrap_or(1);

    let total_pings_success_percent: f64 =
        (total_pings_success as f64) / (total_pings - total_pings_fail) as f64;

    let uptime = total_pings_success_percent * 100.0;

    let latency =
        sqlx::query_as::<_, LatencyModel>("SELECT * FROM latency ORDER BY id DESC LIMIT 1")
            .fetch_one(&db.client_db)
            .await;

    let latency_ms = latency.unwrap().delay;

    println!("Total pings: {}", total_pings);
    println!("Total pings success: {}", total_pings_success);
    println!("Total pings fail: {}", total_pings_fail);

    println!("");
    println!("Uptime: {}%", uptime);
    println!("AVG latency: {}ms", latency_ms);

    println!("-- == -- [END] -- == --\n");

    sqlx::query_as::<_, StatusModel>(
        "INSERT INTO status (uptime, latency) VALUES ($1, $2) RETURNING *",
    )
    .bind(&uptime)
    .bind(&latency_ms)
    .fetch_one(&db.client_db)
    .await
    .unwrap();
}
