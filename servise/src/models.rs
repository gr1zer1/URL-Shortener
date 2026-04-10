use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow,Serialize,Deserialize)]
pub struct LinkModel {
    pub id: i32,
    pub code: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

