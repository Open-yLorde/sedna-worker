use crate::{AppState, executors};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct LastSha {
    sha: String,
}

pub async fn check_for_deploys(db: AppState) {
    let check_for_deploys: bool = std::env::var("CHECK_FOR_DEPLOYS")
        .expect("CHECK_FOR_DEPLOYS must be set")
        .parse::<bool>()
        .unwrap_or(true);

    let use_discord_webhooks: bool = std::env::var("USE_DISCORD_WEBHOOKS")
        .expect("USE_DISCORD_WEBHOOKS must be set")
        .parse::<bool>()
        .unwrap_or(true);
    let discord_webhook_url = std::env::var("DISCORD_WEBHOOK_URL")
        .expect("DISCORD_WEBHOOK_URL must be set, if disable type none");

    if !check_for_deploys {
        return;
    }

    println!("\nDEPLOYS:");

    let last_sha =
        match sqlx::query_as::<_, LastSha>("SELECT sha FROM gb_commits ORDER BY id DESC LIMIT 1;")
            .fetch_optional(&db.client_db)
            .await
        {
            Ok(Some(res)) => res,
            Ok(None) => LastSha {
                sha: "".to_string(),
            },
            Err(err) => {
                println!("Error getting last commit: {}", err);
                return;
            }
        };

    let owner: String = std::env::var("APP_GITHUB_OWNER").expect("APP_GITHUB_OWNER must be set");
    let repo: String = std::env::var("APP_GITHUB_REPO").expect("APP_GITHUB_REPO must be set");
    let branch: String = std::env::var("APP_GITHUB_BRANCH").expect("APP_GITHUB_BRANCH must be set");
    let token: String = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

    let sha = match executors::get_latest_commit::get_latest_commit(
        &owner.to_string(),
        &repo.to_string(),
        &branch.to_string(),
        &token.to_string(),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => {
            println!("Error getting latest commit: {}", err);
            return;
        }
    };

    if sha != last_sha.sha {
        println!("New commit found: {}", sha);

        if use_discord_webhooks {
            match executors::send_discord_webhook::send_discord_webhook(
                &discord_webhook_url,
                &"Deploy",
                &format!("New commit found: {}", sha),
            )
            .await
            {
                Ok(_) => (),
                Err(err) => println!("Error sending discord webhook: {}", err),
            }
        }

        executors::make_deploy::make_deploy().await;
    } else {
        println!("No new commit found");
        if use_discord_webhooks {
            match executors::send_discord_webhook::send_discord_webhook(
                &discord_webhook_url,
                &"Deploy",
                &format!("No new commit found"),
            )
            .await
            {
                Ok(_) => (),
                Err(err) => println!("Error sending discord webhook: {}", err),
            }
        }
    }

    match sqlx::query("INSERT INTO gb_commits (sha) VALUES ($1) ON CONFLICT (sha) DO NOTHING;")
        .bind(&sha)
        .execute(&db.client_db)
        .await
    {
        Ok(_) => (),
        Err(e) => println!("Error inserting commit: {}", e),
    }

    print!("");
}
