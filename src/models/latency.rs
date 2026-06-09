use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LatencyModel {
    pub id: i32,
    pub delay: i32,
    pub created_at: Option<DateTime<Utc>>,
}
