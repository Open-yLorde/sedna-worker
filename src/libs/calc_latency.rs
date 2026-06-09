use crate::{
    AppState,
    models::{latency::LatencyModel, ping::PingModel},
};

pub async fn calc_latency(db: AppState) {
    println!("Calculating latency...");

    let ping_result =
        match sqlx::query_as::<_, PingModel>("SELECT * FROM ping ORDER BY id DESC LIMIT 10")
            .fetch_all(&db.client_db)
            .await
        {
            Ok(result) => result,
            Err(e) => panic!("{}", e),
        };

    let medial: i32 = ping_result.iter().fold(0, |acc, x| acc + x.delay) / ping_result.len() as i32;
    println!("Average latency: {}ms", medial);

    sqlx::query_as::<_, LatencyModel>("INSERT INTO latency (delay) VALUES ($1) RETURNING *")
        .bind(medial)
        .fetch_one(&db.client_db)
        .await
        .unwrap();

    println!("");
}
