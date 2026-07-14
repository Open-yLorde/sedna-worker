use reqwest::Client;

use crate::models::commit::Commit;

pub async fn get_latest_commit(
    owner: &str,
    repo: &str,
    branch: &str,
    token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let commits: Vec<Commit> = client
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/commits?sha={branch}&per_page=1"
        ))
        .header("User-Agent", "Rust-App")
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;

    Ok(commits[0].sha.clone())
}
