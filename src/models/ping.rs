use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PingModel {
    pub id: i32,
    pub endpoint: String,
    pub delay: i32,
    pub timeout: i32,
    pub success: bool,
    pub status_code: i32,
    pub created_at: Option<DateTime<Utc>>,
}
