use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
}
