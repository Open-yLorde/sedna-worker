use sqlx::{Pool, Postgres};

pub async fn local_connect() -> Pool<Postgres> {
    let database_url = std::env::var("LOCAL_DATABASE_URL")
        .unwrap_or("postgres://admin:password123@localhost:5437/db_admin".to_string());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .unwrap();

    let check_migrate = sqlx::migrate!("./src/database/postgres_connection/local_migrations")
        .run(&pool)
        .await;

    match check_migrate {
        Ok(_) => println!("Migrated successfully"),
        Err(e) => println!("Error applying migrations: {:?}", e),
    }

    pool
}
