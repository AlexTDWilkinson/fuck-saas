pub mod channel;
use serde::Deserialize;
use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub created_at: i64, // Make nullable fields optional
}
