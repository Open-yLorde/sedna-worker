use crate::AppState;

pub async fn clear_old_data(db: AppState) {
    println!("CLEAR OLD DATA:");

    // Delete all heartbeats data
    sqlx::query("DELETE FROM heartbeat")
        .execute(&db.client_db)
        .await
        .unwrap();

    // Delete all latency data
    sqlx::query("DELETE FROM latency")
        .execute(&db.client_db)
        .await
        .unwrap();

    let keep_last: i64 = std::env::var("SIZE_OF_STATUS_DATA_AFTER_CLEAR")
        .expect("SIZE_OF_STATUS_DATA_AFTER_CLEAR must be set")
        .parse()
        .unwrap();

    let size_of_status: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM status")
        .fetch_one(&db.client_db)
        .await
        .unwrap_or(0);

    if size_of_status > keep_last {
        sqlx::query(
            r#"
            DELETE FROM status
            WHERE id NOT IN (
                SELECT id
                FROM (
                    SELECT id
                    FROM status
                    ORDER BY id DESC
                    LIMIT $1
                ) AS latest
            )
            "#,
        )
        .bind(keep_last)
        .execute(&db.client_db)
        .await
        .unwrap();
    }

    println!("Done!");
}
