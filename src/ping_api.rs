pub async fn ping_api() -> bool {
    println!("Pinging API...");

    let api_url = std::env::var("API_URL").unwrap_or("http://localhost:8080/api".to_string());

    let result = reqwest::get(format!("{}/system/ping", api_url))
        .await
        .unwrap();

    println!("Status: {}", result.status());

    result.status().is_success()
}
