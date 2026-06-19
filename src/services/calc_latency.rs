use crate::{
    AppState,
    models::{heartbeat::HeartbeatModel, latency::LatencyModel},
};

pub async fn calc_latency(db: AppState) {
    println!("CALC LATENCY:");

    let size_of_heartbeats_to_calc_latency: i32 =
        std::env::var("SIZE_OF_HEARTBEATS_TO_CALC_LATENCY")
            .expect("SIZE_OF_HEARTBEATS_TO_CALC_LATENCY must be set")
            .parse::<i32>()
            .unwrap();

    let ping_result = match sqlx::query_as::<_, HeartbeatModel>(
        "SELECT * FROM heartbeat ORDER BY id DESC LIMIT $1",
    )
    .bind(&size_of_heartbeats_to_calc_latency)
    .fetch_all(&db.client_db)
    .await
    {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    };

    let medial: i32 = ping_result.iter().fold(0, |acc, x| acc + x.delay) / ping_result.len() as i32;
    println!("AVG latency: {}ms", medial);
    println!("");

    sqlx::query_as::<_, LatencyModel>("INSERT INTO latency (delay) VALUES ($1) RETURNING *")
        .bind(medial)
        .fetch_one(&db.client_db)
        .await
        .unwrap();
}
