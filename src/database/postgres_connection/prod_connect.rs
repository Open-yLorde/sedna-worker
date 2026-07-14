use std::time::Duration;

pub async fn prod_connect() {
    let prod_database_url =
        std::env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set");

    let migrate_on_run: bool = std::env::var("MIGRATE_ON_RUN")
        .expect("MIGRATE_ON_RUN must be set")
        .parse::<bool>()
        .unwrap_or(true);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .idle_timeout(Duration::from_secs(5 * 60))
        .acquire_timeout(Duration::from_secs(30))
        .connect(&prod_database_url)
        .await
        .unwrap();

    if migrate_on_run {
        let check_migrate = sqlx::migrate!("./src/database/postgres_connection/prod_migrations")
            .run(&pool)
            .await;

        match check_migrate {
            Ok(_) => println!("[PROD] Migrated successfully\n"),
            Err(e) => {
                println!("[PROD] Error applying migrations: {:?} \n", e);
                std::process::exit(1);
            }
        }
    }

    match pool.begin().await {
        Ok(_) => println!("Successfully connected to production postgres\n"),
        Err(e) => {
            println!("Error connecting to production postgres: {:?} \n", e);
        }
    }
}
